use bevy::ecs::message::Message;
use midix::{UMicros, events::LiveEvent};

use crate::input::FromMidiInputData;

/// An [`Event`] for incoming midi data.
#[derive(Message, Debug, Clone)]
pub struct MidiData {
    /// Returns the timestamp of the data
    pub stamp: UMicros,

    /// The underlying message of the event
    pub message: LiveEvent<'static>,
}

impl FromMidiInputData for MidiData {
    fn from_midi_data(timestamp: UMicros, event: LiveEvent<'static>) -> Self
    where
        Self: Sized,
    {
        Self {
            stamp: timestamp,
            message: event,
        }
    }
}
