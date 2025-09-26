use bevy::app::Plugin;

pub mod input;

#[cfg(feature = "assets")]
pub mod assets;
#[cfg(feature = "synth")]
pub mod synth;

#[derive(Default)]
pub struct MidiPlugin {
    input_settings: crate::input::MidiInputSettings,
}

impl Plugin for MidiPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins(crate::input::MidiIoPlugin {
            input_setings: self.input_settings.clone(),
        });
        #[cfg(feature = "assets")]
        app.add_plugins(crate::assets::MidiAssetsPlugin);

        #[cfg(feature = "synth")]
        app.add_plugins(crate::synth::SynthPlugin);
    }
}

pub mod prelude {
    pub use crate::input::*;

    #[cfg(feature = "assets")]
    pub use crate::assets::*;

    #[cfg(feature = "synth")]
    pub use crate::synth::*;
}
