# What is ggml?

A **machine learning tensor computation library** written in C. Created by Georgi Gerganov — the name comes from his initials (GG).

## Why does it exist?

Frameworks like PyTorch and TensorFlow are heavy. They require a Python runtime, CUDA dependencies, and gigabytes of libraries. ggml aims to **run inference on CPU with pure C and no external dependencies**.

- No external dependencies (pure C)
- CPU-optimized (leverages SIMD: AVX, ARM NEON, etc.)
- Model quantization to drastically reduce memory usage
- Run LLMs, speech models, etc. locally without a GPU

## Relationship with whisper.cpp

whisper.cpp runs the OpenAI Whisper model on top of ggml. Both were created by Georgi Gerganov.

```
OpenAI Whisper (Python/PyTorch)
        ↓ model conversion
ggml format model file (.bin)
        ↓
whisper.cpp (C/C++ inference engine)
   └── ggml (tensor computation library)
```

In other words, ggml is the **computation backend** for whisper.cpp. It handles the low-level operations needed for neural networks: matrix multiplication, convolution, attention, etc.

## ggml model format

PyTorch `.pt` files cannot be used directly. They must be converted to ggml's own binary format.

- Model weights are stored as ggml tensor structures
- Supports quantization to 16-bit, 8-bit, 4-bit, etc.
- Quantization significantly reduces model size (e.g., fp16 → 4-bit is roughly 1/4)

The conversion scripts in whisper.cpp's `models/` directory handle this.

## ggml ecosystem

Projects built on ggml:

| Project | Purpose |
|---------|---------|
| **whisper.cpp** | Speech recognition (Whisper) |
| **llama.cpp** | LLM inference (LLaMA, etc.) |
| **stable-diffusion.cpp** | Image generation |

They all follow the same pattern: convert a Python/PyTorch model to ggml format and run inference in C/C++.

## Role in this project

In `whisper-sys/build.rs`, we compile the ggml source directly:

```rust
// Compile ggml C files
cc::Build::new()
    .files(&[
        "whisper.cpp/ggml/src/ggml.c",
        "whisper.cpp/ggml/src/ggml-alloc.c",
        "whisper.cpp/ggml/src/ggml-quants.c",
    ])
    .compile("ggml");

// Compile ggml C++ files
cc::Build::new()
    .cpp(true)
    .files(&[
        "whisper.cpp/ggml/src/ggml-backend.cpp",
        // ...
    ])
    .compile("ggml-cpp");
```

Since whisper.cpp depends on ggml, we must compile the ggml source alongside it. This is why "add ggml-cpu source" was listed as todo item 6-6.
