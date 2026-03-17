use std::ffi::CString;

use crate::error::WhisperError;

/// VAD (Voice Activity Detection) parameters
#[derive(Debug, Clone, Copy)]
pub struct VadParams {
    pub(crate) raw: whisper_sys::whisper_vad_params,
}

impl VadParams {
    pub fn set_threshold(&mut self, val: f32) -> &mut Self {
        self.raw.threshold = val;
        self
    }

    pub fn set_min_speech_duration_ms(&mut self, val: i32) -> &mut Self {
        self.raw.min_speech_duration_ms = val;
        self
    }

    pub fn set_min_silence_duration_ms(&mut self, val: i32) -> &mut Self {
        self.raw.min_silence_duration_ms = val;
        self
    }

    pub fn set_max_speech_duration_s(&mut self, val: f32) -> &mut Self {
        self.raw.max_speech_duration_s = val;
        self
    }

    pub fn set_speech_pad_ms(&mut self, val: i32) -> &mut Self {
        self.raw.speech_pad_ms = val;
        self
    }

    pub fn set_samples_overlap(&mut self, val: f32) -> &mut Self {
        self.raw.samples_overlap = val;
        self
    }
}

impl Default for VadParams {
    fn default() -> Self {
        let raw = unsafe { whisper_sys::whisper_vad_default_params() };
        VadParams { raw }
    }
}

/// VAD context creation parameters
#[derive(Debug, Clone, Copy)]
pub struct VadContextParams {
    pub(crate) raw: whisper_sys::whisper_vad_context_params,
}

impl VadContextParams {
    pub fn set_n_threads(&mut self, val: i32) -> &mut Self {
        self.raw.n_threads = val;
        self
    }

    pub fn set_use_gpu(&mut self, val: bool) -> &mut Self {
        self.raw.use_gpu = val;
        self
    }

    pub fn set_gpu_device(&mut self, val: i32) -> &mut Self {
        self.raw.gpu_device = val;
        self
    }
}

impl Default for VadContextParams {
    fn default() -> Self {
        let raw = unsafe { whisper_sys::whisper_vad_default_context_params() };
        VadContextParams { raw }
    }
}

/// VAD context for speech detection
pub struct VadContext {
    vctx: *mut whisper_sys::whisper_vad_context,
}

unsafe impl Send for VadContext {}

impl Drop for VadContext {
    fn drop(&mut self) {
        if !self.vctx.is_null() {
            unsafe {
                whisper_sys::whisper_vad_free(self.vctx);
            }
        }
    }
}

impl VadContext {
    /// Load a VAD model from file
    pub fn from_file(path: &str, params: &VadContextParams) -> Result<Self, WhisperError> {
        let c_path = CString::new(path).map_err(|_| WhisperError::InvalidInput)?;
        let vctx = unsafe {
            whisper_sys::whisper_vad_init_from_file_with_params(c_path.as_ptr(), params.raw)
        };
        if vctx.is_null() {
            Err(WhisperError::InitFailed)
        } else {
            Ok(VadContext { vctx })
        }
    }

    /// Load a VAD model using a custom model loader
    pub fn from_loader(
        loader: Box<dyn crate::types::ModelLoader>,
        params: &VadContextParams,
    ) -> Result<Self, WhisperError> {
        let mut c_loader = crate::types::make_c_model_loader(loader);
        let vctx = unsafe {
            whisper_sys::whisper_vad_init_with_params(&mut c_loader, params.raw)
        };
        if vctx.is_null() {
            Err(WhisperError::InitFailed)
        } else {
            Ok(VadContext { vctx })
        }
    }

    /// Detect speech segments in audio samples
    pub fn detect_speech(&mut self, samples: &[f32]) -> Result<bool, WhisperError> {
        let result = unsafe {
            whisper_sys::whisper_vad_detect_speech(
                self.vctx,
                samples.as_ptr(),
                samples.len() as i32,
            )
        };
        Ok(result)
    }

    /// Get the number of probability values
    pub fn n_probs(&self) -> i32 {
        unsafe { whisper_sys::whisper_vad_n_probs(self.vctx) }
    }

    /// Get the VAD probability array
    pub fn probs(&self) -> &[f32] {
        unsafe {
            let ptr = whisper_sys::whisper_vad_probs(self.vctx);
            let n = self.n_probs() as usize;
            if ptr.is_null() || n == 0 {
                &[]
            } else {
                std::slice::from_raw_parts(ptr, n)
            }
        }
    }

    /// Generate speech segments from probabilities
    pub fn segments_from_probs(&mut self, params: &VadParams) -> Result<VadSegments, WhisperError> {
        let segments = unsafe {
            whisper_sys::whisper_vad_segments_from_probs(self.vctx, params.raw)
        };
        if segments.is_null() {
            Err(WhisperError::ProcessingFailed)
        } else {
            Ok(VadSegments { segments })
        }
    }

    /// Generate speech segments directly from audio samples
    pub fn segments_from_samples(
        &mut self,
        params: &VadParams,
        samples: &[f32],
    ) -> Result<VadSegments, WhisperError> {
        let segments = unsafe {
            whisper_sys::whisper_vad_segments_from_samples(
                self.vctx,
                params.raw,
                samples.as_ptr(),
                samples.len() as i32,
            )
        };
        if segments.is_null() {
            Err(WhisperError::ProcessingFailed)
        } else {
            Ok(VadSegments { segments })
        }
    }
}

/// VAD speech segments result
pub struct VadSegments {
    segments: *mut whisper_sys::whisper_vad_segments,
}

unsafe impl Send for VadSegments {}

impl Drop for VadSegments {
    fn drop(&mut self) {
        if !self.segments.is_null() {
            unsafe {
                whisper_sys::whisper_vad_free_segments(self.segments);
            }
        }
    }
}

impl VadSegments {
    pub fn n_segments(&self) -> i32 {
        unsafe { whisper_sys::whisper_vad_segments_n_segments(self.segments) }
    }

    pub fn get_segment_t0(&self, i_segment: i32) -> f32 {
        unsafe { whisper_sys::whisper_vad_segments_get_segment_t0(self.segments, i_segment) }
    }

    pub fn get_segment_t1(&self, i_segment: i32) -> f32 {
        unsafe { whisper_sys::whisper_vad_segments_get_segment_t1(self.segments, i_segment) }
    }
}
