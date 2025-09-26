mod node;

use firewheel::{diff::EventQueue, event::NodeEventType};

mod player_and_commands;
pub use player_and_commands::*;

use bevy::prelude::*;
use bevy_seedling::{prelude::*, time::TimePlugin};
use trotcast::Channel;

use crate::{input::MidiData, synth::node::MidiSynthNode};

pub struct SynthPlugin;

#[derive(SystemSet, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct ProcessSynthCommands;

impl Plugin for SynthPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<TimePlugin>() {
            panic!(
                "Failed to build `SynthPlugin` in `bevy_midix`:\
                `bevy_seedling`'s TimePlugin was not found. Make sure to add `TimePlugin` *before* `SynthPlugin` or `MidiPlugin.\
                This is usually done by adding `SeedlingPlugin` to your `App`."
            );
        }
        // Register our custom node type with bevy_seedling
        // Since MidiSynthNode doesn't implement Diff/Patch, we use register_simple_node.
        //
        // This is mainly because we need more from rustysynth than is available.
        app.register_simple_node::<MidiSynthNode>()
            .register_simple_node::<MidiSynthNode<Channel<MidiData>>>();

        app.configure_sets(Update, ProcessSynthCommands);

        app.add_plugins(node::plugin);

        app.add_systems(Update, process_midi_commands.in_set(ProcessSynthCommands));
    }
}

/// System that processes MIDI commands and sends them to the audio nodes
fn process_midi_commands(mut query: Query<(&FirewheelNode, &mut SynthCommands, &mut AudioEvents)>) {
    for (_, mut commands, mut events) in &mut query {
        if commands.queue.is_empty() {
            continue;
        }

        // Take all pending commands
        let pending = commands.take();

        // Send commands to the audio node as custom events
        for command in pending {
            events.push(NodeEventType::custom(command));
        }
    }
}
