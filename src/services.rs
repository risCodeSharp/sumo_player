use image::{DynamicImage,  ImageReader};
use rodio::{Decoder, OutputStream, Sink};
use std::io::Cursor;

use crate::models::Music;
#[derive(Clone, Copy, Debug)]
pub enum MusicOpenError {
    OpenErr,
    DecoderErr,
    SinkErr,
    // NoTagErr,
    None,
}
pub struct MusicService {
    stream_handle: OutputStream,
    pub music_file: Music,
    sink: Option<Sink>,
}

impl MusicService {
    pub fn new() -> Self {
        let stream_handle =
            rodio::OutputStreamBuilder::open_default_stream().expect("open default audio stream");

        

        Self {
            stream_handle,
            music_file: Music::new("").unwrap(),
            sink: None,
        }
    }

    pub fn open(&mut self, file_path: &str) -> MusicOpenError {
        self.music_file = match Music::new(file_path) {
            Ok(v) => v,
            Err(_e) => {
                return MusicOpenError::OpenErr;
            }
        };

        let reader = match self.music_file.to_reader() {
            Ok(v) => v,
            Err(_) => return MusicOpenError::OpenErr,
        };

        let source = match Decoder::new(reader) {
            Ok(v) => v,
            Err(_e) => {
                return MusicOpenError::DecoderErr;
            }
        };
        let sink = Sink::connect_new(&self.stream_handle.mixer());
        sink.append(source);
        

        self.sink = Some(sink);
        MusicOpenError::None
    }
    pub fn resume(&self) {
        if let Some(sink) = &self.sink {
            sink.play();
        }
    }
    pub fn stop(&self) {
        if let Some(sink) = &self.sink {
            sink.stop();
        }
    }
    pub fn pause(&self) {
        if let Some(sink) = &self.sink {
            sink.pause();
        }
    }

    pub fn decode_image(&self) ->  image::ImageResult<DynamicImage> {
        let bytes = self.music_file.extract_img_bytes()?;
        let bytes: &[u8]  = &bytes;
         ImageReader::new(Cursor::new(bytes))
        .with_guessed_format()?
        .decode()
    }

    pub fn is_music_loaded(&self) -> bool {
        self.sink.is_some()
    }
}
