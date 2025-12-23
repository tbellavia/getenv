use std::io::Write;
use std::process::Command;
use std::process::Stdio;

use crate::clipboard::clipboard::ClipboardError;

pub fn copy(value: &str) -> Result<(), ClipboardError> {
    let mut child = Command::new("pbcopy")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|err| match err.kind() {
            std::io::ErrorKind::NotFound => ClipboardError::NotFound {
                message: "failed to run pbcopy: command not found".to_string(),
            },
            std::io::ErrorKind::PermissionDenied => ClipboardError::PermissionDenied {
                message: "failed to run pbcopy: permission denied".to_string(),
            },
            _ => ClipboardError::ExecutionFailed {
                message: format!("failed to run pbcopy: {err}"),
            },
        })?;

    let mut stdin = child.stdin.take().ok_or(ClipboardError::CannotCopy {
        message: "getenv: failed to copy variable into the clipboard".to_string(),
    })?;

    stdin
        .write_all(value.as_bytes())
        .map_err(|err| ClipboardError::CannotCopy {
            message: format!("getenv: failed top copy variable into the clipboard: {err}"),
        })?;

    Ok(())
}
