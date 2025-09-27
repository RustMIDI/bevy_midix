use bevy::prelude::*;
use midir::MidiInputPort;
use midix::{
    UMicros,
    events::{FromLiveEventBytes, LiveEvent},
};
use trotcast::Channel;

use crate::input::{FromMidiInputData, MidiInputError};

pub(crate) struct MidiInputConnectionHandler {
    conn: midir::MidiInputConnection<()>,
}

impl MidiInputConnectionHandler {
    pub fn new<D: FromMidiInputData>(
        midir_input: midir::MidiInput,
        port: &MidiInputPort,
        port_name: &str,
        sender: Channel<D>,
    ) -> Result<Self, MidiInputError> {
        let conn = midir_input.connect(
            port,
            port_name,
            {
                move |timestamp, data, _| {
                    let Ok(message) = LiveEvent::from_bytes(data) else {
                        return;
                    };
                    if let Err(e) = sender.send(D::from_midi_data(UMicros::new(timestamp), message))
                    {
                        warn!("Error sending MIDI data! {e:?}");
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
