# whisper.cpp C API 함수 목록

> whisper.cpp v1.8.3 기준 (`whisper.h`) — 총 **97개** WHISPER_API 함수
>
> ✅ = 이미 Rust 래핑 완료 | ❌ = 미구현

---

## 1. 버전 정보

| 함수 | 상태 | 설명 |
|------|------|------|
| `whisper_version()` | ❌ | whisper.cpp 라이브러리 버전 문자열 반환 |

---

## 2. 컨텍스트 초기화 (Model Loading)

| 함수 | 상태 | 설명 |
|------|------|------|
| `whisper_init_from_file_with_params(path, params)` | ✅ | 파일 경로로부터 모델을 로드하고 컨텍스트 생성 |
| `whisper_init_from_buffer_with_params(buffer, size, params)` | ❌ | 메모리 버퍼로부터 모델을 로드하고 컨텍스트 생성 |
| `whisper_init_with_params(loader, params)` | ❌ | 커스텀 모델 로더를 사용하여 컨텍스트 생성 |
| `whisper_init_from_file_with_params_no_state(path, params)` | ❌ | 파일로부터 모델 로드 (내부 state 자동 할당 안함, 수동으로 `whisper_init_state()` 필요) |
| `whisper_init_from_buffer_with_params_no_state(buffer, size, params)` | ❌ | 버퍼로부터 모델 로드 (state 자동 할당 안함) |
| `whisper_init_with_params_no_state(loader, params)` | ❌ | 커스텀 로더로 모델 로드 (state 자동 할당 안함) |

### Deprecated 초기화 함수

| 함수 | 상태 | 설명 |
|------|------|------|
| `whisper_init_from_file(path)` | ❌ | ⚠️ Deprecated — `whisper_init_from_file_with_params` 사용 권장 |
| `whisper_init_from_buffer(buffer, size)` | ❌ | ⚠️ Deprecated — `whisper_init_from_buffer_with_params` 사용 권장 |
| `whisper_init(loader)` | ❌ | ⚠️ Deprecated — `whisper_init_with_params` 사용 권장 |
| `whisper_init_from_file_no_state(path)` | ❌ | ⚠️ Deprecated — `whisper_init_from_file_with_params_no_state` 사용 권장 |
| `whisper_init_from_buffer_no_state(buffer, size)` | ❌ | ⚠️ Deprecated — `whisper_init_from_buffer_with_params_no_state` 사용 권장 |
| `whisper_init_no_state(loader)` | ❌ | ⚠️ Deprecated — `whisper_init_with_params_no_state` 사용 권장 |

---

## 3. State 관리

| 함수 | 상태 | 설명 |
|------|------|------|
| `whisper_init_state(ctx)` | ❌ | 별도의 whisper_state를 생성. no_state 계열 init 함수와 함께 사용 |

---

## 4. OpenVINO 지원

| 함수 | 상태 | 설명 |
|------|------|------|
| `whisper_ctx_init_openvino_encoder_with_state(ctx, state, model_path, device, cache_dir)` | ❌ | 지정된 state에 대해 OpenVINO 인코더 초기화 |
| `whisper_ctx_init_openvino_encoder(ctx, model_path, device, cache_dir)` | ❌ | 기본 state에 대해 OpenVINO 인코더 초기화 |

---

## 5. 메모리 해제

| 함수 | 상태 | 설명 |
|------|------|------|
| `whisper_free(ctx)` | ✅ | 컨텍스트에 할당된 모든 메모리 해제 (`Drop` trait으로 구현) |
| `whisper_free_state(state)` | ❌ | whisper_state 메모리 해제 |
| `whisper_free_params(params)` | ❌ | whisper_full_params 메모리 해제 (by_ref로 할당된 경우) |
| `whisper_free_context_params(params)` | ❌ | whisper_context_params 메모리 해제 (by_ref로 할당된 경우) |

---

## 6. 오디오 처리 (Audio Processing)

| 함수 | 상태 | 설명 |
|------|------|------|
| `whisper_pcm_to_mel(ctx, samples, n_samples, n_threads)` | ❌ | RAW PCM 오디오를 log mel 스펙트로그램으로 변환 (기본 state에 저장) |
| `whisper_pcm_to_mel_with_state(ctx, state, samples, n_samples, n_threads)` | ❌ | 지정된 state에 log mel 스펙트로그램 저장 |
| `whisper_set_mel(ctx, data, n_len, n_mel)` | ❌ | 사전 계산된 커스텀 log mel 스펙트로그램을 기본 state에 설정 (n_mel=80) |
| `whisper_set_mel_with_state(ctx, state, data, n_len, n_mel)` | ❌ | 지정된 state에 커스텀 log mel 스펙트로그램 설정 |

---

## 7. 인코더 / 디코더 (Encoder / Decoder)

| 함수 | 상태 | 설명 |
|------|------|------|
| `whisper_encode(ctx, offset, n_threads)` | ❌ | 기본 state의 mel 스펙트로그램에 대해 인코더 실행 |
| `whisper_encode_with_state(ctx, state, offset, n_threads)` | ❌ | 지정된 state의 mel 스펙트로그램에 대해 인코더 실행 |
| `whisper_decode(ctx, tokens, n_tokens, n_past, n_threads)` | ❌ | 다음 토큰의 logits/확률을 얻기 위해 디코더 실행 |
| `whisper_decode_with_state(ctx, state, tokens, n_tokens, n_past, n_threads)` | ❌ | 지정된 state로 디코더 실행 |

---

## 8. 토큰화 (Tokenization)

| 함수 | 상태 | 설명 |
|------|------|------|
| `whisper_tokenize(ctx, text, tokens, n_max_tokens)` | ❌ | 텍스트를 토큰 배열로 변환. 성공 시 토큰 수 반환 |
| `whisper_token_count(ctx, text)` | ❌ | 텍스트의 토큰 수 반환 (`-whisper_tokenize(ctx, text, NULL, 0)`과 동일) |

---

## 9. 언어 (Language)

| 함수 | 상태 | 설명 |
|------|------|------|
| `whisper_lang_max_id()` | ❌ | 지원하는 최대 언어 ID 반환 (= 사용 가능한 언어 수 - 1) |
| `whisper_lang_id(lang)` | ❌ | 언어 문자열로 ID 조회 (e.g. "de" → 2, "german" → 2). 없으면 -1 |
| `whisper_lang_str(id)` | ❌ | 언어 ID로 짧은 코드 반환 (e.g. 2 → "de") |
| `whisper_lang_str_full(id)` | ❌ | 언어 ID로 전체 이름 반환 (e.g. 2 → "german") |
| `whisper_lang_auto_detect(ctx, offset_ms, n_threads, lang_probs)` | ❌ | mel 데이터를 기반으로 언어 자동 감지. 확률 배열에 각 언어별 확률 저장 |
| `whisper_lang_auto_detect_with_state(ctx, state, offset_ms, n_threads, lang_probs)` | ❌ | 지정된 state로 언어 자동 감지 |

---

## 10. 모델 정보 조회

| 함수 | 상태 | 설명 |
|------|------|------|
| `whisper_n_len(ctx)` | ❌ | mel 스펙트로그램 길이 반환 |
| `whisper_n_len_from_state(state)` | ❌ | 지정된 state의 mel 길이 반환 |
| `whisper_n_vocab(ctx)` | ❌ | 어휘(vocab) 크기 반환 |
| `whisper_n_text_ctx(ctx)` | ❌ | 텍스트 컨텍스트 크기 반환 |
| `whisper_n_audio_ctx(ctx)` | ❌ | 오디오 컨텍스트 크기 반환 |
| `whisper_is_multilingual(ctx)` | ❌ | 다국어 모델인지 여부 반환 |

---

## 11. 모델 아키텍처 정보

| 함수 | 상태 | 설명 |
|------|------|------|
| `whisper_model_n_vocab(ctx)` | ❌ | 모델 어휘 크기 |
| `whisper_model_n_audio_ctx(ctx)` | ❌ | 모델 오디오 컨텍스트 크기 |
| `whisper_model_n_audio_state(ctx)` | ❌ | 모델 오디오 state 차원 수 |
| `whisper_model_n_audio_head(ctx)` | ❌ | 모델 오디오 어텐션 헤드 수 |
| `whisper_model_n_audio_layer(ctx)` | ❌ | 모델 오디오 레이어 수 |
| `whisper_model_n_text_ctx(ctx)` | ❌ | 모델 텍스트 컨텍스트 크기 |
| `whisper_model_n_text_state(ctx)` | ❌ | 모델 텍스트 state 차원 수 |
| `whisper_model_n_text_head(ctx)` | ❌ | 모델 텍스트 어텐션 헤드 수 |
| `whisper_model_n_text_layer(ctx)` | ❌ | 모델 텍스트 레이어 수 |
| `whisper_model_n_mels(ctx)` | ❌ | 모델 mel 주파수 빈 수 |
| `whisper_model_ftype(ctx)` | ❌ | 모델 파일 타입 (양자화 타입 등) |
| `whisper_model_type(ctx)` | ❌ | 모델 타입 (tiny, base, small, medium, large 등) |
| `whisper_model_type_readable(ctx)` | ❌ | 사람이 읽을 수 있는 모델 타입 문자열 반환 |

---

## 12. Logits 접근

| 함수 | 상태 | 설명 |
|------|------|------|
| `whisper_get_logits(ctx)` | ❌ | 마지막 `whisper_decode()` 호출의 logits 포인터 반환 |
| `whisper_get_logits_from_state(state)` | ❌ | 지정된 state의 logits 포인터 반환 |

---

## 13. 토큰 ↔ 문자열 변환

| 함수 | 상태 | 설명 |
|------|------|------|
| `whisper_token_to_str(ctx, token)` | ❌ | 토큰 ID를 문자열로 변환 |

---

## 14. 특수 토큰 (Special Tokens)

| 함수 | 상태 | 설명 |
|------|------|------|
| `whisper_token_eot(ctx)` | ❌ | End of Text 토큰 ID |
| `whisper_token_sot(ctx)` | ❌ | Start of Text 토큰 ID |
| `whisper_token_solm(ctx)` | ❌ | Start of Last Message 토큰 ID |
| `whisper_token_prev(ctx)` | ❌ | Previous 토큰 ID |
| `whisper_token_nosp(ctx)` | ❌ | No Speech 토큰 ID |
| `whisper_token_not(ctx)` | ❌ | Not 토큰 ID |
| `whisper_token_beg(ctx)` | ❌ | Begin 토큰 ID (타임스탬프 시작) |
| `whisper_token_lang(ctx, lang_id)` | ❌ | 지정된 언어의 토큰 ID |

---

## 15. 태스크 토큰 (Task Tokens)

| 함수 | 상태 | 설명 |
|------|------|------|
| `whisper_token_translate(ctx)` | ❌ | 번역(translate) 태스크 토큰 ID |
| `whisper_token_transcribe(ctx)` | ❌ | 전사(transcribe) 태스크 토큰 ID |

---

## 16. 성능/타이밍 정보

| 함수 | 상태 | 설명 |
|------|------|------|
| `whisper_get_timings(ctx)` | ❌ | 성능 타이밍 구조체 포인터 반환 (sample/encode/decode/batchd/prompt ms) |
| `whisper_print_timings(ctx)` | ❌ | 타이밍 정보를 stderr에 출력 |
| `whisper_reset_timings(ctx)` | ❌ | 타이밍 카운터 초기화 |

---

## 17. 시스템 정보

| 함수 | 상태 | 설명 |
|------|------|------|
| `whisper_print_system_info()` | ❌ | CPU 기능(SSE, AVX 등) 등 시스템 정보 문자열 반환 |

---

## 18. 기본 파라미터 생성

| 함수 | 상태 | 설명 |
|------|------|------|
| `whisper_context_default_params()` | ✅ | 기본 컨텍스트 파라미터 구조체 반환 (값 복사) |
| `whisper_context_default_params_by_ref()` | ❌ | 기본 컨텍스트 파라미터를 힙에 할당하여 포인터 반환 (호출자가 `whisper_free_context_params`으로 해제 필요) |
| `whisper_full_default_params(strategy)` | ✅ | 기본 추론 파라미터 구조체 반환 (값 복사) |
| `whisper_full_default_params_by_ref(strategy)` | ❌ | 기본 추론 파라미터를 힙에 할당하여 포인터 반환 (호출자가 `whisper_free_params`으로 해제 필요) |

---

## 19. 전체 파이프라인 (Full Pipeline)

| 함수 | 상태 | 설명 |
|------|------|------|
| `whisper_full(ctx, params, samples, n_samples)` | ✅ | 전체 추론 파이프라인 실행: PCM → mel → 인코더 → 디코더 → 텍스트 |
| `whisper_full_with_state(ctx, state, params, samples, n_samples)` | ❌ | 지정된 state로 전체 파이프라인 실행 (멀티스레드 안전) |
| `whisper_full_parallel(ctx, params, samples, n_samples, n_processors)` | ❌ | 오디오를 청크로 분할하여 병렬 처리. 속도 향상 가능하나 청크 경계에서 정확도 저하 가능 |

---

## 20. 세그먼트 결과 조회

| 함수 | 상태 | 설명 |
|------|------|------|
| `whisper_full_n_segments(ctx)` | ✅ | 생성된 텍스트 세그먼트 수 반환 |
| `whisper_full_n_segments_from_state(state)` | ❌ | 지정된 state의 세그먼트 수 반환 |
| `whisper_full_lang_id(ctx)` | ❌ | 기본 state의 감지된 언어 ID 반환 |
| `whisper_full_lang_id_from_state(state)` | ❌ | 지정된 state의 감지된 언어 ID 반환 |
| `whisper_full_get_segment_t0(ctx, i_segment)` | ❌ | 세그먼트 시작 시간 (단위: 10ms, 즉 centiseconds) |
| `whisper_full_get_segment_t0_from_state(state, i_segment)` | ❌ | 지정된 state에서 세그먼트 시작 시간 |
| `whisper_full_get_segment_t1(ctx, i_segment)` | ❌ | 세그먼트 종료 시간 |
| `whisper_full_get_segment_t1_from_state(state, i_segment)` | ❌ | 지정된 state에서 세그먼트 종료 시간 |
| `whisper_full_get_segment_speaker_turn_next(ctx, i_segment)` | ❌ | 다음 세그먼트에서 화자 전환이 예측되는지 여부 (tinydiarize) |
| `whisper_full_get_segment_speaker_turn_next_from_state(state, i_segment)` | ❌ | 지정된 state에서 화자 전환 예측 여부 |
| `whisper_full_get_segment_text(ctx, i_segment)` | ✅ | 지정된 세그먼트의 텍스트 반환 |
| `whisper_full_get_segment_text_from_state(state, i_segment)` | ❌ | 지정된 state에서 세그먼트 텍스트 반환 |
| `whisper_full_get_segment_no_speech_prob(ctx, i_segment)` | ❌ | 세그먼트의 no_speech 확률 반환 |
| `whisper_full_get_segment_no_speech_prob_from_state(state, i_segment)` | ❌ | 지정된 state에서 no_speech 확률 반환 |

---

## 21. 토큰 레벨 결과 조회

| 함수 | 상태 | 설명 |
|------|------|------|
| `whisper_full_n_tokens(ctx, i_segment)` | ❌ | 지정된 세그먼트의 토큰 수 반환 |
| `whisper_full_n_tokens_from_state(state, i_segment)` | ❌ | 지정된 state에서 토큰 수 반환 |
| `whisper_full_get_token_text(ctx, i_segment, i_token)` | ❌ | 지정된 토큰의 텍스트 반환 |
| `whisper_full_get_token_text_from_state(ctx, state, i_segment, i_token)` | ❌ | 지정된 state에서 토큰 텍스트 반환 |
| `whisper_full_get_token_id(ctx, i_segment, i_token)` | ❌ | 지정된 토큰의 ID 반환 |
| `whisper_full_get_token_id_from_state(state, i_segment, i_token)` | ❌ | 지정된 state에서 토큰 ID 반환 |
| `whisper_full_get_token_data(ctx, i_segment, i_token)` | ❌ | 토큰 데이터 구조체 반환 (확률, 타임스탬프 등 포함) |
| `whisper_full_get_token_data_from_state(state, i_segment, i_token)` | ❌ | 지정된 state에서 토큰 데이터 반환 |
| `whisper_full_get_token_p(ctx, i_segment, i_token)` | ❌ | 지정된 토큰의 확률 반환 |
| `whisper_full_get_token_p_from_state(state, i_segment, i_token)` | ❌ | 지정된 state에서 토큰 확률 반환 |

---

## 22. Voice Activity Detection (VAD)

| 함수 | 상태 | 설명 |
|------|------|------|
| `whisper_vad_default_params()` | ❌ | VAD 기본 파라미터 반환 (threshold, min_speech_duration 등) |
| `whisper_vad_default_context_params()` | ❌ | VAD 컨텍스트 기본 파라미터 반환 (n_threads, use_gpu 등) |
| `whisper_vad_init_from_file_with_params(path, params)` | ❌ | 파일로부터 VAD 모델 로드 |
| `whisper_vad_init_with_params(loader, params)` | ❌ | 커스텀 로더로 VAD 모델 로드 |
| `whisper_vad_detect_speech(vctx, samples, n_samples)` | ❌ | 음성 구간 감지 실행 |
| `whisper_vad_n_probs(vctx)` | ❌ | VAD 확률 배열 길이 반환 |
| `whisper_vad_probs(vctx)` | ❌ | VAD 확률 배열 포인터 반환 |
| `whisper_vad_segments_from_probs(vctx, params)` | ❌ | 확률 기반으로 음성 세그먼트 생성 |
| `whisper_vad_segments_from_samples(vctx, params, samples, n_samples)` | ❌ | 오디오 샘플로부터 직접 음성 세그먼트 생성 |
| `whisper_vad_segments_n_segments(segments)` | ❌ | VAD 세그먼트 수 반환 |
| `whisper_vad_segments_get_segment_t0(segments, i_segment)` | ❌ | VAD 세그먼트 시작 시간 반환 |
| `whisper_vad_segments_get_segment_t1(segments, i_segment)` | ❌ | VAD 세그먼트 종료 시간 반환 |
| `whisper_vad_free_segments(segments)` | ❌ | VAD 세그먼트 메모리 해제 |
| `whisper_vad_free(ctx)` | ❌ | VAD 컨텍스트 메모리 해제 |

---

## 23. 벤치마크 / 유틸리티

| 함수 | 상태 | 설명 |
|------|------|------|
| `whisper_bench_memcpy(n_threads)` | ❌ | 메모리 복사 벤치마크 실행 |
| `whisper_bench_memcpy_str(n_threads)` | ❌ | 메모리 복사 벤치마크 결과를 문자열로 반환 |
| `whisper_bench_ggml_mul_mat(n_threads)` | ❌ | 행렬 곱셈 벤치마크 실행 |
| `whisper_bench_ggml_mul_mat_str(n_threads)` | ❌ | 행렬 곱셈 벤치마크 결과를 문자열로 반환 |

---

## 24. 로깅

| 함수 | 상태 | 설명 |
|------|------|------|
| `whisper_log_set(log_callback, user_data)` | ❌ | 로깅 콜백 설정 (기본: stderr 출력) |

---

## 구현 현황 요약

| 항목 | 수 |
|------|---|
| 전체 WHISPER_API 함수 | 97 |
| ✅ 구현 완료 | 7 |
| ❌ 미구현 | 90 |
| Deprecated 함수 (구현 불필요) | 6 |
| **실질 미구현 (Deprecated 제외)** | **84** |

### 구현 완료 함수 목록

1. `whisper_init_from_file_with_params` — `WhisperContext::from_file()`
2. `whisper_context_default_params` — `WhisperContext::from_file()` 내부
3. `whisper_free` — `WhisperContext::drop()`
4. `whisper_full` — `WhisperContext::recognize()`
5. `whisper_full_default_params` — `WhisperParams::new()`
6. `whisper_full_n_segments` — `WhisperContext::get_text()` 내부
7. `whisper_full_get_segment_text` — `WhisperContext::get_text()` 내부
