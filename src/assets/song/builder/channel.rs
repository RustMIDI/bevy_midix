use midix::prelude::*;

use super::MidiSongBuilder;

/// A struct provided to play a bunch of commands for one channel.
///
/// All time is in microseconds.
pub struct ChannelBuilder<'a> {
    pub(crate) builder: &'a mut MidiSongBuilder,
    pub(crate) channel: Channel,
}

impl ChannelBuilder<'_> {
    /// Set a program change (new voice)
    pub fn program_change(&mut self, time: u64, program: Program) -> &mut Self {
        self.builder.add(Timed::new(
            time,
            ChannelVoiceMessage::new(self.channel, VoiceEvent::program_change(program)),
        ));
        self
    }
    /// Turn a note on
    pub fn note_on(&mut self, time: u64, note: Note, velocity: Velocity) -> &mut Self {
        self.builder.add(Timed::new(
            time,
            ChannelVoiceMessage::new(self.channel, VoiceEvent::note_on(note, velocity)),
        ));
        self
    }

    /// Turn a note off
    pub fn note_off(&mut self, time: u64, note: Note, velocity: Velocity) -> &mut Self {
        self.builder.add(Timed::new(
            time,
            ChannelVoiceMessage::new(self.channel, VoiceEvent::note_off(note, velocity)),
        ));
        self
    }
    /// Modify the velocity of a note after it's been played.
    pub fn after_touch(&mut self, time: u64, note: Note, velocity: Velocity) -> &mut Self {
        self.builder.add(Timed::new(
            time,
            ChannelVoiceMessage::new(self.channel, VoiceEvent::after_touch(note, velocity)),
        ));
        self
    }

    ///Modify the controller's presets
    pub fn control_change(&mut self, time: u64, controller: Controller) -> &mut Self {
        self.builder.add(Timed::new(
            time,
            ChannelVoiceMessage::new(self.channel, VoiceEvent::control_change(controller)),
        ));
        self
    }

    /// Change the note velocity of a whole channel at once without starting new notes
    pub fn channel_after_touch(&mut self, time: u64, velocity: Velocity) -> &mut Self {
        self.builder.add(Timed::new(
            time,
            ChannelVoiceMessage::new(self.channel, VoiceEvent::channel_after_touch(velocity)),
        ));
        self
    }
    /// Bend the pitch
    pub fn pitch_bend(&mut self, time: u64, pitch_bend: PitchBend) -> &mut Self {
        self.builder.add(Timed::new(
            time,
            ChannelVoiceMessage::new(self.channel, VoiceEvent::pitch_bend(pitch_bend)),
        ));
        self
    }
}
