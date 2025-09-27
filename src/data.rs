use bevy::prelude::*;
use midix::{UMicros, events::LiveEvent};

use crate::input::{FromMidiInputData, MidiInput};

/// An [`Event`] for incoming midi data.
#[derive(Message, Debug, Clone)]
pub struct MidiData {
    /// Returns the timestamp of the data
    pub stamp: UMicros,

    /// The underlying message of the event
    pub message: LiveEvent<'static>,
}
#[derive(Clone)]
pub struct MidiDataSettings {
    pub add_channel_event: bool,
}

/// This is the set that will pass [`MidiData`] to the event writer
/// IF `add_channel_event` is configured.
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RecordMidiData;

#[allow(clippy::derivable_impls)]
impl Default for MidiDataSettings {
    fn default() -> Self {
        Self {
            add_channel_event: false,
        }
    }
}

impl FromMidiInputData for MidiData {
    type Settings = MidiDataSettings;
    fn from_midi_data(timestamp: UMicros, event: LiveEvent<'static>) -> Self
    where
        Self: Sized,
    {
        Self {
            stamp: timestamp,
            message: event,
        }
    }

    #[cfg(feature = "synth")]
    fn to_channel_voice(&self) -> Option<midix::prelude::ChannelVoiceMessage> {
        self.message.channel_voice().copied()
    }

    fn configure_plugin(settings: &Self::Settings, app: &mut bevy::app::App) {
        app.configure_sets(Update, RecordMidiData);
        if settings.add_channel_event {
            app.add_message::<MidiData>();

            app.add_systems(Startup, create_recv_channel::<MidiData>)
                .add_systems(Update, write_midi_data::<MidiData>.in_set(RecordMidiData));
        }
    }
}

#[derive(Resource)]
struct RecvChannel<D: FromMidiInputData>(pub trotcast::Receiver<D>);

fn create_recv_channel<D: FromMidiInputData>(mut commands: Commands, input: Res<MidiInput<D>>) {
    let rx = input.channel().spawn_rx();
    commands.insert_resource(RecvChannel(rx));
}

fn write_midi_data<D: FromMidiInputData + Message>(
    mut recv: ResMut<RecvChannel<D>>,
    mut message_writer: MessageWriter<D>,
) {
    while let Ok(msg) = recv.0.try_recv() {
        message_writer.write(msg);
    }
}
