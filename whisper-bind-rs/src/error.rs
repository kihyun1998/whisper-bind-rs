use std::fmt;

#[derive(Debug)]
pub enum WhisperError {
    /// Model or context initialization failed
    InitFailed,
    /// Invalid input (e.g. null byte in string, invalid parameters)
    InvalidInput,
    /// General processing error (whisper_full, whisper_full_parallel, etc.)
    ProcessingFailed,
    /// Encoder execution failed
    EncodeFailed,
    /// Decoder execution failed
    DecodeFailed,
    /// UTF-8 conversion error
    Utf8Error,
    /// Null pointer returned from C API
    NullPointer,
}

impl fmt::Display for WhisperError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WhisperError::InitFailed => write!(f, "whisper initialization failed"),
            WhisperError::InvalidInput => write!(f, "invalid input"),
            WhisperError::ProcessingFailed => write!(f, "processing failed"),
            WhisperError::EncodeFailed => write!(f, "encoder failed"),
            WhisperError::DecodeFailed => write!(f, "decoder failed"),
            WhisperError::Utf8Error => write!(f, "UTF-8 conversion error"),
            WhisperError::NullPointer => write!(f, "null pointer returned"),
        }
    }
}

impl std::error::Error for WhisperError {}
