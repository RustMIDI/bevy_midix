use std::sync::Arc;

use bevy::prelude::*;
use bevy_seedling::{prelude::AudioEvents, time::Audio};
use midix_synth::prelude::SoundFont;

use crate::{
    assets::SoundFontAsset,
    input::{FromMidiInputData, MidiInput},
    synth::{SynthPlayer, node::MidiSynthNode},
};

pub fn plugin<D: FromMidiInputData>(app: &mut App) {
    app.add_systems(Update, spawn_midi_nodes::<D>)
        .add_systems(PostUpdate, (request_update_for_synth, update_synth));
}

#[derive(Component)]
pub struct UpdateSfChannel(crossbeam_channel::Sender<Arc<SoundFont>>);

/// System that spawns MIDI synthesizer nodes for entities with soundfonts
///
/// once the soundfont has loaded.
fn spawn_midi_nodes<D: FromMidiInputData>(
    mut commands: Commands,
    soundfont_assets: Res<Assets<SoundFontAsset>>,
    query: Query<(Entity, &SynthPlayer), Without<MidiSynthNode>>,
    time: Res<Time<Audio>>,
    midi_io: Res<MidiInput<D>>,
) {
    for (entity, synth_player) in &query {
        // Check if soundfont is loaded
        let Some(soundfont_asset) = soundfont_assets.get(&synth_player.handle) else {
            continue;
        };

        let (tx, rx) = crossbeam_channel::bounded(3);
        tx.send(Arc::clone(soundfont_asset.file())).unwrap();

        // Get config or use defaults
        let mut entity_commands = commands.entity(entity);

        if synth_player.midi_input_enabled {
            let node = MidiSynthNode::new_with_io_channel(rx, true, midi_io.channel().clone());

            // Add the node and its configuration to the entity
            // bevy_seedling will automatically handle node creation and connection
            entity_commands.insert(node);
        } else {
            let node = MidiSynthNode::new(rx, true);

            // Add the node and its configuration to the entity
            // bevy_seedling will automatically handle node creation and connection
            entity_commands.insert(node);
        }
        entity_commands.insert((AudioEvents::new(&time), UpdateSfChannel(tx)));
    }
}

#[derive(Component)]
pub struct NeedsUpdate;
fn request_update_for_synth(
    mut commands: Commands,
    players: Query<Entity, (Changed<SynthPlayer>, Without<NeedsUpdate>)>,
) {
    for player in players {
        commands.entity(player).insert(NeedsUpdate);
    }
}
fn update_synth(
    mut commands: Commands,
    players: Query<(Entity, &SynthPlayer, &UpdateSfChannel), With<NeedsUpdate>>,
    soundfont_assets: Res<Assets<SoundFontAsset>>,
) {
    for (player, synth_player, channel) in players {
        //TODO: we should actually make the soundfont ready to go
        if let Some(soundfont) = soundfont_assets.get(&synth_player.handle)
            && channel.0.try_send(Arc::clone(&soundfont.file)).is_ok()
        {
            commands.entity(player).remove::<NeedsUpdate>();
        }
    }
}
