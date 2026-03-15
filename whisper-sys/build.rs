fn main() {
    // 1. cc로 whisper.cpp 컴파일
    cc::Build::new()
        .cpp(true)
        .file("whisper.cpp/src/whisper.cpp")
        .include("whisper.cpp/include")
        .include("whisper.cpp/ggml/include")
        .define("WHISPER_VERSION", "\"1.8.3\"")
        .compile("whisper");

    // 2. bindgen으로 바인딩 생성
    let bindings = bindgen::Builder::default()
        .header("whisper.cpp/include/whisper.h")
        .clang_arg("-Iwhisper.cpp/ggml/include")
        .generate()
        .expect("bindgen 실패");

    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .unwrap();
}
