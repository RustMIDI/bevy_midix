use midix::prelude::*;

use super::{SimpleMidiSong, SimpleSection};

/// A struct provided to update the settings of a particular channel for a song
pub struct ChannelModifier<'a> {
    pub(crate) song: &'a mut SimpleMidiSong,
    pub(crate) channel: Channel,
}

impl<'s> ChannelModifier<'s> {
    /// Set the voice for a channel
    pub fn set_voice(&mut self, program: Program) -> &mut Self {
        let preset = self.song.channel_presets.entry(self.channel).or_default();
        preset.program = program;
        self
    }
    /// Set the voice for a channel
    pub fn set_volume(&mut self, volume: Velocity) -> &mut Self {
        let preset = self.song.channel_presets.entry(self.channel).or_default();
        preset.velocity = volume;
        self
    }

    /// Do something with the channel at this beat
    pub fn beat<'b>(&'b mut self, beat_no: u64) -> BeatChannel<'b, 's> {
        BeatChannel {
            channel_mod: self,
            beat: beat_no,
        }
    }
    /// Plays a section with an absolute offset from the start of the song
    pub fn play_section(&mut self, section: &SimpleSection, beat_offset: u64) -> &mut Self {
        for (beat, event) in section.events() {
            let absolute_beat = *beat + beat_offset;
            let velocity = self
                .song
                .channel_presets
                .get(&self.channel)
                .copied()
                .unwrap_or_default()
                .velocity;
            //TODO: look at this
            self.song.add_events(
                absolute_beat,
                event.iter().map(|e| {
                    let event = match e {
                        VoiceEvent::NoteOn { note, .. } => VoiceEvent::NoteOn {
                            note: *note,
                            velocity,
                        },
                        _ => *e,
                    };
                    ChannelVoiceMessage::new(self.channel, event)
                }),
            );
        }
        self
    }
}

/// A struct that will tell a channel to do something at a particular beat
pub struct BeatChannel<'b, 's> {
    channel_mod: &'b mut ChannelModifier<'s>,
    beat: u64,
}

impl<'b, 's> BeatChannel<'b, 's> {
    /// play a note for this channel. Does not override other notes that will be played.
    pub fn play(self, note: Note) -> &'b mut ChannelModifier<'s> {
        let velocity = self
            .channel_mod
            .song
            .channel_presets
            .get(&self.channel_mod.channel)
            .copied()
            .unwrap_or_default()
            .velocity;
        let event = ChannelVoiceMessage::new(
            self.channel_mod.channel,
            VoiceEvent::note_on(note, velocity),
        );

        self.channel_mod.song.add_event(self.beat, event);
        self.channel_mod
    }

    /// play some notes for this channel. Does not override other notes that will be played.
    pub fn play_notes<Notes>(self, notes: Notes) -> &'b mut ChannelModifier<'s>
    where
        Notes: IntoIterator<Item = Note>,
    {
        let events = notes.into_iter().map(|note| {
            ChannelVoiceMessage::new(
                self.channel_mod.channel,
                VoiceEvent::note_on(note, Velocity::MAX),
            )
        });
        self.channel_mod.song.add_events(self.beat, events);
        self.channel_mod
    }
}
