use bevy::prelude::*;

use crate::input::{MidiInput, MidiInputSettings};

#[derive(Default)]
pub struct MidiIoPlugin {
    pub input_setings: MidiInputSettings,
}

impl Plugin for MidiIoPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MidiInput::new(self.input_setings.clone()));
    }
}
