# 자막 출력 테스트 가이드

## 사전 준비

### 1. whisper 모델 다운로드

```bash
# ggml-base 모델 (~148MB)
# https://huggingface.co/ggerganov/whisper.cpp/tree/main
# ggml-base.bin 다운로드 후 프로젝트 루트에 models/ 폴더 생성하여 저장

mkdir models
# ggml-base.bin을 models/ 에 넣기
```

### 2. ffmpeg 설치

```bash
choco install ffmpeg
```

### 3. 영상에서 오디오 추출

```bash
# whisper는 16kHz, mono, f32 PCM이 필요
ffmpeg -i video.mp4 -ar 16000 -ac 1 -f wav output.wav
```

## 예제 작성

### Cargo.toml에 추가

```toml
[dev-dependencies]
hound = "3"
```

### examples/subtitle.rs 생성

```rust
use std::env;
use whisper_bind_rs::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <model_path> <wav_path>", args[0]);
        std::process::exit(1);
    }

    let model_path = &args[1];
    let wav_path = &args[2];

    // WAV 파일 읽기
    let reader = hound::WavReader::open(wav_path).expect("WAV 파일 열기 실패");
    let spec = reader.spec();
    assert_eq!(spec.channels, 1, "mono 오디오만 지원");
    assert_eq!(spec.sample_rate, 16000, "16kHz 오디오만 지원");

    let samples: Vec<f32> = match spec.sample_format {
        hound::SampleFormat::Int => reader
            .into_samples::<i16>()
            .map(|s| s.unwrap() as f32 / 32768.0)
            .collect(),
        hound::SampleFormat::Float => reader
            .into_samples::<f32>()
            .map(|s| s.unwrap())
            .collect(),
    };

    // 모델 로드
    let params = WhisperContextParams::default();
    let mut ctx = WhisperContext::from_file(model_path, &params)
        .expect("모델 로드 실패");

    // 추론 파라미터 설정
    let mut wparams = WhisperFullParams::new(SamplingStrategy::Greedy);
    wparams
        .set_language("auto")    // 자동 언어 감지
        .set_print_progress(false)
        .set_print_realtime(false)
        .set_print_timestamps(false);

    // 추론 실행
    ctx.full(&wparams, &samples).expect("추론 실패");

    // SRT 형식으로 자막 출력
    let n_segments = ctx.full_n_segments();
    for i in 0..n_segments {
        let t0 = ctx.full_get_segment_t0(i); // 단위: 10ms
        let t1 = ctx.full_get_segment_t1(i);
        let text = ctx.full_get_segment_text(i).unwrap_or("");

        println!("{}", i + 1);
        println!("{} --> {}", format_time(t0), format_time(t1));
        println!("{}", text.trim());
        println!();
    }
}

/// 10ms 단위 → SRT 타임스탬프 (HH:MM:SS,mmm)
fn format_time(t: i64) -> String {
    let ms = (t * 10) % 1000;
    let s = (t * 10 / 1000) % 60;
    let m = (t * 10 / 60000) % 60;
    let h = t * 10 / 3600000;
    format!("{:02}:{:02}:{:02},{:03}", h, m, s, ms)
}
```

## 실행

```bash
cargo run --example subtitle -- models/ggml-base.bin output.wav
```

## 출력 예시 (SRT 형식)

```
1
00:00:00,000 --> 00:00:03,200
 안녕하세요 오늘은 날씨가 좋습니다

2
00:00:03,200 --> 00:00:06,800
 자막 테스트 중입니다
```
