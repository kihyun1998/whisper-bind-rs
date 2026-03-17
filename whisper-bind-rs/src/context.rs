use std::ffi::{CStr, CString};

use crate::error::WhisperError;
use crate::params::{WhisperContextParams, WhisperFullParams};
use crate::state::WhisperState;
use crate::types::{Timings, WhisperToken};

pub struct WhisperContext {
    pub(crate) ctx: *mut whisper_sys::whisper_context,
}

unsafe impl Send for WhisperContext {}

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
    // ===== Initialization =====

    /// Load a model from file with the given context parameters
    pub fn from_file(path: &str, params: &WhisperContextParams) -> Result<Self, WhisperError> {
        let c_path = CString::new(path).map_err(|_| WhisperError::InvalidInput)?;
        let ctx = unsafe {
            whisper_sys::whisper_init_from_file_with_params(c_path.as_ptr(), params.raw)
        };
        if ctx.is_null() {
            Err(WhisperError::InitFailed)
        } else {
            Ok(WhisperContext { ctx })
        }
    }

    /// Load a model from a memory buffer
    pub fn from_buffer(buffer: &[u8], params: &WhisperContextParams) -> Result<Self, WhisperError> {
        let ctx = unsafe {
            whisper_sys::whisper_init_from_buffer_with_params(
                buffer.as_ptr() as *mut std::ffi::c_void,
                buffer.len(),
                params.raw,
            )
        };
        if ctx.is_null() {
            Err(WhisperError::InitFailed)
        } else {
            Ok(WhisperContext { ctx })
        }
    }

    /// Load a model using a custom model loader
    pub fn from_loader(
        loader: Box<dyn crate::types::ModelLoader>,
        params: &WhisperContextParams,
    ) -> Result<Self, WhisperError> {
        let mut c_loader = crate::types::make_c_model_loader(loader);
        let ctx = unsafe {
            whisper_sys::whisper_init_with_params(&mut c_loader, params.raw)
        };
        if ctx.is_null() {
            Err(WhisperError::InitFailed)
        } else {
            Ok(WhisperContext { ctx })
        }
    }

    /// Load a model from file without allocating internal state.
    /// Use `init_state()` to create a state manually.
    pub fn from_file_no_state(
        path: &str,
        params: &WhisperContextParams,
    ) -> Result<Self, WhisperError> {
        let c_path = CString::new(path).map_err(|_| WhisperError::InvalidInput)?;
        let ctx = unsafe {
            whisper_sys::whisper_init_from_file_with_params_no_state(c_path.as_ptr(), params.raw)
        };
        if ctx.is_null() {
            Err(WhisperError::InitFailed)
        } else {
            Ok(WhisperContext { ctx })
        }
    }

    /// Load a model using a custom loader without allocating internal state.
    pub fn from_loader_no_state(
        loader: Box<dyn crate::types::ModelLoader>,
        params: &WhisperContextParams,
    ) -> Result<Self, WhisperError> {
        let mut c_loader = crate::types::make_c_model_loader(loader);
        let ctx = unsafe {
            whisper_sys::whisper_init_with_params_no_state(&mut c_loader, params.raw)
        };
        if ctx.is_null() {
            Err(WhisperError::InitFailed)
        } else {
            Ok(WhisperContext { ctx })
        }
    }

    /// Load a model from a memory buffer without allocating internal state.
    pub fn from_buffer_no_state(
        buffer: &[u8],
        params: &WhisperContextParams,
    ) -> Result<Self, WhisperError> {
        let ctx = unsafe {
            whisper_sys::whisper_init_from_buffer_with_params_no_state(
                buffer.as_ptr() as *mut std::ffi::c_void,
                buffer.len(),
                params.raw,
            )
        };
        if ctx.is_null() {
            Err(WhisperError::InitFailed)
        } else {
            Ok(WhisperContext { ctx })
        }
    }

    // ===== State =====

    /// Create a new state for this context
    pub fn init_state(&self) -> Result<WhisperState, WhisperError> {
        let state = unsafe { whisper_sys::whisper_init_state(self.ctx) };
        if state.is_null() {
            Err(WhisperError::InitFailed)
        } else {
            Ok(WhisperState { state })
        }
    }

    // ===== Model Info =====

    pub fn n_len(&self) -> i32 {
        unsafe { whisper_sys::whisper_n_len(self.ctx) }
    }

    pub fn n_vocab(&self) -> i32 {
        unsafe { whisper_sys::whisper_n_vocab(self.ctx) }
    }

    pub fn n_text_ctx(&self) -> i32 {
        unsafe { whisper_sys::whisper_n_text_ctx(self.ctx) }
    }

    pub fn n_audio_ctx(&self) -> i32 {
        unsafe { whisper_sys::whisper_n_audio_ctx(self.ctx) }
    }

    pub fn is_multilingual(&self) -> bool {
        unsafe { whisper_sys::whisper_is_multilingual(self.ctx) != 0 }
    }

    // ===== Model Architecture Info =====

    pub fn model_n_vocab(&self) -> i32 {
        unsafe { whisper_sys::whisper_model_n_vocab(self.ctx) }
    }

    pub fn model_n_audio_ctx(&self) -> i32 {
        unsafe { whisper_sys::whisper_model_n_audio_ctx(self.ctx) }
    }

    pub fn model_n_audio_state(&self) -> i32 {
        unsafe { whisper_sys::whisper_model_n_audio_state(self.ctx) }
    }

    pub fn model_n_audio_head(&self) -> i32 {
        unsafe { whisper_sys::whisper_model_n_audio_head(self.ctx) }
    }

    pub fn model_n_audio_layer(&self) -> i32 {
        unsafe { whisper_sys::whisper_model_n_audio_layer(self.ctx) }
    }

    pub fn model_n_text_ctx(&self) -> i32 {
        unsafe { whisper_sys::whisper_model_n_text_ctx(self.ctx) }
    }

    pub fn model_n_text_state(&self) -> i32 {
        unsafe { whisper_sys::whisper_model_n_text_state(self.ctx) }
    }

    pub fn model_n_text_head(&self) -> i32 {
        unsafe { whisper_sys::whisper_model_n_text_head(self.ctx) }
    }

    pub fn model_n_text_layer(&self) -> i32 {
        unsafe { whisper_sys::whisper_model_n_text_layer(self.ctx) }
    }

    pub fn model_n_mels(&self) -> i32 {
        unsafe { whisper_sys::whisper_model_n_mels(self.ctx) }
    }

    pub fn model_ftype(&self) -> i32 {
        unsafe { whisper_sys::whisper_model_ftype(self.ctx) }
    }

    pub fn model_type(&self) -> i32 {
        unsafe { whisper_sys::whisper_model_type(self.ctx) }
    }

    pub fn model_type_readable(&self) -> &str {
        unsafe {
            let ptr = whisper_sys::whisper_model_type_readable(self.ctx);
            if ptr.is_null() {
                ""
            } else {
                CStr::from_ptr(ptr).to_str().unwrap_or("")
            }
        }
    }

    // ===== Tokens =====

    pub fn token_to_str(&self, token: WhisperToken) -> Option<&str> {
        unsafe {
            let ptr = whisper_sys::whisper_token_to_str(self.ctx, token);
            if ptr.is_null() {
                None
            } else {
                CStr::from_ptr(ptr).to_str().ok()
            }
        }
    }

    pub fn tokenize(&self, text: &str) -> Result<Vec<WhisperToken>, WhisperError> {
        let c_text = CString::new(text).map_err(|_| WhisperError::InvalidInput)?;
        // First, get the token count
        let count =
            unsafe { whisper_sys::whisper_token_count(self.ctx, c_text.as_ptr()) };
        if count <= 0 {
            return Ok(Vec::new());
        }
        let mut tokens = vec![0i32; count as usize];
        let result = unsafe {
            whisper_sys::whisper_tokenize(
                self.ctx,
                c_text.as_ptr(),
                tokens.as_mut_ptr(),
                count,
            )
        };
        if result < 0 {
            Err(WhisperError::ProcessingFailed)
        } else {
            tokens.truncate(result as usize);
            Ok(tokens)
        }
    }

    pub fn token_count(&self, text: &str) -> Result<i32, WhisperError> {
        let c_text = CString::new(text).map_err(|_| WhisperError::InvalidInput)?;
        let count =
            unsafe { whisper_sys::whisper_token_count(self.ctx, c_text.as_ptr()) };
        Ok(count)
    }

    // ===== Special Tokens =====

    pub fn token_eot(&self) -> WhisperToken {
        unsafe { whisper_sys::whisper_token_eot(self.ctx) }
    }

    pub fn token_sot(&self) -> WhisperToken {
        unsafe { whisper_sys::whisper_token_sot(self.ctx) }
    }

    pub fn token_solm(&self) -> WhisperToken {
        unsafe { whisper_sys::whisper_token_solm(self.ctx) }
    }

    pub fn token_prev(&self) -> WhisperToken {
        unsafe { whisper_sys::whisper_token_prev(self.ctx) }
    }

    pub fn token_nosp(&self) -> WhisperToken {
        unsafe { whisper_sys::whisper_token_nosp(self.ctx) }
    }

    pub fn token_not(&self) -> WhisperToken {
        unsafe { whisper_sys::whisper_token_not(self.ctx) }
    }

    pub fn token_beg(&self) -> WhisperToken {
        unsafe { whisper_sys::whisper_token_beg(self.ctx) }
    }

    pub fn token_lang(&self, lang_id: i32) -> WhisperToken {
        unsafe { whisper_sys::whisper_token_lang(self.ctx, lang_id) }
    }

    // ===== Task Tokens =====

    pub fn token_translate(&self) -> WhisperToken {
        unsafe { whisper_sys::whisper_token_translate(self.ctx) }
    }

    pub fn token_transcribe(&self) -> WhisperToken {
        unsafe { whisper_sys::whisper_token_transcribe(self.ctx) }
    }

    // ===== Language Auto-detect =====

    /// Detect the spoken language from mel data.
    /// Returns (language_id, probabilities_per_language).
    pub fn lang_auto_detect(
        &self,
        offset_ms: i32,
        n_threads: i32,
    ) -> Result<(i32, Vec<f32>), WhisperError> {
        let n_langs = (crate::lang_max_id() + 1) as usize;
        let mut probs = vec![0.0f32; n_langs];
        let result = unsafe {
            whisper_sys::whisper_lang_auto_detect(
                self.ctx,
                offset_ms,
                n_threads,
                probs.as_mut_ptr(),
            )
        };
        if result < 0 {
            Err(WhisperError::ProcessingFailed)
        } else {
            Ok((result, probs))
        }
    }

    // ===== OpenVINO =====

    /// Initialize OpenVINO encoder for the default state
    pub fn init_openvino_encoder(
        &self,
        model_path: Option<&str>,
        device: &str,
        cache_dir: Option<&str>,
    ) -> Result<(), WhisperError> {
        let c_model = model_path
            .map(|s| CString::new(s).map_err(|_| WhisperError::InvalidInput))
            .transpose()?;
        let c_device = CString::new(device).map_err(|_| WhisperError::InvalidInput)?;
        let c_cache = cache_dir
            .map(|s| CString::new(s).map_err(|_| WhisperError::InvalidInput))
            .transpose()?;

        let result = unsafe {
            whisper_sys::whisper_ctx_init_openvino_encoder(
                self.ctx,
                c_model.as_ref().map_or(std::ptr::null(), |c| c.as_ptr()),
                c_device.as_ptr(),
                c_cache.as_ref().map_or(std::ptr::null(), |c| c.as_ptr()),
            )
        };
        if result == 0 {
            Ok(())
        } else {
            Err(WhisperError::InitFailed)
        }
    }

    // ===== Timings =====

    pub fn get_timings(&self) -> Option<Timings> {
        unsafe {
            let ptr = whisper_sys::whisper_get_timings(self.ctx);
            if ptr.is_null() {
                None
            } else {
                Some(Timings {
                    sample_ms: (*ptr).sample_ms,
                    encode_ms: (*ptr).encode_ms,
                    decode_ms: (*ptr).decode_ms,
                    batchd_ms: (*ptr).batchd_ms,
                    prompt_ms: (*ptr).prompt_ms,
                })
            }
        }
    }

    pub fn print_timings(&self) {
        unsafe { whisper_sys::whisper_print_timings(self.ctx) }
    }

    pub fn reset_timings(&self) {
        unsafe { whisper_sys::whisper_reset_timings(self.ctx) }
    }

    // ===== Audio Processing =====

    /// Convert RAW PCM audio to log mel spectrogram (stored in default state)
    pub fn pcm_to_mel(&self, samples: &[f32], n_threads: i32) -> Result<(), WhisperError> {
        let result = unsafe {
            whisper_sys::whisper_pcm_to_mel(
                self.ctx,
                samples.as_ptr(),
                samples.len() as i32,
                n_threads,
            )
        };
        if result == 0 {
            Ok(())
        } else {
            Err(WhisperError::ProcessingFailed)
        }
    }

    /// Convert PCM to mel spectrogram using a specific state
    pub fn pcm_to_mel_with_state(
        &self,
        state: &mut WhisperState,
        samples: &[f32],
        n_threads: i32,
    ) -> Result<(), WhisperError> {
        let result = unsafe {
            whisper_sys::whisper_pcm_to_mel_with_state(
                self.ctx,
                state.state,
                samples.as_ptr(),
                samples.len() as i32,
                n_threads,
            )
        };
        if result == 0 {
            Ok(())
        } else {
            Err(WhisperError::ProcessingFailed)
        }
    }

    /// Set a custom log mel spectrogram in the default state
    pub fn set_mel(&self, data: &[f32], n_len: i32, n_mel: i32) -> Result<(), WhisperError> {
        let result = unsafe {
            whisper_sys::whisper_set_mel(self.ctx, data.as_ptr(), n_len, n_mel)
        };
        if result == 0 {
            Ok(())
        } else {
            Err(WhisperError::ProcessingFailed)
        }
    }

    /// Set a custom log mel spectrogram in a specific state
    pub fn set_mel_with_state(
        &self,
        state: &mut WhisperState,
        data: &[f32],
        n_len: i32,
        n_mel: i32,
    ) -> Result<(), WhisperError> {
        let result = unsafe {
            whisper_sys::whisper_set_mel_with_state(
                self.ctx,
                state.state,
                data.as_ptr(),
                n_len,
                n_mel,
            )
        };
        if result == 0 {
            Ok(())
        } else {
            Err(WhisperError::ProcessingFailed)
        }
    }

    // ===== Encoder / Decoder =====

    pub fn encode(&self, offset: i32, n_threads: i32) -> Result<(), WhisperError> {
        let result = unsafe { whisper_sys::whisper_encode(self.ctx, offset, n_threads) };
        if result == 0 {
            Ok(())
        } else {
            Err(WhisperError::EncodeFailed)
        }
    }

    pub fn encode_with_state(
        &self,
        state: &mut WhisperState,
        offset: i32,
        n_threads: i32,
    ) -> Result<(), WhisperError> {
        let result = unsafe {
            whisper_sys::whisper_encode_with_state(self.ctx, state.state, offset, n_threads)
        };
        if result == 0 {
            Ok(())
        } else {
            Err(WhisperError::EncodeFailed)
        }
    }

    pub fn decode(
        &self,
        tokens: &[WhisperToken],
        n_past: i32,
        n_threads: i32,
    ) -> Result<(), WhisperError> {
        let result = unsafe {
            whisper_sys::whisper_decode(
                self.ctx,
                tokens.as_ptr(),
                tokens.len() as i32,
                n_past,
                n_threads,
            )
        };
        if result == 0 {
            Ok(())
        } else {
            Err(WhisperError::DecodeFailed)
        }
    }

    pub fn decode_with_state(
        &self,
        state: &mut WhisperState,
        tokens: &[WhisperToken],
        n_past: i32,
        n_threads: i32,
    ) -> Result<(), WhisperError> {
        let result = unsafe {
            whisper_sys::whisper_decode_with_state(
                self.ctx,
                state.state,
                tokens.as_ptr(),
                tokens.len() as i32,
                n_past,
                n_threads,
            )
        };
        if result == 0 {
            Ok(())
        } else {
            Err(WhisperError::DecodeFailed)
        }
    }

    // ===== Logits =====

    /// Get logits from the last decode() call (default state).
    /// The slice has shape [n_tokens * n_vocab].
    pub fn get_logits(&self) -> Option<&[f32]> {
        unsafe {
            let ptr = whisper_sys::whisper_get_logits(self.ctx);
            if ptr.is_null() {
                None
            } else {
                let n_vocab = self.n_vocab() as usize;
                Some(std::slice::from_raw_parts(ptr, n_vocab))
            }
        }
    }

    // ===== Full Pipeline =====

    /// Run the full whisper pipeline: PCM -> mel -> encoder -> decoder -> text
    pub fn full(&mut self, params: &WhisperFullParams, audio: &[f32]) -> Result<(), WhisperError> {
        let result = unsafe {
            whisper_sys::whisper_full(self.ctx, params.raw, audio.as_ptr(), audio.len() as i32)
        };
        if result == 0 {
            Ok(())
        } else {
            Err(WhisperError::ProcessingFailed)
        }
    }

    /// Run full pipeline with a specific state (thread-safe)
    pub fn full_with_state(
        &self,
        state: &mut WhisperState,
        params: &WhisperFullParams,
        audio: &[f32],
    ) -> Result<(), WhisperError> {
        let result = unsafe {
            whisper_sys::whisper_full_with_state(
                self.ctx,
                state.state,
                params.raw,
                audio.as_ptr(),
                audio.len() as i32,
            )
        };
        if result == 0 {
            Ok(())
        } else {
            Err(WhisperError::ProcessingFailed)
        }
    }

    /// Run full pipeline in parallel (splits audio into chunks)
    pub fn full_parallel(
        &mut self,
        params: &WhisperFullParams,
        audio: &[f32],
        n_processors: i32,
    ) -> Result<(), WhisperError> {
        let result = unsafe {
            whisper_sys::whisper_full_parallel(
                self.ctx,
                params.raw,
                audio.as_ptr(),
                audio.len() as i32,
                n_processors,
            )
        };
        if result == 0 {
            Ok(())
        } else {
            Err(WhisperError::ProcessingFailed)
        }
    }

    // ===== Segment Results =====

    pub fn full_n_segments(&self) -> i32 {
        unsafe { whisper_sys::whisper_full_n_segments(self.ctx) }
    }

    pub fn full_lang_id(&self) -> i32 {
        unsafe { whisper_sys::whisper_full_lang_id(self.ctx) }
    }

    pub fn full_get_segment_t0(&self, i_segment: i32) -> i64 {
        unsafe { whisper_sys::whisper_full_get_segment_t0(self.ctx, i_segment) }
    }

    pub fn full_get_segment_t1(&self, i_segment: i32) -> i64 {
        unsafe { whisper_sys::whisper_full_get_segment_t1(self.ctx, i_segment) }
    }

    pub fn full_get_segment_text(&self, i_segment: i32) -> Option<&str> {
        unsafe {
            let ptr = whisper_sys::whisper_full_get_segment_text(self.ctx, i_segment);
            if ptr.is_null() {
                None
            } else {
                CStr::from_ptr(ptr).to_str().ok()
            }
        }
    }

    pub fn full_get_segment_speaker_turn_next(&self, i_segment: i32) -> bool {
        unsafe { whisper_sys::whisper_full_get_segment_speaker_turn_next(self.ctx, i_segment) }
    }

    pub fn full_get_segment_no_speech_prob(&self, i_segment: i32) -> f32 {
        unsafe { whisper_sys::whisper_full_get_segment_no_speech_prob(self.ctx, i_segment) }
    }

    // ===== Token-Level Results =====

    pub fn full_n_tokens(&self, i_segment: i32) -> i32 {
        unsafe { whisper_sys::whisper_full_n_tokens(self.ctx, i_segment) }
    }

    pub fn full_get_token_text(&self, i_segment: i32, i_token: i32) -> Option<&str> {
        unsafe {
            let ptr = whisper_sys::whisper_full_get_token_text(self.ctx, i_segment, i_token);
            if ptr.is_null() {
                None
            } else {
                CStr::from_ptr(ptr).to_str().ok()
            }
        }
    }

    pub fn full_get_token_id(&self, i_segment: i32, i_token: i32) -> WhisperToken {
        unsafe { whisper_sys::whisper_full_get_token_id(self.ctx, i_segment, i_token) }
    }

    pub fn full_get_token_data(
        &self,
        i_segment: i32,
        i_token: i32,
    ) -> crate::types::TokenData {
        let raw =
            unsafe { whisper_sys::whisper_full_get_token_data(self.ctx, i_segment, i_token) };
        raw.into()
    }

    pub fn full_get_token_p(&self, i_segment: i32, i_token: i32) -> f32 {
        unsafe { whisper_sys::whisper_full_get_token_p(self.ctx, i_segment, i_token) }
    }

    // ===== Convenience Methods =====

    /// Get all text from the last `full()` call
    pub fn get_text(&self) -> Result<String, WhisperError> {
        let n_segments = self.full_n_segments();
        let mut text = String::new();
        for i in 0..n_segments {
            if let Some(seg_text) = self.full_get_segment_text(i) {
                text.push_str(seg_text);
            }
        }
        Ok(text)
    }

    /// Get all segments with full metadata from the last `full()` call
    pub fn segments(&self) -> Vec<crate::types::Segment> {
        let n = self.full_n_segments();
        (0..n)
            .map(|i| crate::types::Segment {
                t0: self.full_get_segment_t0(i),
                t1: self.full_get_segment_t1(i),
                text: self.full_get_segment_text(i).unwrap_or("").to_string(),
                no_speech_prob: self.full_get_segment_no_speech_prob(i),
                speaker_turn_next: self.full_get_segment_speaker_turn_next(i),
            })
            .collect()
    }
}
