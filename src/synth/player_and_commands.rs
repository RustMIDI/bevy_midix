use bevy::prelude::*;
use midix::prelude::*;

use crate::assets::SoundFontAsset;

/// Component that specifies which soundfont to use for a MIDI synth
#[derive(Component)]
pub struct SynthPlayer {
    pub(crate) handle: Handle<SoundFontAsset>,
    pub(crate) midi_input_enabled: bool,
}

impl SynthPlayer {
    pub fn new(handle: Handle<SoundFontAsset>, midi_input_enabled: bool) -> Self {
        Self {
            handle,
            midi_input_enabled,
        }
    }

    pub fn handle(&self) -> &Handle<SoundFontAsset> {
        &self.handle
    }
}

/// Component for sending MIDI commands to a synthesizer node via ECS.
///
/// It is not required to have this (i.e. you are using a SynthPlayer<Channel>)
#[derive(Component, Default)]
pub struct SynthCommands {
    /// Queue of MIDI commands to send
    pub queue: Vec<ChannelVoiceMessage>,
}

impl SynthCommands {
    /// Add a MIDI command to the queue
    pub fn send(&mut self, command: ChannelVoiceMessage) {
        self.queue.push(command);
    }

    /// Add multiple MIDI commands to the queue
    pub fn send_batch(&mut self, commands: impl IntoIterator<Item = ChannelVoiceMessage>) {
        self.queue.extend(commands);
    }

    /// Take all commands, leaving the queue empty
    pub fn take(&mut self) -> Vec<ChannelVoiceMessage> {
        std::mem::take(&mut self.queue)
    }
}
