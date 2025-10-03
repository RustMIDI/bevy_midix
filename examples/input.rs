use bevy::prelude::*;
use bevy_midix::{
    data::{MidiData, MidiDataSettings},
    prelude::*,
};
use trotcast::Receiver;
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            MidiPlugin::<MidiData>::new(
                MidiInputSettings::default(),
                MidiDataSettings {
                    // this is false by default.
                    add_channel_event: true,
                },
                false,
            ),
        ))
        .add_systems(Startup, create_receiver)
        .add_systems(
            Update,
            (connect_to_first_input, read_messages, read_from_channel),
        )
        .run();
}
#[derive(Resource)]
pub struct InputRecv(Receiver<MidiData>);
fn create_receiver(mut commands: Commands, input: Res<MidiInput>) {
    commands.insert_resource(InputRecv(input.channel().spawn_rx()));
}

fn connect_to_first_input(mut input: ResMut<MidiInput>) {
    let Some(ports) = input.refresh_ports() else {
        return;
    };
    if let Some(first) = ports.first().cloned() {
        info!("Connecting to {}", first.id());
        _ = input
            .connect_to_port(&first)
            .inspect_err(|e| error!("{e:?}"));
    }
}

fn read_messages(mut messages: MessageReader<MidiData>) {
    for msg in messages.read() {
        info!("From Message Reader: {msg:?}");
    }
}
fn read_from_channel(mut input: ResMut<InputRecv>) {
    while let Ok(msg) = input.0.try_recv() {
        info!("From Channel: {msg:?}");
    }
}
