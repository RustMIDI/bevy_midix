use std::marker::PhantomData;

use bevy::prelude::*;

use crate::input::{FromMidiInputData, MidiData, MidiInput, MidiInputSettings};

pub struct MidiIoPlugin<D: FromMidiInputData = MidiData> {
    pub input_setings: MidiInputSettings,
    pub add_channel_event: bool,
    pub(crate) _p: PhantomData<D>,
}

impl<D: FromMidiInputData> MidiIoPlugin<D> {
    pub fn new(input_setings: MidiInputSettings, add_channel_event: bool) -> Self {
        Self {
            input_setings,
            add_channel_event,
            _p: PhantomData,
        }
    }
}

#[allow(clippy::derivable_impls)]
impl Default for MidiIoPlugin {
    fn default() -> Self {
        Self {
            input_setings: Default::default(),
            add_channel_event: false,
            _p: PhantomData,
        }
    }
}

impl<D: FromMidiInputData> Plugin for MidiIoPlugin<D> {
    fn build(&self, app: &mut App) {
        app.insert_resource(MidiInput::<D>::new(self.input_setings.clone()));
        if self.add_channel_event {
            app.add_message::<MidiData>();
            app.add_systems(Startup, create_recv_channel)
                .add_systems(Update, write_midi_data);
        }
    }
}

#[derive(Resource)]
struct RecvChannel(pub trotcast::Receiver<MidiData>);

fn create_recv_channel(mut commands: Commands, input: Res<MidiInput>) {
    let rx = input.channel().spawn_rx();
    commands.insert_resource(RecvChannel(rx));
}

fn write_midi_data(mut recv: ResMut<RecvChannel>, mut message_writer: MessageWriter<MidiData>) {
    while let Ok(msg) = recv.0.try_recv() {
        message_writer.write(msg);
    }
}
