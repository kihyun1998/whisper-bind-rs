use std::ffi::CString;

use crate::types::SamplingStrategy;

/// Parameters for creating a whisper context
#[derive(Debug, Clone)]
pub struct WhisperContextParams {
    pub(crate) raw: whisper_sys::whisper_context_params,
}

impl WhisperContextParams {
    pub fn use_gpu(mut self, enable: bool) -> Self {
        self.raw.use_gpu = enable;
        self
    }

    pub fn flash_attn(mut self, enable: bool) -> Self {
        self.raw.flash_attn = enable;
        self
    }

    pub fn gpu_device(mut self, device: i32) -> Self {
        self.raw.gpu_device = device;
        self
    }
}

impl Default for WhisperContextParams {
    fn default() -> Self {
        let raw = unsafe { whisper_sys::whisper_context_default_params() };
        WhisperContextParams { raw }
    }
}

/// Heap-allocated context params (returned by whisper_context_default_params_by_ref).
/// Automatically freed on drop via whisper_free_context_params.
pub struct WhisperContextParamsRef {
    ptr: *mut whisper_sys::whisper_context_params,
}

impl WhisperContextParamsRef {
    /// Create default context params allocated on the heap.
    pub fn new() -> Self {
        let ptr = unsafe { whisper_sys::whisper_context_default_params_by_ref() };
        WhisperContextParamsRef { ptr }
    }

    /// Get a reference to the underlying params
    pub fn as_params(&self) -> &whisper_sys::whisper_context_params {
        unsafe { &*self.ptr }
    }

    /// Get a mutable reference to the underlying params
    pub fn as_params_mut(&mut self) -> &mut whisper_sys::whisper_context_params {
        unsafe { &mut *self.ptr }
    }
}

impl Drop for WhisperContextParamsRef {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                whisper_sys::whisper_free_context_params(self.ptr);
            }
        }
    }
}

/// Parameters for whisper_full inference
pub struct WhisperFullParams {
    pub(crate) raw: whisper_sys::whisper_full_params,
    // Keep owned strings alive so raw pointers remain valid
    _language: Option<CString>,
    _initial_prompt: Option<CString>,
    _suppress_regex: Option<CString>,
}

impl WhisperFullParams {
    pub fn new(strategy: SamplingStrategy) -> Self {
        let raw = unsafe { whisper_sys::whisper_full_default_params(strategy.to_raw()) };
        WhisperFullParams {
            raw,
            _language: None,
            _initial_prompt: None,
            _suppress_regex: None,
        }
    }

    // --- Core settings ---

    pub fn set_n_threads(&mut self, n: i32) -> &mut Self {
        self.raw.n_threads = n;
        self
    }

    pub fn set_n_max_text_ctx(&mut self, n: i32) -> &mut Self {
        self.raw.n_max_text_ctx = n;
        self
    }

    pub fn set_offset_ms(&mut self, ms: i32) -> &mut Self {
        self.raw.offset_ms = ms;
        self
    }

    pub fn set_duration_ms(&mut self, ms: i32) -> &mut Self {
        self.raw.duration_ms = ms;
        self
    }

    pub fn set_translate(&mut self, enable: bool) -> &mut Self {
        self.raw.translate = enable;
        self
    }

    pub fn set_no_context(&mut self, enable: bool) -> &mut Self {
        self.raw.no_context = enable;
        self
    }

    pub fn set_no_timestamps(&mut self, enable: bool) -> &mut Self {
        self.raw.no_timestamps = enable;
        self
    }

    pub fn set_single_segment(&mut self, enable: bool) -> &mut Self {
        self.raw.single_segment = enable;
        self
    }

    // --- Print settings ---

    pub fn set_print_special(&mut self, enable: bool) -> &mut Self {
        self.raw.print_special = enable;
        self
    }

    pub fn set_print_progress(&mut self, enable: bool) -> &mut Self {
        self.raw.print_progress = enable;
        self
    }

    pub fn set_print_realtime(&mut self, enable: bool) -> &mut Self {
        self.raw.print_realtime = enable;
        self
    }

    pub fn set_print_timestamps(&mut self, enable: bool) -> &mut Self {
        self.raw.print_timestamps = enable;
        self
    }

    // --- Token-level timestamps ---

    pub fn set_token_timestamps(&mut self, enable: bool) -> &mut Self {
        self.raw.token_timestamps = enable;
        self
    }

    pub fn set_thold_pt(&mut self, val: f32) -> &mut Self {
        self.raw.thold_pt = val;
        self
    }

    pub fn set_thold_ptsum(&mut self, val: f32) -> &mut Self {
        self.raw.thold_ptsum = val;
        self
    }

    pub fn set_max_len(&mut self, val: i32) -> &mut Self {
        self.raw.max_len = val;
        self
    }

    pub fn set_split_on_word(&mut self, enable: bool) -> &mut Self {
        self.raw.split_on_word = enable;
        self
    }

    pub fn set_max_tokens(&mut self, val: i32) -> &mut Self {
        self.raw.max_tokens = val;
        self
    }

    // --- Experimental ---

    pub fn set_debug_mode(&mut self, enable: bool) -> &mut Self {
        self.raw.debug_mode = enable;
        self
    }

    pub fn set_audio_ctx(&mut self, val: i32) -> &mut Self {
        self.raw.audio_ctx = val;
        self
    }

    pub fn set_tdrz_enable(&mut self, enable: bool) -> &mut Self {
        self.raw.tdrz_enable = enable;
        self
    }

    // --- Language ---

    pub fn set_language(&mut self, lang: &str) -> &mut Self {
        if let Ok(c) = CString::new(lang) {
            self.raw.language = c.as_ptr();
            self._language = Some(c);
        }
        self
    }

    pub fn set_detect_language(&mut self, enable: bool) -> &mut Self {
        self.raw.detect_language = enable;
        self
    }

    // --- Prompts ---

    pub fn set_initial_prompt(&mut self, prompt: &str) -> &mut Self {
        if let Ok(c) = CString::new(prompt) {
            self.raw.initial_prompt = c.as_ptr();
            self._initial_prompt = Some(c);
        }
        self
    }

    pub fn set_suppress_regex(&mut self, regex: &str) -> &mut Self {
        if let Ok(c) = CString::new(regex) {
            self.raw.suppress_regex = c.as_ptr();
            self._suppress_regex = Some(c);
        }
        self
    }

    // --- Decoding parameters ---

    pub fn set_suppress_blank(&mut self, enable: bool) -> &mut Self {
        self.raw.suppress_blank = enable;
        self
    }

    pub fn set_suppress_nst(&mut self, enable: bool) -> &mut Self {
        self.raw.suppress_nst = enable;
        self
    }

    pub fn set_temperature(&mut self, val: f32) -> &mut Self {
        self.raw.temperature = val;
        self
    }

    pub fn set_max_initial_ts(&mut self, val: f32) -> &mut Self {
        self.raw.max_initial_ts = val;
        self
    }

    pub fn set_length_penalty(&mut self, val: f32) -> &mut Self {
        self.raw.length_penalty = val;
        self
    }

    // --- Fallback parameters ---

    pub fn set_temperature_inc(&mut self, val: f32) -> &mut Self {
        self.raw.temperature_inc = val;
        self
    }

    pub fn set_entropy_thold(&mut self, val: f32) -> &mut Self {
        self.raw.entropy_thold = val;
        self
    }

    pub fn set_logprob_thold(&mut self, val: f32) -> &mut Self {
        self.raw.logprob_thold = val;
        self
    }

    pub fn set_no_speech_thold(&mut self, val: f32) -> &mut Self {
        self.raw.no_speech_thold = val;
        self
    }

    // --- Strategy-specific ---

    pub fn set_greedy_best_of(&mut self, val: i32) -> &mut Self {
        self.raw.greedy.best_of = val;
        self
    }

    pub fn set_beam_size(&mut self, val: i32) -> &mut Self {
        self.raw.beam_search.beam_size = val;
        self
    }

    pub fn set_beam_patience(&mut self, val: f32) -> &mut Self {
        self.raw.beam_search.patience = val;
        self
    }
}

/// Heap-allocated full params (returned by whisper_full_default_params_by_ref).
/// Automatically freed on drop via whisper_free_params.
pub struct WhisperFullParamsRef {
    ptr: *mut whisper_sys::whisper_full_params,
}

impl WhisperFullParamsRef {
    /// Create default full params allocated on the heap.
    pub fn new(strategy: SamplingStrategy) -> Self {
        let ptr = unsafe { whisper_sys::whisper_full_default_params_by_ref(strategy.to_raw()) };
        WhisperFullParamsRef { ptr }
    }

    /// Get a reference to the underlying params
    pub fn as_params(&self) -> &whisper_sys::whisper_full_params {
        unsafe { &*self.ptr }
    }

    /// Get a mutable reference to the underlying params
    pub fn as_params_mut(&mut self) -> &mut whisper_sys::whisper_full_params {
        unsafe { &mut *self.ptr }
    }
}

impl Drop for WhisperFullParamsRef {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                whisper_sys::whisper_free_params(self.ptr);
            }
        }
    }
}
