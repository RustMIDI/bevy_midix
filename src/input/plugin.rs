use bevy::prelude::*;

use crate::input::{FromMidiInputData, MidiData, MidiDataSettings, MidiInput, MidiInputSettings};

pub struct MidiIoPlugin<D: FromMidiInputData = MidiData> {
    pub input_setings: MidiInputSettings,
    pub data_settings: D::Settings,
}

impl<D: FromMidiInputData> MidiIoPlugin<D> {
    pub fn new(input_setings: MidiInputSettings, data_settings: D::Settings) -> Self {
        Self {
            input_setings,
            data_settings,
        }
    }
}

#[allow(clippy::derivable_impls)]
impl Default for MidiIoPlugin {
    fn default() -> Self {
        Self {
            input_setings: Default::default(),
            data_settings: MidiDataSettings::default(),
        }
    }
}

impl<D: FromMidiInputData> Plugin for MidiIoPlugin<D> {
    fn build(&self, app: &mut App) {
        app.insert_resource(MidiInput::<D>::new(self.input_setings.clone()));
    }
}
