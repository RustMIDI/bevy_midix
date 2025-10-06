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
    input::FromMidiInputData,
    synth::node::{MidiSynthNode, MidiSynthProcessor},
};

impl<D: FromMidiInputData> MidiSynthNode<Channel<D>> {
    /// Create a new node with a loaded soundfont and reverb/chorus param
    pub(crate) fn new_with_io_channel(
        soundfont_channel: crossbeam_channel::Receiver<Arc<SoundFont>>,
        enable_reverb_and_chorus: bool,
        channel: Channel<D>,
    ) -> Self {
        Self {
            sf_recv: soundfont_channel,
            enable_reverb_and_chorus,
            channel,
        }
    }
}

impl<D: FromMidiInputData> AudioNode for MidiSynthNode<Channel<D>> {
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

impl<D: FromMidiInputData> MidiSynthProcessor<Receiver<D>> {
    /// Create a new MIDI synthesizer processor
    pub fn new_with_channel(
        config: &MidiSynthNode<Channel<D>>,
        cx: ConstructProcessorContext,
    ) -> Self {
        let mut settings = SynthesizerSettings::new(cx.stream_info.sample_rate.get() as i32);
        settings.enable_reverb_and_chorus = config.enable_reverb_and_chorus;

        let soundfont_channel = config.sf_recv.clone();
        let soundfont = soundfont_channel.try_recv().unwrap();

        let synthesizer =
            Synthesizer::new(soundfont, &settings).expect("Failed to create synthesizer");

        Self {
            soundfont_channel,
            synthesizer,
            io_channel: config.channel.spawn_rx(),
        }
    }
}
