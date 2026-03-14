# whisper-bind-rs

whisper.cpp를 바인딩하면서 Rust FFI를 학습하는 프로젝트.

## 배울 것

1. **git submodule** — 외부 C/C++ 소스를 레포에 포함시키는 방법
2. **Cargo workspace** — 여러 crate를 하나의 레포에서 관리하는 구조
3. **-sys crate 관례** — Rust 생태계에서 C 라이브러리 바인딩의 표준 패턴
4. **build.rs + cc crate** — 빌드 스크립트에서 C/C++ 소스를 컴파일하는 법
5. **bindgen** — C 헤더 파일로부터 Rust FFI 코드를 자동 생성하는 법
6. **unsafe FFI를 safe API로 감싸기** — raw 포인터, lifetime, Drop trait 활용
7. **테스트** — FFI 바인딩이 실제로 동작하는지 확인

## 방식

**bindgen + cc crate** 조합.

```
whisper-bind-rs/          # workspace root
├── Cargo.toml            # workspace 설정
├── whisper-sys/          # -sys crate (unsafe FFI)
│   ├── build.rs          # cc로 컴파일 + bindgen으로 바인딩 생성
│   ├── whisper.cpp/      # git submodule
│   └── src/lib.rs        # 바인딩 re-export
└── whisper-bind-rs/      # safe wrapper crate
    └── src/lib.rs        # safe API
```

## 진행 상황

- [x] 1. whisper.cpp submodule 추가
- [x] 2. workspace + crate 구조 잡기
- [ ] 3. build.rs 작성 (cc + bindgen)
- [ ] 4. whisper-sys 바인딩 확인
- [ ] 5. safe wrapper 작성
- [ ] 6. 테스트
