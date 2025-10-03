use bevy::{prelude::*, time::common_conditions::on_timer};
use bevy_midix::prelude::*;
use bevy_seedling::prelude::*;
use std::time::Duration;
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            SeedlingPlugin::default(),
            MidiPlugin::default(),
        ))
        .add_systems(Startup, load_sf2)
        .add_systems(
            Update,
            scale_me
                .run_if(on_timer(Duration::from_millis(200)))
                .before(ProcessSynthCommands),
        )
        .run();
}
/// Take a look here for some soundfonts:
///
/// <https://sites.google.com/site/soundfonts4u/>
fn load_sf2(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(SynthPlayer::new(assets.load("soundfont.sf2"), false));
}

struct Scale {
    current_note: Note,
    note_on: bool,
    forward: bool,
    incremented_by: u8,
    max_increment: u8,
}

impl Scale {
    pub fn calculate_next_key(&mut self) {
        if self.forward {
            if self.incremented_by == self.max_increment {
                self.forward = false;
                self.incremented_by -= 1;
                self.current_note -= 1;
            } else {
                self.incremented_by += 1;
                self.current_note += 1;
            }
        } else if self.incremented_by == 0 {
            self.forward = true;
            self.incremented_by += 1;
            self.current_note += 1;
        } else {
            self.incremented_by -= 1;
            self.current_note -= 1;
        }
    }
}

impl Default for Scale {
    fn default() -> Self {
        Scale {
            current_note: Note::new(Key::C, Octave::new(2)),
            note_on: true,
            forward: true,
            incremented_by: 0,
            max_increment: 11,
        }
    }
}

fn scale_me(mut synth: Single<&mut SynthCommands>, mut scale: Local<Scale>) {
    const VEL: Velocity = Velocity::new_unchecked(67);

    if scale.note_on {
        //play note on
        synth.send(ChannelVoiceMessage::new(
            Channel::One,
            VoiceEvent::note_on(scale.current_note, VEL),
        ));
    } else {
        //turn off the note
        synth.send(ChannelVoiceMessage::new(
            Channel::One,
            VoiceEvent::note_off(scale.current_note, VEL),
        ));
        scale.calculate_next_key()
    }

    scale.note_on = !scale.note_on;
}
