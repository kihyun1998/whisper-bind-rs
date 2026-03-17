fn main() {
    // 1. ggml c file compile
    let mut ggml_c = cc::Build::new();
    ggml_c
        .files(&[
            "whisper.cpp/ggml/src/ggml.c",
            "whisper.cpp/ggml/src/ggml-alloc.c",
            "whisper.cpp/ggml/src/ggml-quants.c",
        ])
        .include("whisper.cpp/ggml/include")
        .include("whisper.cpp/ggml/src")
        .include("whisper.cpp/ggml/src/ggml-cpu")
        .define("GGML_VERSION", "\"0.9.7\"")
        .define("GGML_COMMIT", "\"\"");

    if ggml_c.get_compiler().is_like_msvc() {
        ggml_c.flag("/w");
    } else {
        ggml_c.flag("-w");
    }
    ggml_c.compile("ggml");

    // 2. ggml c++ file compile
    let mut ggml_cpp = cc::Build::new();
    ggml_cpp
        .cpp(true)
        .files(&[
            "whisper.cpp/ggml/src/ggml-backend.cpp",
            "whisper.cpp/ggml/src/ggml-backend-reg.cpp",
            "whisper.cpp/ggml/src/ggml-backend-dl.cpp",
            "whisper.cpp/ggml/src/ggml-opt.cpp",
            "whisper.cpp/ggml/src/ggml-threading.cpp",
            "whisper.cpp/ggml/src/ggml.cpp",
            "whisper.cpp/ggml/src/gguf.cpp",
        ])
        .include("whisper.cpp/ggml/include");

    if ggml_cpp.get_compiler().is_like_msvc() {
        ggml_cpp.flag("/std:c++17").flag("/w");
    } else {
        ggml_cpp.flag("-std=c++17").flag("-w");
    }
    ggml_cpp.compile("ggml-cpp");

    // 3. whisper.cpp compile
    let mut whisper = cc::Build::new();
    whisper
        .cpp(true)
        .file("whisper.cpp/src/whisper.cpp")
        .include("whisper.cpp/include")
        .include("whisper.cpp/ggml/include")
        .define("WHISPER_VERSION", "\"1.8.3\"");

    if whisper.get_compiler().is_like_msvc() {
        whisper.flag("/std:c++17").flag("/utf-8").flag("/w");
    } else {
        whisper.flag("-std=c++17").flag("-w");
    }
    whisper.compile("whisper");

    // 4. bindgen으로 바인딩 생성
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
