use bevy::app::Plugin;

use crate::input::{FromMidiInputData, MidiData};

pub mod input;

#[cfg(feature = "assets")]
pub mod assets;
#[cfg(feature = "synth")]
pub mod synth;

#[derive(Default)]
pub struct MidiPlugin<D: FromMidiInputData = MidiData> {
    input_settings: crate::input::MidiInputSettings,
    data_settings: D::Settings,
}

impl<D: FromMidiInputData> Plugin for MidiPlugin<D> {
    fn build(&self, app: &mut bevy::app::App) {
        input::midi_io_plugin_inner::<D>(self.input_settings.clone(), &self.data_settings, app);

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

    pub use crate::MidiPlugin;

    pub use midix::prelude::*;
}
