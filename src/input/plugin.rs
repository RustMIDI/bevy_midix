use bevy::prelude::*;

use crate::{
    data::MidiDataSettings,
    input::{FromMidiInputData, MidiData, MidiInput, MidiInputSettings},
};

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
        midi_io_plugin_inner::<D>(self.input_setings.clone(), &self.data_settings, app);
    }
}

pub(crate) fn midi_io_plugin_inner<D: FromMidiInputData>(
    input_settings: MidiInputSettings,
    data_settings: &D::Settings,
    app: &mut App,
) {
    app.insert_resource(MidiInput::<D>::new(input_settings));
    D::configure_plugin(data_settings, app);
}
