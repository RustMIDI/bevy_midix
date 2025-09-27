use std::sync::Arc;

use bevy_seedling::prelude::ChannelCount;
use firewheel::{
    channel_config::ChannelConfig,
    event::ProcEvents,
    node::{
        AudioNode, AudioNodeInfo, AudioNodeProcessor, ConstructProcessorContext, EmptyConfig,
        ProcBuffers, ProcExtra, ProcInfo, ProcessStatus,
    },
};
use midix::prelude::ChannelVoiceMessage;
use midix_synth::prelude::{SoundFont, Synthesizer, SynthesizerSettings};
use trotcast::{Channel, Receiver};

use crate::{
    input::data::MidiData,
    synth::node::{MidiSynthNode, MidiSynthProcessor},
};

impl MidiSynthNode<Channel<MidiData>> {
    /// Create a new node with a loaded soundfont and reverb/chorus param
    pub(crate) fn new_with_channel(
        soundfont: Arc<SoundFont>,
        enable_reverb_and_chorus: bool,
        channel: Channel<MidiData>,
    ) -> Self {
        Self {
            soundfont,
            enable_reverb_and_chorus,
            channel,
        }
    }
}

impl AudioNode for MidiSynthNode<Channel<MidiData>> {
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
        MidiSynthProcessor::new_with_channel(self, cx)
    }
}

impl MidiSynthProcessor<Receiver<MidiData>> {
    /// Create a new MIDI synthesizer processor
    pub fn new_with_channel(
        config: &MidiSynthNode<Channel<MidiData>>,
        cx: ConstructProcessorContext,
    ) -> Self {
        let mut settings = SynthesizerSettings::new(cx.stream_info.sample_rate.get() as i32);
        settings.enable_reverb_and_chorus = config.enable_reverb_and_chorus;

        let synthesizer = Synthesizer::new(config.soundfont.clone(), &settings)
            .expect("Failed to create synthesizer");

        Self {
            synthesizer,
            channel: config.channel.spawn_rx(),
        }
    }
}

impl AudioNodeProcessor for MidiSynthProcessor<Receiver<MidiData>> {
    fn process(
        &mut self,
        info: &ProcInfo,
        ProcBuffers { outputs, .. }: ProcBuffers,
        events: &mut ProcEvents,
        _extra: &mut ProcExtra,
    ) -> ProcessStatus {
        // Process other incoming MIDI events
        for event in events.drain() {
            if let Some(message) = event.downcast_ref::<ChannelVoiceMessage>() {
                self.process_message(*message);
            }
        }

        // drain our midi data
        while let Ok(data) = self.channel.try_recv() {
            if let Some(cvm) = data.message.channel_voice() {
                self.process_message(*cvm);
            }
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
