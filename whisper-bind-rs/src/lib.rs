use std::ffi::CString;

#[derive(Debug)]
pub enum WhisperError {
    InitFailed,
    RecognizeFailed,
}

pub struct WhisperContext {
    ctx: *mut whisper_sys::whisper_context,
}

impl Drop for WhisperContext {
    fn drop(&mut self) {
        if !self.ctx.is_null() {
            unsafe {
                whisper_sys::whisper_free(self.ctx);
            }
        }
    }
}

impl WhisperContext {
    pub fn from_file(path: &str) -> Result<Self, WhisperError> {
        let c_path = CString::new(path).map_err(|_| WhisperError::InitFailed)?;
        let ctx = unsafe {
            whisper_sys::whisper_init_from_file_with_params(
                c_path.as_ptr(),
                whisper_sys::whisper_context_default_params(),
            )
        };
        if ctx.is_null() {
            Err(WhisperError::InitFailed)
        } else {
            Ok(WhisperContext { ctx })
        }
    }

    pub fn recognize(&mut self, params: &WhisperParams, audio: &[f32]) -> Result<(), WhisperError> {
        let result = unsafe {
            whisper_sys::whisper_full(self.ctx, params.params, audio.as_ptr(), audio.len() as i32)
        };
        if result == 0 {
            Ok(())
        } else {
            Err(WhisperError::RecognizeFailed)
        }
    }

    pub fn get_text(&self) -> Result<String, WhisperError> {
        let n_segments = unsafe { whisper_sys::whisper_full_n_segments(self.ctx) };
        let mut text = String::new();

        for i in 0..n_segments {
            let c_str = unsafe { whisper_sys::whisper_full_get_segment_text(self.ctx, i) };

            if c_str.is_null() {
                return Err(WhisperError::RecognizeFailed);
            }
            let segment = unsafe { std::ffi::CStr::from_ptr(c_str) };
            text.push_str(
                segment
                    .to_str()
                    .map_err(|_| WhisperError::RecognizeFailed)?,
            );
        }
        Ok(text)
    }
}

pub struct WhisperParams {
    params: whisper_sys::whisper_full_params,
}

impl WhisperParams {
    pub fn new() -> Self {
        let params = unsafe {
            whisper_sys::whisper_full_default_params(
                whisper_sys::whisper_sampling_strategy_WHISPER_SAMPLING_GREEDY,
            )
        };
        WhisperParams { params }
    }
}
