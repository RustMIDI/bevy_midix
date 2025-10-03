use bevy::prelude::*;
use bevy_midix::prelude::*;
use bevy_seedling::SeedlingPlugin;
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            SeedlingPlugin::default(),
            MidiPlugin::default(),
        ))
        .add_systems(Startup, create_receiver)
        .add_systems(Update, connect_to_first_input)
        .run();
}
fn create_receiver(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(SynthPlayer::new(assets.load("soundfont.sf2"), true));
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
