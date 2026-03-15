fn main() {
    cc::Build::new()
        .cpp(true)
        .file("whisper.cpp/src/whisper.cpp")
        .include("whisper.cpp/include")
        .include("whisper.cpp/ggml/include")
        .define("WHISPER_VERSION", "\"1.8.3\"")
        .compile("whisper");
}
