mod node;
pub use node::*;

use std::marker::PhantomData;

use firewheel::{diff::EventQueue, event::NodeEventType};

mod player_and_commands;
pub use player_and_commands::*;

use bevy::prelude::*;
use bevy_seedling::prelude::*;
use trotcast::Channel;

use crate::{data::MidiData, input::FromMidiInputData};

pub struct SynthPlugin<D: FromMidiInputData = MidiData> {
    _p: PhantomData<D>,
}

impl Default for SynthPlugin {
    fn default() -> Self {
        Self::new()
    }
}
impl<D: FromMidiInputData> SynthPlugin<D> {
    pub fn new() -> Self {
        Self { _p: PhantomData }
    }
}

#[derive(SystemSet, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct ProcessSynthCommands;

impl<D: FromMidiInputData> Plugin for SynthPlugin<D> {
    fn build(&self, app: &mut App) {
        // Register our custom node type with bevy_seedling
        // Since MidiSynthNode doesn't implement Diff/Patch, we use register_simple_node.
        //
        // This is mainly because we need more from rustysynth than is available.
        app.register_simple_node::<MidiSynthNode>()
            .register_simple_node::<MidiSynthNode<Channel<D>>>();

        app.configure_sets(Update, ProcessSynthCommands);

        app.add_plugins(node::plugin::<D>);

        app.add_systems(Startup, check_for_seedling)
            .add_systems(Update, process_midi_commands.in_set(ProcessSynthCommands));
    }
}
fn check_for_seedling(time: Option<Res<Time<Audio>>>) {
    if time.is_none() {
        panic!(
            "Failed to build `SynthPlugin` in `bevy_midix`:\
            `bevy_seedling`'s TimePlugin was not found. Make sure to add `TimePlugin` *before* `SynthPlugin` or `MidiPlugin.\
            This is usually done by adding `SeedlingPlugin` to your `App`."
        );
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
