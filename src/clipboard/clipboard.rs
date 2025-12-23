use crate::clipboard::platform;

pub enum ClipboardError {
    NotFound { message: String },
    PermissionDenied { message: String },
    ExecutionFailed { message: String },
    CannotCopy { message: String },
}

impl ClipboardError {
    pub fn get_message(&self) -> String {
        match self {
            Self::NotFound { message } => message.into(),
            Self::PermissionDenied { message } => message.into(),
            Self::ExecutionFailed { message } => message.into(),
            Self::CannotCopy { message } => message.into(),
        }
    }
}

pub fn copy(value: &str) -> Result<(), ClipboardError> {
    platform::copy(value)
}
