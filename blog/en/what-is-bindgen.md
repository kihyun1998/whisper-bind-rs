# What is bindgen?

A tool that reads C/C++ header files (`.h`) and **automatically generates** Rust FFI binding code.

## Why use it?

To call a C library from Rust, you need to manually declare function signatures, structs, constants, etc. in an `extern "C"` block.

```rust
extern "C" {
    fn whisper_init(path: *const c_char) -> *mut whisper_context;
    fn whisper_free(ctx: *mut whisper_context);
    // ... dozens of functions written by hand
}
```

For large header files, writing this manually is impractical and a single typo can cause memory bugs. bindgen automates this process.

## How it works

1. Parses C/C++ header files using **libclang**
2. Analyzes functions, structs, enums, constants, typedefs, etc.
3. Generates corresponding Rust FFI code

```
whisper.h  →  [bindgen + libclang]  →  bindings.rs
```

## Usage

### 1. Add dependency

```toml
[build-dependencies]
bindgen = "0.71"
```

### 2. Generate bindings in build.rs

```rust
let bindings = bindgen::Builder::default()
    .header("whisper.cpp/include/whisper.h")
    .clang_arg("-Iwhisper.cpp/ggml/include")  // additional include path
    .generate()
    .expect("bindgen failed");

let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
bindings.write_to_file(out_path.join("bindings.rs")).unwrap();
```

### 3. Import in lib.rs

```rust
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
```

The `allow` attributes are needed because C-style names (`whisper_full_params`, etc.) trigger Rust naming convention warnings.

## Example of generated code

Given this C code in a header:

```c
struct whisper_context;
int whisper_full(struct whisper_context * ctx, struct whisper_full_params params, const float * samples, int n_samples);
```

bindgen generates this Rust code:

```rust
#[repr(C)]
pub struct whisper_context { _unused: [u8; 0] }

extern "C" {
    pub fn whisper_full(
        ctx: *mut whisper_context,
        params: whisper_full_params,
        samples: *const f32,
        n_samples: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
```

## What is OUT_DIR?

`OUT_DIR` is an environment variable provided by Cargo to build scripts, pointing to a temporary directory for storing generated files. It points to a path like `target/debug/build/<crate>/out/`.

Generated files don't pollute the source tree and are imported at compile time via the `include!` macro.

## Things to note

- bindgen uses **libclang** internally, so clang must be installed on the system (on macOS, it's included in Xcode Command Line Tools).
- C++ templates may not generate bindings correctly. Targeting C interfaces (`extern "C"`) is the most reliable approach.
- All generated code is `unsafe`. Wrapping it in a safe API is done in a separate wrapper crate.
