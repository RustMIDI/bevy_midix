use bevy::ecs::message::Message;
use midix::{UMicros, events::LiveEvent};

/// An [`Event`] for incoming midi data.
#[derive(Message, Debug, Clone)]
pub struct MidiData {
    /// Returns the timestamp of the data
    pub stamp: UMicros,

    /// The underlying message of the event
    pub message: LiveEvent<'static>,
}
