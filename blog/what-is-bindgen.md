# bindgen이란?

C/C++ 헤더 파일(`.h`)을 읽어서 Rust FFI 바인딩 코드를 **자동으로 생성**해주는 도구다.

## 왜 쓰는가

C 라이브러리를 Rust에서 쓰려면 `extern "C"` 블록에 함수 시그니처, 구조체, 상수 등을 일일이 선언해야 한다.

```rust
extern "C" {
    fn whisper_init(path: *const c_char) -> *mut whisper_context;
    fn whisper_free(ctx: *mut whisper_context);
    // ... 수십 개의 함수를 수동으로 작성
}
```

헤더 파일이 크면 이걸 손으로 쓰는 건 비현실적이고, 오타 하나로 메모리 버그가 생길 수 있다. bindgen은 이 과정을 자동화한다.

## 어떻게 동작하는가

1. C/C++ 헤더 파일을 **libclang**으로 파싱한다
2. 함수, 구조체, enum, 상수, typedef 등을 분석한다
3. 대응하는 Rust FFI 코드를 생성한다

```
whisper.h  →  [bindgen + libclang]  →  bindings.rs
```

## 사용법

### 1. 의존성 추가

```toml
[build-dependencies]
bindgen = "0.71"
```

### 2. build.rs에서 바인딩 생성

```rust
let bindings = bindgen::Builder::default()
    .header("whisper.cpp/include/whisper.h")
    .clang_arg("-Iwhisper.cpp/ggml/include")  // 추가 include 경로
    .generate()
    .expect("bindgen 실패");

let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
bindings.write_to_file(out_path.join("bindings.rs")).unwrap();
```

### 3. lib.rs에서 가져오기

```rust
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
```

`allow` 속성은 C 스타일 이름(`whisper_full_params` 등)이 Rust 네이밍 컨벤션 경고를 발생시키기 때문에 필요하다.

## 생성되는 코드 예시

헤더에 이런 C 코드가 있으면:

```c
struct whisper_context;
int whisper_full(struct whisper_context * ctx, struct whisper_full_params params, const float * samples, int n_samples);
```

bindgen이 이런 Rust 코드를 생성한다:

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

## OUT_DIR이란?

`OUT_DIR`은 Cargo가 빌드 스크립트에 제공하는 환경변수로, 빌드 중 생성되는 파일을 저장하는 임시 디렉토리다. `target/debug/build/<crate>/out/` 같은 경로를 가리킨다.

생성된 파일은 소스 트리를 오염시키지 않고, `include!` 매크로로 컴파일 타임에 가져온다.

## 주의할 점

- bindgen은 내부적으로 **libclang**을 사용하므로, 시스템에 clang이 설치되어 있어야 한다 (macOS는 Xcode Command Line Tools에 포함).
- C++ 템플릿은 바인딩이 제대로 생성되지 않을 수 있다. C 인터페이스(`extern "C"`)를 대상으로 쓰는 것이 가장 안정적이다.
- 생성된 코드는 모두 `unsafe`다. safe API로 감싸는 것은 별도의 wrapper crate에서 한다.
