use std::ffi::OsStr;
use std::ffi::OsString;
use std::path::Component;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

use crate::GitToolingError;

pub(crate) fn ensure_git_repository(path: &Path) -> Result<(), GitToolingError> {
    match run_git_for_stdout(
        path,
        vec![
            OsString::from("rev-parse"),
            OsString::from("--is-inside-work-tree"),
        ],
        None,
    ) {
        Ok(output) if output.trim() == "true" => Ok(()),
        Ok(_) => Err(GitToolingError::NotAGitRepository {
            path: path.to_path_buf(),
        }),
        Err(GitToolingError::GitCommand { status, .. }) if status.code() == Some(128) => {
            Err(GitToolingError::NotAGitRepository {
                path: path.to_path_buf(),
            })
        }
        Err(err) => Err(err),
    }
}

pub(crate) fn resolve_head(path: &Path) -> Result<Option<String>, GitToolingError> {
    match run_git_for_stdout(
        path,
        vec![
            OsString::from("rev-parse"),
            OsString::from("--verify"),
            OsString::from("HEAD"),
        ],
        None,
    ) {
        Ok(sha) => Ok(Some(sha)),
        Err(GitToolingError::GitCommand { status, .. }) if status.code() == Some(128) => Ok(None),
        Err(other) => Err(other),
    }
}

pub(crate) fn normalize_relative_path(path: &Path) -> Result<PathBuf, GitToolingError> {
    let mut result = PathBuf::new();
    let mut saw_component = false;
    for component in path.components() {
        saw_component = true;
        match component {
            Component::Normal(part) => result.push(part),
            Component::CurDir => {}
            Component::ParentDir => {
                if !result.pop() {
                    return Err(GitToolingError::PathEscapesRepository {
                        path: path.to_path_buf(),
                    });
                }
            }
            Component::RootDir | Component::Prefix(_) => {
                return Err(GitToolingError::NonRelativePath {
                    path: path.to_path_buf(),
                });
            }
        }
    }

    if !saw_component {
        return Err(GitToolingError::NonRelativePath {
            path: path.to_path_buf(),
        });
    }

    Ok(result)
}

pub(crate) fn resolve_repository_root(path: &Path) -> Result<PathBuf, GitToolingError> {
    let root = run_git_for_stdout(
        path,
        vec![
            OsString::from("rev-parse"),
            OsString::from("--show-toplevel"),
        ],
        None,
    )?;
    Ok(PathBuf::from(root))
}

pub(crate) fn apply_repo_prefix_to_force_include(
    prefix: Option<&Path>,
    paths: &[PathBuf],
) -> Vec<PathBuf> {
    if paths.is_empty() {
        return Vec::new();
    }

    match prefix {
        Some(prefix) => paths.iter().map(|path| prefix.join(path)).collect(),
        None => paths.to_vec(),
    }
}

/// Returns the path of `repo_path` relative to `repo_root` when `repo_path`
/// is a (non-empty) subdirectory of the repository root.
///
/// Behaviour:
/// - If `repo_root == repo_path` this function returns `None` (not considered a subdir).
/// - First it attempts `strip_prefix(repo_root)` and returns the result if it's
///   non-empty. This handles the common case where both paths are in the same
///   representation (no symlink / canonicalization needed).
/// - If that fails (for example because of symlinks or different path representations),
///   it falls back to canonicalizing both paths and tries `strip_prefix` again.
/// - If the resulting relative path is empty, `None` is returned (we avoid returning
///   an empty PathBuf when the paths are equal).
///
/// Parameters:
/// - `repo_root`: the repository root path to strip from.
/// - `repo_path`: the path to test (may be inside the repo or outside).
///
/// Returns:
/// - `Some(PathBuf)` with the relative path from `repo_root` to `repo_path` (guaranteed
///   to be non-empty), or `None` if `repo_path` equals `repo_root` or is not inside the repo.
///
/// Example:
/// ```text
/// repo_root = /project
/// repo_path = /project/sub/dir
/// returns Some(PathBuf::from("sub/dir"))
/// ```
pub(crate) fn repo_subdir(repo_root: &Path, repo_path: &Path) -> Option<PathBuf> {
    if repo_root == repo_path {
        return None;
    }

    repo_path
        .strip_prefix(repo_root)
        .ok()
        .and_then(non_empty_path)
        .or_else(|| {
            let repo_root_canon = repo_root.canonicalize().ok()?;
            let repo_path_canon = repo_path.canonicalize().ok()?;
            repo_path_canon
                .strip_prefix(&repo_root_canon)
                .ok()
                .and_then(non_empty_path)
        })
}

/// Helper that converts a `&Path` into `Some(PathBuf)` only if the path is
/// not empty. Returns `None` for an empty path. This is used to avoid returning
/// an empty relative path (which can happen when the paths are equal).
fn non_empty_path(path: &Path) -> Option<PathBuf> {
    if path.as_os_str().is_empty() {
        None
    } else {
        Some(path.to_path_buf())
    }
}

pub(crate) fn run_git_for_status<I, S>(
    dir: &Path,
    args: I,
    env: Option<&[(OsString, OsString)]>,
) -> Result<(), GitToolingError>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    run_git(dir, args, env)?;
    Ok(())
}

pub(crate) fn run_git_for_stdout<I, S>(
    dir: &Path,
    args: I,
    env: Option<&[(OsString, OsString)]>,
) -> Result<String, GitToolingError>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let run = run_git(dir, args, env)?;
    String::from_utf8(run.output.stdout)
        .map(|value| value.trim().to_string()) // 当前方法比run_git_for_stdout_all多了个trim
        .map_err(|source| GitToolingError::GitOutputUtf8 {
            command: run.command,
            source,
        })
}

/// Executes `git` and returns the full stdout without trimming so callers
/// can parse delimiter-sensitive output, propagating UTF-8 errors with context.
pub(crate) fn run_git_for_stdout_all<I, S>(
    dir: &Path,
    args: I,
    env: Option<&[(OsString, OsString)]>,
) -> Result<String, GitToolingError>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    // Keep the raw stdout untouched so callers can parse delimiter-sensitive
    // output (e.g. NUL-separated paths) without trimming artefacts.
    let run = run_git(dir, args, env)?;
    // Propagate UTF-8 conversion failures with the command context for debugging.
    String::from_utf8(run.output.stdout).map_err(|source| GitToolingError::GitOutputUtf8 {
        command: run.command,
        source,
    })
}

///
/// 调用迭代器的 size_hint()，得到一个 (lower_bound, Option<upper_bound>) 的元组：
/// lower 是已知的最小剩余元素数（保守值）。
/// upper 是一个可选的最大剩余元素数（若迭代器能报告上界则为 Some(n)，否则 None）。
/// 目的是为了在创建 Vec 时预分配合适容量，减少后续扩容。
///
fn run_git<I, S>(
    dir: &Path,
    args: I,
    env: Option<&[(OsString, OsString)]>,
) -> Result<GitRun, GitToolingError>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let iterator = args.into_iter();
    let (lower, upper) = iterator.size_hint();
    let mut args_vec = Vec::with_capacity(upper.unwrap_or(lower));
    for arg in iterator {
        args_vec.push(OsString::from(arg.as_ref()));
    }
    let command_string = build_command_string(&args_vec);
    let mut command = Command::new("git");
    command.current_dir(dir);
    if let Some(envs) = env {
        for (key, value) in envs {
            command.env(key, value);
        }
    }
    command.args(&args_vec);
    let output = command.output()?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(GitToolingError::GitCommand {
            command: command_string,
            status: output.status,
            stderr,
        });
    }
    Ok(GitRun {
        command: command_string,
        output,
    })
}

/// 这里返回只是用来展示的，不是用来执行的
fn build_command_string(args: &[OsString]) -> String {
    if args.is_empty() {
        return "git".to_string();
    }
    let joined = args
        .iter()
        .map(|arg| arg.to_string_lossy().into_owned())
        .collect::<Vec<_>>()
        .join(" ");
    format!("git {joined}")
}

struct GitRun {
    command: String,
    output: std::process::Output,
}
