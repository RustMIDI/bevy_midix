use bevy::prelude::*;

mod asset;
pub use asset::*;

#[cfg(feature = "synth")]
mod sound_font;
#[cfg(feature = "synth")]
pub use sound_font::*;

mod song;
pub use song::*;

pub struct MidiAssetsPlugin;

impl Plugin for MidiAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset_loader::<MidiFileLoader>()
            .init_asset::<MidiFile>()
            .register_type::<MidiFile>();

        #[cfg(feature = "synth")]
        app.init_asset_loader::<SoundFontLoader>()
            .init_asset::<SoundFontAsset>();
    }
}
