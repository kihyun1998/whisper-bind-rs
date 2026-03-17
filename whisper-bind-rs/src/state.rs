use std::ffi::CStr;

use crate::types::{TokenData, WhisperToken};

pub struct WhisperState {
    pub(crate) state: *mut whisper_sys::whisper_state,
}

unsafe impl Send for WhisperState {}

impl Drop for WhisperState {
    fn drop(&mut self) {
        if !self.state.is_null() {
            unsafe {
                whisper_sys::whisper_free_state(self.state);
            }
        }
    }
}

impl WhisperState {
    // ===== Model Info =====

    pub fn n_len(&self) -> i32 {
        unsafe { whisper_sys::whisper_n_len_from_state(self.state) }
    }

    // ===== Language =====

    pub fn lang_auto_detect(
        &mut self,
        ctx: &crate::context::WhisperContext,
        offset_ms: i32,
        n_threads: i32,
    ) -> Result<(i32, Vec<f32>), crate::error::WhisperError> {
        let n_langs = (crate::lang_max_id() + 1) as usize;
        let mut probs = vec![0.0f32; n_langs];
        let result = unsafe {
            whisper_sys::whisper_lang_auto_detect_with_state(
                ctx.ctx,
                self.state,
                offset_ms,
                n_threads,
                probs.as_mut_ptr(),
            )
        };
        if result < 0 {
            Err(crate::error::WhisperError::ProcessingFailed)
        } else {
            Ok((result, probs))
        }
    }

    // ===== OpenVINO =====

    pub fn init_openvino_encoder(
        &mut self,
        ctx: &crate::context::WhisperContext,
        model_path: Option<&str>,
        device: &str,
        cache_dir: Option<&str>,
    ) -> Result<(), crate::error::WhisperError> {
        use std::ffi::CString;
        let c_model = model_path
            .map(|s| CString::new(s).map_err(|_| crate::error::WhisperError::InvalidInput))
            .transpose()?;
        let c_device =
            CString::new(device).map_err(|_| crate::error::WhisperError::InvalidInput)?;
        let c_cache = cache_dir
            .map(|s| CString::new(s).map_err(|_| crate::error::WhisperError::InvalidInput))
            .transpose()?;

        let result = unsafe {
            whisper_sys::whisper_ctx_init_openvino_encoder_with_state(
                ctx.ctx,
                self.state,
                c_model.as_ref().map_or(std::ptr::null(), |c| c.as_ptr()),
                c_device.as_ptr(),
                c_cache.as_ref().map_or(std::ptr::null(), |c| c.as_ptr()),
            )
        };
        if result == 0 {
            Ok(())
        } else {
            Err(crate::error::WhisperError::InitFailed)
        }
    }

    // ===== Logits =====

    pub fn get_logits(&self) -> Option<&[f32]> {
        unsafe {
            let ptr = whisper_sys::whisper_get_logits_from_state(self.state);
            if ptr.is_null() {
                None
            } else {
                // Note: caller must know the vocab size to correctly use this
                // We return a minimal slice; use with context's n_vocab()
                Some(std::slice::from_raw_parts(ptr, 1))
            }
        }
    }

    // ===== Segment Results =====

    pub fn full_n_segments(&self) -> i32 {
        unsafe { whisper_sys::whisper_full_n_segments_from_state(self.state) }
    }

    pub fn full_lang_id(&self) -> i32 {
        unsafe { whisper_sys::whisper_full_lang_id_from_state(self.state) }
    }

    pub fn full_get_segment_t0(&self, i_segment: i32) -> i64 {
        unsafe { whisper_sys::whisper_full_get_segment_t0_from_state(self.state, i_segment) }
    }

    pub fn full_get_segment_t1(&self, i_segment: i32) -> i64 {
        unsafe { whisper_sys::whisper_full_get_segment_t1_from_state(self.state, i_segment) }
    }

    pub fn full_get_segment_text(&self, i_segment: i32) -> Option<&str> {
        unsafe {
            let ptr =
                whisper_sys::whisper_full_get_segment_text_from_state(self.state, i_segment);
            if ptr.is_null() {
                None
            } else {
                CStr::from_ptr(ptr).to_str().ok()
            }
        }
    }

    pub fn full_get_segment_speaker_turn_next(&self, i_segment: i32) -> bool {
        unsafe {
            whisper_sys::whisper_full_get_segment_speaker_turn_next_from_state(
                self.state, i_segment,
            )
        }
    }

    pub fn full_get_segment_no_speech_prob(&self, i_segment: i32) -> f32 {
        unsafe {
            whisper_sys::whisper_full_get_segment_no_speech_prob_from_state(
                self.state, i_segment,
            )
        }
    }

    // ===== Token-Level Results =====

    pub fn full_n_tokens(&self, i_segment: i32) -> i32 {
        unsafe { whisper_sys::whisper_full_n_tokens_from_state(self.state, i_segment) }
    }

    pub fn full_get_token_text<'a>(
        &self,
        ctx: &'a crate::context::WhisperContext,
        i_segment: i32,
        i_token: i32,
    ) -> Option<&'a str> {
        unsafe {
            let ptr = whisper_sys::whisper_full_get_token_text_from_state(
                ctx.ctx, self.state, i_segment, i_token,
            );
            if ptr.is_null() {
                None
            } else {
                CStr::from_ptr(ptr).to_str().ok()
            }
        }
    }

    pub fn full_get_token_id(&self, i_segment: i32, i_token: i32) -> WhisperToken {
        unsafe {
            whisper_sys::whisper_full_get_token_id_from_state(self.state, i_segment, i_token)
        }
    }

    pub fn full_get_token_data(&self, i_segment: i32, i_token: i32) -> TokenData {
        let raw = unsafe {
            whisper_sys::whisper_full_get_token_data_from_state(self.state, i_segment, i_token)
        };
        raw.into()
    }

    pub fn full_get_token_p(&self, i_segment: i32, i_token: i32) -> f32 {
        unsafe {
            whisper_sys::whisper_full_get_token_p_from_state(self.state, i_segment, i_token)
        }
    }

    // ===== Convenience =====

    pub fn get_text(&self) -> Result<String, crate::error::WhisperError> {
        let n_segments = self.full_n_segments();
        let mut text = String::new();
        for i in 0..n_segments {
            if let Some(seg_text) = self.full_get_segment_text(i) {
                text.push_str(seg_text);
            }
        }
        Ok(text)
    }

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
