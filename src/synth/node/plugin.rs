use std::sync::Arc;

use bevy::prelude::*;
use bevy_seedling::{prelude::AudioEvents, time::Audio};

use crate::{
    assets::SoundFontAsset,
    input::MidiInput,
    synth::{SynthPlayer, node::MidiSynthNode},
};

#[derive(Component)]
struct NodeSpawned;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, spawn_midi_nodes);
}

/// System that spawns MIDI synthesizer nodes for entities with soundfonts
///
/// once the soundfont has loaded.
fn spawn_midi_nodes(
    mut commands: Commands,
    soundfont_assets: Res<Assets<SoundFontAsset>>,
    query: Query<(Entity, &SynthPlayer), Without<NodeSpawned>>,
    time: Res<Time<Audio>>,
    midi_io: Res<MidiInput>,
) {
    for (entity, synth_player) in &query {
        // Check if soundfont is loaded
        let Some(soundfont_asset) = soundfont_assets.get(&synth_player.handle) else {
            continue;
        };

        // Get config or use defaults
        let mut entity_commands = commands.entity(entity);

        if synth_player.midi_input_enabled {
            let node = MidiSynthNode::new_with_channel(
                Arc::clone(soundfont_asset.file()),
                true,
                midi_io.channel().clone(),
            );

            // Add the node and its configuration to the entity
            // bevy_seedling will automatically handle node creation and connection
            entity_commands.insert(node);
        } else {
            let node = MidiSynthNode::new(Arc::clone(soundfont_asset.file()), true);

            // Add the node and its configuration to the entity
            // bevy_seedling will automatically handle node creation and connection
            entity_commands.insert(node);
        }
        entity_commands.insert((AudioEvents::new(&time), NodeSpawned));
    }
}
