use bevy::prelude::*;

mod asset;
pub use asset::*;

#[cfg(feature = "synth")]
mod sound_font;
use midix::file::MidiFile;
#[cfg(feature = "synth")]
pub use sound_font::*;

mod song;
pub use song::*;

/// Plugin for loading and managing MIDI-related assets.
///
/// This plugin enables loading MIDI files and soundfont files as Bevy assets.
/// It registers the necessary asset loaders and types so you can load these
/// files using Bevy's asset server, making it easy to manage MIDI songs and
/// instrument soundfonts as game resources.
pub struct MidiAssetsPlugin;

impl Plugin for MidiAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset_loader::<MidiFileLoader>()
            .init_asset::<MidiFile<'static>>()
            .register_type::<MidiFile<'static>>();

        #[cfg(feature = "synth")]
        app.init_asset_loader::<SoundFontLoader>()
            .init_asset::<SoundFontAsset>();
    }
}
