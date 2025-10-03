use bevy::prelude::*;

use crate::{
    data::MidiDataSettings,
    input::{FromMidiInputData, MidiData, MidiInput, MidiInputSettings},
};

/// Plugin for managing MIDI input/output operations.
///
/// This plugin handles the low-level MIDI device connections and data routing.
/// It's typically used internally by `MidiPlugin`, but can be used directly if
/// you need more granular control over MIDI I/O without the additional features.
pub struct MidiIoPlugin<D: FromMidiInputData = MidiData> {
    /// Settings for MIDI input device configuration and connection behavior.
    pub input_setings: MidiInputSettings,
    /// Configuration for how raw MIDI data is converted to the type `D`.
    pub data_settings: D::Settings,
}

impl<D: FromMidiInputData> MidiIoPlugin<D> {
    /// Creates a new MidiIoPlugin with specified input and data conversion settings.
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
