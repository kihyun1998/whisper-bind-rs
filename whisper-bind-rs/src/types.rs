pub type WhisperToken = i32;

/// Sampling strategy for the decoder
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SamplingStrategy {
    Greedy,
    BeamSearch,
}

impl SamplingStrategy {
    pub(crate) fn to_raw(self) -> whisper_sys::whisper_sampling_strategy {
        match self {
            SamplingStrategy::Greedy => {
                whisper_sys::whisper_sampling_strategy_WHISPER_SAMPLING_GREEDY
            }
            SamplingStrategy::BeamSearch => {
                whisper_sys::whisper_sampling_strategy_WHISPER_SAMPLING_BEAM_SEARCH
            }
        }
    }
}

/// Token data returned from whisper_full_get_token_data
#[derive(Debug, Clone, Copy)]
pub struct TokenData {
    pub id: WhisperToken,
    pub tid: WhisperToken,
    pub p: f32,
    pub plog: f32,
    pub pt: f32,
    pub ptsum: f32,
    pub t0: i64,
    pub t1: i64,
    pub t_dtw: i64,
    pub vlen: f32,
}

impl From<whisper_sys::whisper_token_data> for TokenData {
    fn from(raw: whisper_sys::whisper_token_data) -> Self {
        TokenData {
            id: raw.id,
            tid: raw.tid,
            p: raw.p,
            plog: raw.plog,
            pt: raw.pt,
            ptsum: raw.ptsum,
            t0: raw.t0,
            t1: raw.t1,
            t_dtw: raw.t_dtw,
            vlen: raw.vlen,
        }
    }
}

/// Performance timing information
#[derive(Debug, Clone, Copy)]
pub struct Timings {
    pub sample_ms: f32,
    pub encode_ms: f32,
    pub decode_ms: f32,
    pub batchd_ms: f32,
    pub prompt_ms: f32,
}

/// Trait for custom model loaders.
///
/// Implement this trait to load whisper models from custom sources
/// (e.g. network streams, encrypted files, embedded resources).
pub trait ModelLoader {
    /// Read `output.len()` bytes into `output`. Returns the number of bytes actually read.
    fn read(&mut self, output: &mut [u8]) -> usize;
    /// Return true if the end of the data source has been reached.
    fn eof(&mut self) -> bool;
    /// Close the data source and release any resources.
    fn close(&mut self);
}

/// Helper to create a C-compatible `whisper_model_loader` from a Rust `ModelLoader`.
///
/// The returned struct takes ownership of the loader. The `close` callback will
/// free the heap-allocated trait object.
pub(crate) fn make_c_model_loader(loader: Box<dyn ModelLoader>) -> whisper_sys::whisper_model_loader {
    // Double-box: outer Box is on heap, inner Box<dyn> is the trait object
    let ctx = Box::into_raw(Box::new(loader)) as *mut std::ffi::c_void;

    unsafe extern "C" fn read_cb(
        ctx: *mut std::ffi::c_void,
        output: *mut std::ffi::c_void,
        read_size: usize,
    ) -> usize {
        unsafe {
            let loader = &mut *(ctx as *mut Box<dyn ModelLoader>);
            let slice = std::slice::from_raw_parts_mut(output as *mut u8, read_size);
            loader.read(slice)
        }
    }

    unsafe extern "C" fn eof_cb(ctx: *mut std::ffi::c_void) -> bool {
        unsafe {
            let loader = &mut *(ctx as *mut Box<dyn ModelLoader>);
            loader.eof()
        }
    }

    unsafe extern "C" fn close_cb(ctx: *mut std::ffi::c_void) {
        unsafe {
            let mut loader = Box::from_raw(ctx as *mut Box<dyn ModelLoader>);
            loader.close();
        }
    }

    whisper_sys::whisper_model_loader {
        context: ctx,
        read: Some(read_cb),
        eof: Some(eof_cb),
        close: Some(close_cb),
    }
}

/// A recognized text segment with timing and metadata
#[derive(Debug, Clone)]
pub struct Segment {
    pub t0: i64,
    pub t1: i64,
    pub text: String,
    pub no_speech_prob: f32,
    pub speaker_turn_next: bool,
}
