use std::sync::Arc;

use bevy::prelude::*;
use firewheel::{
    channel_config::{ChannelConfig, ChannelCount},
    event::ProcEvents,
    node::{
        AudioNode, AudioNodeInfo, AudioNodeProcessor, ConstructProcessorContext, EmptyConfig,
        ProcBuffers, ProcExtra, ProcInfo, ProcessStatus,
    },
};
use midix::prelude::*;

mod channel_node;

mod plugin;
use midix_synth::prelude::{SoundFont, Synthesizer, SynthesizerSettings};
pub(super) use plugin::plugin;
use trotcast::Receiver;

use crate::{input::FromMidiInputData, synth::SynthCommands};

/// Configuration for the MIDI synthesizer node
#[derive(Debug, Component, TypePath)]
#[require(SynthCommands)]
pub struct MidiSynthNode<C: Clone = ()> {
    sf_recv: crossbeam_channel::Receiver<Arc<SoundFont>>,
    /// Enable reverb and chorus
    pub enable_reverb_and_chorus: bool,
    /// Custom channel data associated with this synthesizer node.
    ///
    /// This field allows attaching application-specific data to the synthesizer,
    /// such as channel routing information or metadata. The type is generic to
    /// support different use cases.
    pub channel: C,
}
impl<C: Clone> Clone for MidiSynthNode<C> {
    fn clone(&self) -> Self {
        Self {
            sf_recv: self.sf_recv.clone(),
            enable_reverb_and_chorus: self.enable_reverb_and_chorus,
            channel: self.channel.clone(),
        }
    }
}

impl MidiSynthNode {
    /// Create a new node with a loaded soundfont and reverb/chorus param
    pub fn new(
        soundfont_channel: crossbeam_channel::Receiver<Arc<SoundFont>>,
        enable_reverb_and_chorus: bool,
    ) -> Self {
        Self {
            sf_recv: soundfont_channel,
            enable_reverb_and_chorus,
            channel: (),
        }
    }
}

//impl

impl AudioNode for MidiSynthNode {
    type Configuration = EmptyConfig;

    fn info(&self, _config: &Self::Configuration) -> AudioNodeInfo {
        AudioNodeInfo::new()
            .debug_name("MIDI Synthesizer")
            .channel_config(ChannelConfig {
                num_inputs: ChannelCount::ZERO,
                num_outputs: ChannelCount::STEREO,
            })
    }

    fn construct_processor(
        &self,
        _config: &Self::Configuration,
        cx: ConstructProcessorContext,
    ) -> impl AudioNodeProcessor {
        MidiSynthProcessor::new(self, cx)
    }
}

/// MIDI synthesizer audio node processor
pub struct MidiSynthProcessor<C = ()> {
    pub(crate) synthesizer: Synthesizer,
    pub(crate) soundfont_channel: crossbeam_channel::Receiver<Arc<SoundFont>>,
    pub(crate) io_channel: C,
}

impl MidiSynthProcessor {
    /// Create a new MIDI synthesizer processor
    pub fn new(config: &MidiSynthNode, cx: ConstructProcessorContext) -> Self {
        let mut settings = SynthesizerSettings::new(cx.stream_info.sample_rate.get() as i32);
        settings.enable_reverb_and_chorus = config.enable_reverb_and_chorus;
        let soundfont_channel = config.sf_recv.clone();
        let soundfont = soundfont_channel.try_recv().unwrap();

        let synthesizer =
            Synthesizer::new(soundfont, &settings).expect("Failed to create synthesizer");

        Self {
            synthesizer,
            soundfont_channel,
            io_channel: (),
        }
    }
}
impl<C> MidiSynthProcessor<C> {
    /// Process a MIDI command
    pub fn process_message(&mut self, command: ChannelVoiceMessage) {
        self.synthesizer.process_midi_message(command);
    }
}

impl<C: IoChannel> AudioNodeProcessor for MidiSynthProcessor<C> {
    fn process(
        &mut self,
        info: &ProcInfo,
        ProcBuffers { outputs, .. }: ProcBuffers,
        events: &mut ProcEvents,
        _extra: &mut ProcExtra,
    ) -> ProcessStatus {
        if let Ok(new_sf) = self.soundfont_channel.try_recv() {
            let mut settings = SynthesizerSettings::new(info.sample_rate.get() as i32);
            settings.enable_reverb_and_chorus = self.synthesizer.get_enable_reverb_and_chorus();

            let synthesizer =
                Synthesizer::new(new_sf, &settings).expect("Failed to create synthesizer");
            self.synthesizer = synthesizer;
            return ProcessStatus::outputs_not_silent();
        }

        // Process incoming MIDI events
        for event in events.drain() {
            if let Some(message) = event.downcast_ref::<ChannelVoiceMessage>() {
                self.process_message(*message);
            }
        }

        // drain our midi data
        while let Some(cvm) = self.io_channel.try_recv() {
            self.process_message(cvm);
        }

        let frames = info.frames;

        // guaranteed to be 2 due to our node's STEREO value.
        let (left, right) = outputs.split_at_mut(1);
        // Render audio from the synthesizer
        self.synthesizer
            .render(&mut left[0][..frames], &mut right[0][..frames]);
        ProcessStatus::outputs_not_silent()
    }
}

pub(crate) trait IoChannel: Send + Sync + 'static {
    fn try_recv(&mut self) -> Option<ChannelVoiceMessage>;
}

impl IoChannel for () {
    fn try_recv(&mut self) -> Option<ChannelVoiceMessage> {
        None
    }
}

impl<D: FromMidiInputData> IoChannel for Receiver<D> {
    fn try_recv(&mut self) -> Option<ChannelVoiceMessage> {
        self.try_recv()
            .ok()
            .and_then(|data| data.to_channel_voice_message())
    }
}
