mod context;
mod error;
mod params;
mod state;
mod types;
mod vad;

pub use context::WhisperContext;
pub use error::WhisperError;
pub use params::{WhisperContextParams, WhisperContextParamsRef, WhisperFullParams, WhisperFullParamsRef};
pub use state::WhisperState;
pub use types::{ModelLoader, SamplingStrategy, Segment, Timings, TokenData, WhisperToken};
pub use vad::{VadContext, VadContextParams, VadParams, VadSegments};

// --- Constants ---

pub const SAMPLE_RATE: u32 = 16000;
pub const N_FFT: u32 = 400;
pub const HOP_LENGTH: u32 = 160;
pub const CHUNK_SIZE: u32 = 30;

// --- Free functions ---

/// Return the whisper.cpp library version string
pub fn version() -> &'static str {
    unsafe {
        let ptr = whisper_sys::whisper_version();
        std::ffi::CStr::from_ptr(ptr).to_str().unwrap_or("")
    }
}

/// Return system information string (CPU features like SSE, AVX, etc.)
pub fn system_info() -> &'static str {
    unsafe {
        let ptr = whisper_sys::whisper_print_system_info();
        std::ffi::CStr::from_ptr(ptr).to_str().unwrap_or("")
    }
}

/// Return the largest language id (number of available languages - 1)
pub fn lang_max_id() -> i32 {
    unsafe { whisper_sys::whisper_lang_max_id() }
}

/// Return the id of the specified language, or None if not found
pub fn lang_id(lang: &str) -> Option<i32> {
    let c_lang = std::ffi::CString::new(lang).ok()?;
    let id = unsafe { whisper_sys::whisper_lang_id(c_lang.as_ptr()) };
    if id < 0 { None } else { Some(id) }
}

/// Return the short language code for the given id (e.g. 2 -> "de")
pub fn lang_str(id: i32) -> Option<&'static str> {
    unsafe {
        let ptr = whisper_sys::whisper_lang_str(id);
        if ptr.is_null() {
            None
        } else {
            std::ffi::CStr::from_ptr(ptr).to_str().ok()
        }
    }
}

/// Return the full language name for the given id (e.g. 2 -> "german")
pub fn lang_str_full(id: i32) -> Option<&'static str> {
    unsafe {
        let ptr = whisper_sys::whisper_lang_str_full(id);
        if ptr.is_null() {
            None
        } else {
            std::ffi::CStr::from_ptr(ptr).to_str().ok()
        }
    }
}

/// Run memory copy benchmark, return timing result
pub fn bench_memcpy(n_threads: i32) -> i32 {
    unsafe { whisper_sys::whisper_bench_memcpy(n_threads) }
}

/// Run memory copy benchmark, return result as string
pub fn bench_memcpy_str(n_threads: i32) -> &'static str {
    unsafe {
        let ptr = whisper_sys::whisper_bench_memcpy_str(n_threads);
        std::ffi::CStr::from_ptr(ptr).to_str().unwrap_or("")
    }
}

/// Run ggml matrix multiplication benchmark, return timing result
pub fn bench_ggml_mul_mat(n_threads: i32) -> i32 {
    unsafe { whisper_sys::whisper_bench_ggml_mul_mat(n_threads) }
}

/// Run ggml matrix multiplication benchmark, return result as string
pub fn bench_ggml_mul_mat_str(n_threads: i32) -> &'static str {
    unsafe {
        let ptr = whisper_sys::whisper_bench_ggml_mul_mat_str(n_threads);
        std::ffi::CStr::from_ptr(ptr).to_str().unwrap_or("")
    }
}

// --- Logging ---

/// Set a custom log callback. Pass `None` to restore default (stderr) logging.
///
/// The callback receives the log level (as ggml_log_level) and the log message.
///
/// # Safety
/// The callback must be safe to call from any thread. It is stored as a global
/// function pointer inside whisper.cpp.
pub fn log_set(callback: Option<unsafe extern "C" fn(level: i32, text: *const std::ffi::c_char, user_data: *mut std::ffi::c_void)>) {
    unsafe {
        match callback {
            Some(cb) => {
                whisper_sys::whisper_log_set(Some(std::mem::transmute(cb)), std::ptr::null_mut());
            }
            None => {
                whisper_sys::whisper_log_set(None, std::ptr::null_mut());
            }
        }
    }
}

/// Suppress all whisper.cpp log output
pub fn log_suppress() {
    unsafe extern "C" fn noop(_level: i32, _text: *const std::ffi::c_char, _user_data: *mut std::ffi::c_void) {}
    unsafe {
        whisper_sys::whisper_log_set(Some(std::mem::transmute(noop as unsafe extern "C" fn(i32, *const std::ffi::c_char, *mut std::ffi::c_void))), std::ptr::null_mut());
    }
}

// --- Tests ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        let v = version();
        assert!(!v.is_empty(), "version should not be empty");
        println!("whisper version: {}", v);
    }

    #[test]
    fn test_system_info() {
        let info = system_info();
        assert!(!info.is_empty(), "system info should not be empty");
        println!("system info: {}", info);
    }

    #[test]
    fn test_lang_max_id() {
        let max_id = lang_max_id();
        assert!(max_id > 0, "should support multiple languages");
        println!("lang max id: {}", max_id);
    }

    #[test]
    fn test_lang_id() {
        assert_eq!(lang_id("en"), Some(0));
        assert!(lang_id("ko").is_some());
        assert!(lang_id("de").is_some());
        assert_eq!(lang_id("xyznotreal"), None);
    }

    #[test]
    fn test_lang_str() {
        assert_eq!(lang_str(0), Some("en"));
        assert!(lang_str(-1).is_none());
        assert!(lang_str(99999).is_none());
    }

    #[test]
    fn test_lang_str_full() {
        assert_eq!(lang_str_full(0), Some("english"));
        assert!(lang_str_full(-1).is_none());
    }

    #[test]
    fn test_default_context_params() {
        let params = WhisperContextParams::default();
        // Just verify it doesn't crash and has reasonable defaults
        let _ = params;
    }

    #[test]
    fn test_full_params_greedy() {
        let mut params = WhisperFullParams::new(SamplingStrategy::Greedy);
        params.set_language("en").set_n_threads(4).set_print_progress(false);
        // Verify chaining works without crash
    }

    #[test]
    fn test_full_params_beam_search() {
        let mut params = WhisperFullParams::new(SamplingStrategy::BeamSearch);
        params.set_beam_size(5).set_beam_patience(1.0);
        // Verify beam search params work
    }

    #[test]
    fn test_sampling_strategy() {
        assert_eq!(SamplingStrategy::Greedy, SamplingStrategy::Greedy);
        assert_ne!(SamplingStrategy::Greedy, SamplingStrategy::BeamSearch);
    }

    #[test]
    fn test_vad_default_params() {
        let mut params = VadParams::default();
        params.set_threshold(0.5).set_min_speech_duration_ms(250);
        // Verify creation and setter chaining
    }

    #[test]
    fn test_vad_default_context_params() {
        let mut params = VadContextParams::default();
        params.set_n_threads(4).set_use_gpu(false);
        // Verify creation and setter chaining
    }

    #[test]
    fn test_full_params_all_setters() {
        let mut p = WhisperFullParams::new(SamplingStrategy::Greedy);
        p.set_n_threads(4)
            .set_n_max_text_ctx(16384)
            .set_offset_ms(0)
            .set_duration_ms(0)
            .set_translate(false)
            .set_no_context(true)
            .set_no_timestamps(false)
            .set_single_segment(false)
            .set_print_special(false)
            .set_print_progress(false)
            .set_print_realtime(false)
            .set_print_timestamps(true)
            .set_token_timestamps(false)
            .set_thold_pt(0.01)
            .set_thold_ptsum(0.01)
            .set_max_len(0)
            .set_split_on_word(false)
            .set_max_tokens(0)
            .set_debug_mode(false)
            .set_audio_ctx(0)
            .set_tdrz_enable(false)
            .set_suppress_regex("")
            .set_initial_prompt("")
            .set_language("en")
            .set_detect_language(false)
            .set_suppress_blank(true)
            .set_suppress_nst(false)
            .set_temperature(0.0)
            .set_max_initial_ts(1.0)
            .set_length_penalty(-1.0)
            .set_temperature_inc(0.2)
            .set_entropy_thold(2.4)
            .set_logprob_thold(-1.0)
            .set_no_speech_thold(0.6)
            .set_greedy_best_of(5)
            .set_beam_size(5)
            .set_beam_patience(-1.0);
    }

    #[test]
    fn test_log_suppress() {
        log_suppress();
        // After suppressing, whisper log calls should not panic
        let _ = version();
    }
}
