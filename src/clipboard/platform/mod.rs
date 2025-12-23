use crate::clipboard::clipboard::ClipboardError;
#[cfg(target_os = "macos")]
use crate::clipboard::platform;

mod macos;

pub fn copy(value: &str) -> Result<(), ClipboardError> {
    #[cfg(target_os = "macos")]
    platform::macos::copy(value)
}
