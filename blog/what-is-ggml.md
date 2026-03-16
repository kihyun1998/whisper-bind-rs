# ggml이란?

C로 작성된 **머신러닝 텐서 연산 라이브러리**다. Georgi Gerganov가 만들었고, 이름도 그의 이니셜(GG)에서 따왔다.

## 왜 존재하는가

PyTorch나 TensorFlow 같은 프레임워크는 무겁다. Python 런타임, CUDA 의존성, 수 GB의 라이브러리가 필요하다. ggml은 이런 의존성 없이 **순수 C로 CPU에서 추론을 돌리는 것**이 목표다.

- 외부 의존성 없음 (순수 C)
- CPU 최적화 (AVX, ARM NEON 등 SIMD 활용)
- 모델을 양자화(quantization)해서 메모리 사용량을 크게 줄임
- GPU 없이도 로컬에서 LLM, 음성 모델 등을 실행 가능

## whisper.cpp와의 관계

whisper.cpp는 OpenAI Whisper 모델을 ggml 위에서 돌리는 프로젝트다. 둘 다 Georgi Gerganov가 만들었다.

```
OpenAI Whisper (Python/PyTorch)
        ↓ 모델 변환
ggml 포맷 모델 파일 (.bin)
        ↓
whisper.cpp (C/C++ 추론 엔진)
   └── ggml (텐서 연산 라이브러리)
```

즉 ggml은 whisper.cpp의 **연산 백엔드**다. 행렬 곱셈, 컨볼루션, attention 등 신경망에 필요한 저수준 연산을 담당한다.

## ggml 모델 포맷

PyTorch의 `.pt` 파일을 그대로 쓸 수 없다. ggml 자체 바이너리 포맷으로 변환해야 한다.

- 모델 가중치를 ggml 텐서 구조로 저장
- 16-bit, 8-bit, 4-bit 등으로 양자화 가능
- 양자화하면 모델 크기가 크게 줄어듦 (예: fp16 → 4bit이면 약 1/4)

whisper.cpp의 `models/` 디렉토리에 있는 변환 스크립트들이 이 작업을 해준다.

## ggml 생태계

ggml을 기반으로 한 프로젝트들:

| 프로젝트 | 용도 |
|----------|------|
| **whisper.cpp** | 음성 인식 (Whisper) |
| **llama.cpp** | LLM 추론 (LLaMA 등) |
| **stable-diffusion.cpp** | 이미지 생성 |

모두 같은 패턴이다: Python/PyTorch 모델을 ggml 포맷으로 변환하고, C/C++로 추론한다.

## 이 프로젝트에서의 역할

`whisper-sys/build.rs`에서 ggml 소스를 직접 컴파일하고 있다:

```rust
// ggml C 파일 컴파일
cc::Build::new()
    .files(&[
        "whisper.cpp/ggml/src/ggml.c",
        "whisper.cpp/ggml/src/ggml-alloc.c",
        "whisper.cpp/ggml/src/ggml-quants.c",
    ])
    .compile("ggml");

// ggml C++ 파일 컴파일
cc::Build::new()
    .cpp(true)
    .files(&[
        "whisper.cpp/ggml/src/ggml-backend.cpp",
        // ...
    ])
    .compile("ggml-cpp");
```

whisper.cpp가 ggml에 의존하기 때문에, whisper.cpp를 빌드하려면 ggml 소스도 함께 컴파일해야 한다. 이것이 todo 6-6에 "ggml-cpu 소스 추가"가 있었던 이유다.
