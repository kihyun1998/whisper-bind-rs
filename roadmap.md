# whisper-bind-rs 전체 API 래핑 구현 계획

## Context
현재 whisper.cpp v1.8.3의 97개 C API 함수 중 7개만 구현된 상태. Deprecated 6개 제외 84개 함수를 안전한 Rust 래퍼로 구현해야 함. 단계별로 구현하고 각 단계마다 테스트를 수행.

## 모듈 구조
```
whisper-bind-rs/src/
├── lib.rs          # re-exports, 상수, free functions (version, system_info, lang_*, benchmarks)
├── error.rs        # WhisperError
├── context.rs      # WhisperContext (모든 ctx 메서드)
├── state.rs        # WhisperState (모든 state 메서드)
├── params.rs       # WhisperContextParams, WhisperFullParams, SamplingStrategy
├── types.rs        # TokenData, Timings, WhisperToken alias 등
├── vad.rs          # VadContext, VadSegments, VadParams, VadContextParams
```

---

## Phase 1: 기반 구조 (Types + Error + Params + Free Functions)

**파일**: `error.rs`, `types.rs`, `params.rs`, `lib.rs`

### error.rs
- `WhisperError` enum 확장: `InitFailed`, `InvalidInput`, `ProcessingFailed`, `EncodeFailed`, `DecodeFailed`, `Utf8Error`, `NullPointer`
- `std::fmt::Display`, `std::error::Error` 구현

### types.rs
- `pub type WhisperToken = i32;`
- `TokenData` struct (id, tid, p, plog, pt, ptsum, t0, t1, t_dtw, vlen)
- `Timings` struct (sample_ms, encode_ms, decode_ms, batchd_ms, prompt_ms)
- `SamplingStrategy` enum (Greedy, BeamSearch)

### params.rs
- `WhisperContextParams` struct (use_gpu, flash_attn, gpu_device, dtw 관련)
  - `Default` impl → `whisper_context_default_params()` 호출
- `WhisperFullParams` struct (내부에 whisper_sys::whisper_full_params 래핑)
  - `new(strategy)` → `whisper_full_default_params()`
  - Builder 메서드들: `set_language()`, `set_translate()`, `set_n_threads()`, `set_token_timestamps()`, `set_single_segment()`, `set_print_progress()`, `set_initial_prompt()` 등 주요 필드 setter

### lib.rs
- 상수: `SAMPLE_RATE`, `N_FFT`, `HOP_LENGTH`, `CHUNK_SIZE`
- `version() -> &str`
- `system_info() -> &str`
- `lang_max_id() -> i32`
- `lang_id(lang: &str) -> Option<i32>`
- `lang_str(id: i32) -> Option<&str>`
- `lang_str_full(id: i32) -> Option<&str>`
- `bench_memcpy(n_threads: i32) -> i32`
- `bench_memcpy_str(n_threads: i32) -> &str`
- `bench_ggml_mul_mat(n_threads: i32) -> i32`
- `bench_ggml_mul_mat_str(n_threads: i32) -> &str`
- 모든 모듈 re-export

### 테스트
```rust
#[test] fn test_version()         // whisper_version() 반환값 확인
#[test] fn test_system_info()     // whisper_print_system_info() 반환값 확인
#[test] fn test_lang_max_id()     // > 0 확인
#[test] fn test_lang_id()         // "en" → 0, "ko" → 유효값, "xyz" → None
#[test] fn test_lang_str()        // 0 → "en"
#[test] fn test_lang_str_full()   // 0 → "english"
#[test] fn test_default_params()  // WhisperContextParams::default(), WhisperFullParams::new()
#[test] fn test_sampling_strategy() // Greedy, BeamSearch 둘 다 생성
```

→ `cargo test` 실행하여 검증 (모델 파일 불필요)

---

## Phase 2: Context + State + Model Info

**파일**: `context.rs`, `state.rs`

### context.rs - WhisperContext
**초기화**:
- `from_file(path, params: &WhisperContextParams)` — 기존 개선
- `from_buffer(buffer: &[u8], params: &WhisperContextParams)`
- `from_file_no_state(path, params)` — state 없이 로드
- `from_buffer_no_state(buffer, params)`
- Drop trait → `whisper_free()`

**모델 정보** (모두 `&self` 메서드):
- `n_len()`, `n_vocab()`, `n_text_ctx()`, `n_audio_ctx()`, `is_multilingual()`
- `model_n_vocab()`, `model_n_audio_ctx()`, `model_n_audio_state()`, `model_n_audio_head()`
- `model_n_audio_layer()`, `model_n_text_ctx()`, `model_n_text_state()`, `model_n_text_head()`
- `model_n_text_layer()`, `model_n_mels()`, `model_ftype()`, `model_type()`
- `model_type_readable() -> &str`

**토큰 관련**:
- `token_to_str(token) -> Option<&str>`
- `tokenize(text) -> Result<Vec<WhisperToken>>`
- `token_count(text) -> i32`
- 특수 토큰: `token_eot()`, `token_sot()`, `token_solm()`, `token_prev()`, `token_nosp()`, `token_not()`, `token_beg()`, `token_lang(lang_id)`
- 태스크 토큰: `token_translate()`, `token_transcribe()`

**언어**:
- `lang_auto_detect(offset_ms, n_threads) -> Result<(i32, Vec<f32>)>`

**OpenVINO**:
- `init_openvino_encoder(model_path, device, cache_dir) -> Result<()>`

**타이밍**:
- `get_timings() -> Option<Timings>`
- `print_timings()`
- `reset_timings()`

### state.rs - WhisperState
- `WhisperState` struct (내부 `*mut whisper_sys::whisper_state`)
- `WhisperContext::init_state() -> Result<WhisperState>`
- Drop → `whisper_free_state()`
- `n_len() -> i32`
- `lang_auto_detect(ctx, offset_ms, n_threads) -> Result<(i32, Vec<f32>)>`
- `init_openvino_encoder(ctx, model_path, device, cache_dir) -> Result<()>`

### 테스트
```rust
#[test] fn test_context_params_default()  // WhisperContextParams::default() 생성
// 모델이 있으면:
// #[test] fn test_from_file()            // 모델 로드
// #[test] fn test_model_info()           // n_vocab > 0 등
```

→ `cargo test` + `cargo build` 확인

---

## Phase 3: Full Pipeline + Segment/Token Results

**파일**: `context.rs`, `state.rs`, `params.rs`

### params.rs - WhisperFullParams 나머지 setter
- 콜백 관련은 제외 (또는 간단한 progress callback만)
- `set_no_context()`, `set_no_timestamps()`, `set_print_special()`, `set_print_realtime()`, `set_print_timestamps()`
- `set_thold_pt()`, `set_thold_ptsum()`, `set_max_len()`, `set_split_on_word()`, `set_max_tokens()`
- `set_debug_mode()`, `set_audio_ctx()`, `set_tdrz_enable()`, `set_suppress_regex()`
- `set_detect_language()`, `set_suppress_blank()`, `set_suppress_nst()`
- `set_temperature()`, `set_max_initial_ts()`, `set_length_penalty()`
- `set_temperature_inc()`, `set_entropy_thold()`, `set_logprob_thold()`, `set_no_speech_thold()`
- `set_greedy_best_of()`, `set_beam_size()`, `set_beam_patience()`

### context.rs - 파이프라인 & 결과
**파이프라인**:
- `recognize(&mut self, params, audio)` → 기존 (whisper_full)
- `recognize_parallel(&mut self, params, audio, n_processors)` → whisper_full_parallel

**세그먼트 결과** (`&self`):
- `full_n_segments()`, `full_lang_id()`
- `full_get_segment_t0(i)`, `full_get_segment_t1(i)`
- `full_get_segment_text(i) -> Option<&str>`
- `full_get_segment_speaker_turn_next(i) -> bool`
- `full_get_segment_no_speech_prob(i) -> f32`

**토큰 레벨 결과**:
- `full_n_tokens(i_segment)`
- `full_get_token_text(i_segment, i_token) -> Option<&str>`
- `full_get_token_id(i_segment, i_token)`
- `full_get_token_data(i_segment, i_token) -> TokenData`
- `full_get_token_p(i_segment, i_token) -> f32`

**편의 메서드** (기존 `get_text()` 유지 + 확장):
- `segments() -> Vec<Segment>` — Segment { t0, t1, text, no_speech_prob, speaker_turn_next }

### state.rs - State 기반 파이프라인 & 결과
- `recognize_with_state(ctx, state, params, audio)`
- 세그먼트/토큰 결과의 `_from_state` 버전들

### 오디오 처리 & 인코더/디코더 (context.rs)
- `pcm_to_mel(samples, n_threads)`, `pcm_to_mel_with_state(state, samples, n_threads)`
- `set_mel(data, n_len, n_mel)`, `set_mel_with_state(state, data, n_len, n_mel)`
- `encode(offset, n_threads)`, `encode_with_state(state, offset, n_threads)`
- `decode(tokens, n_past, n_threads)`, `decode_with_state(state, tokens, n_past, n_threads)`
- `get_logits() -> Option<&[f32]>`, `get_logits_from_state(state) -> Option<&[f32]>`

### 테스트
```rust
#[test] fn test_full_params_builder()  // 빌더 체이닝 테스트
#[test] fn test_segments_struct()      // Segment 타입 확인
// 모델 테스트 (선택):
// #[test] fn test_full_pipeline()     // 오디오 → 텍스트 전체 흐름
```

→ `cargo test` + `cargo build` 확인

---

## Phase 4: VAD + Benchmarks + Logging

**파일**: `vad.rs`, `lib.rs`

### vad.rs
- `VadParams` struct → `Default` impl (`whisper_vad_default_params()`)
  - setter: `set_threshold()`, `set_min_speech_duration_ms()`, 등
- `VadContextParams` struct → `Default` impl
  - setter: `set_n_threads()`, `set_use_gpu()`, `set_gpu_device()`
- `VadContext` struct
  - `from_file(path, params)` → `whisper_vad_init_from_file_with_params`
  - `detect_speech(samples) -> Result<bool>`
  - `n_probs() -> i32`
  - `probs() -> &[f32]`
  - `segments_from_probs(params) -> VadSegments`
  - `segments_from_samples(params, samples) -> VadSegments`
  - Drop → `whisper_vad_free()`
- `VadSegments` struct
  - `n_segments() -> i32`
  - `get_segment_t0(i) -> f32`
  - `get_segment_t1(i) -> f32`
  - Drop → `whisper_vad_free_segments()`

### lib.rs - Logging
- `log_set(callback)` — `Option<fn(level, text)>` 형태로 래핑

### 테스트
```rust
#[test] fn test_vad_default_params()          // VadParams::default()
#[test] fn test_vad_default_context_params()  // VadContextParams::default()
#[test] fn test_bench_memcpy_str()            // 결과 문자열 확인 (느릴 수 있음)
```

→ `cargo test` + 최종 `cargo build` 확인

---

## 구현 제외 항목
- Deprecated 함수 6개 (문서에서 권장하지 않음)
- `whisper_init_with_params` / `whisper_init_with_params_no_state` (커스텀 모델 로더 — 안전한 Rust 래핑이 복잡, 필요시 추후 추가)
- `whisper_vad_init_with_params` (커스텀 모델 로더)
- `whisper_free_params`, `whisper_free_context_params` (Rust에서는 stack 값으로 관리하므로 by_ref 할당 불필요)
- `whisper_context_default_params_by_ref`, `whisper_full_default_params_by_ref` (by_ref 변형 불필요)

## 검증
각 Phase 완료 후:
1. `cargo build` — 컴파일 확인
2. `cargo test` — 단위 테스트
3. Phase별 테스트 가능한 free function 우선 검증
