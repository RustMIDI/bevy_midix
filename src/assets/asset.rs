use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    prelude::*,
};

use crate::assets::MidiSong;

use midix::{file::MidiFile, prelude::*, reader::ReaderError};

pub trait MidiFileExt {
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
