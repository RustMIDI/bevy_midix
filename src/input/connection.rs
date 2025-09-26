use bevy::prelude::*;
use midir::MidiInputPort;
use midix::events::{FromLiveEventBytes, LiveEvent};
use proj3_core::UMicros;
use trotcast::Channel;

use super::MidiInputError;

/// An [`Event`] for incoming midi data.
#[derive(Event, Debug, Clone)]
pub struct MidiData {
    /// Returns the timestamp of the data    /// Something happened when refreshing the port statuses
    pub stamp: UMicros,

    /// The underlying message of the event
    pub message: LiveEvent<'static>,
}

pub(crate) struct MidiInputConnectionHandler {
    conn: midir::MidiInputConnection<()>,
}

impl MidiInputConnectionHandler {
    pub fn new(
        midir_input: midir::MidiInput,
        port: &MidiInputPort,
        port_name: &str,
        sender: Channel<MidiData>,
    ) -> Result<Self, MidiInputError> {
        let conn = midir_input.connect(
            port,
            port_name,
            {
                move |timestamp, data, _| {
                    let Ok(message) = LiveEvent::from_bytes(data) else {
                        return;
                    };
                    if let Err(e) = sender.send(MidiData {
                        stamp: UMicros::new(timestamp),
                        message,
                    }) {
                        warn!("Error sending metadata! {e:?}");
                    }
                }
            },
            (),
        )?;

        Ok(Self { conn })
    }

    pub fn close(self) -> midir::MidiInput {
        let (listener, _) = self.conn.close();
        listener
    }
}
