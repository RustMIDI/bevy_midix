use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    prelude::*,
};

use crate::assets::MidiSong;

use midix::{file::MidiFile, prelude::*, reader::ReaderError};

/// Extension trait for converting MIDI files into playable songs.
///
/// This trait provides convenience methods for working with loaded MIDI files,
/// allowing them to be converted into a format suitable for playback.
pub trait MidiFileExt {
    /// Converts this MIDI file into a MidiSong that can be played by the synthesizer.
    ///
    /// This consumes the MIDI file and extracts all the timing and event information
    /// needed for playback.
    fn into_song(self) -> MidiSong;
}

impl<'a> MidiFileExt for MidiFile<'a> {
    fn into_song(self) -> MidiSong {
        MidiSong::new(
            self.into_events()
                .filter_map(|Timed { timestamp, event }| match event {
                    LiveEvent::ChannelVoice(event) => Some(Timed { timestamp, event }),
                    _ => None,
                })
                .collect(),
        )
    }
}

/// Loader for sound fonts
#[derive(Default)]
pub struct MidiFileLoader;

impl AssetLoader for MidiFileLoader {
    type Asset = MidiFile<'static>;
    type Settings = ();
    type Error = ReaderError;
    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await.unwrap();

        let res = MidiFile::parse(bytes)?;

        Ok(res)
    }

    fn extensions(&self) -> &[&str] {
        &["mid"]
    }
}
