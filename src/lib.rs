#![doc = r#"
# bevy_MIDIx

Bevy plugin that uses [`midix`](https://crates.io/crates/midix),
[`midir`](https://github.com/Boddlnagg/midir), and a [`midix_synth`](https://crates.io/crates/midix_synth) fork to play midi sounds!

Read from MIDI devices, MIDI files, and programmable input, and output to user audio with a soundfont!

## Features
- Enable `web` for WASM compatibility

## Example
```rust, no_run
use std::time::Duration;
use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
};
use bevy_midix::{midix::prelude::*, prelude::*};
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(LogPlugin {
                level: Level::INFO,
                ..default()
            }),
            MidiPlugin {
                input: None,
                ..Default::default()
            },
        ))
        .add_systems(Startup, load_sf2)
        .add_systems(Update, scale_me)
        .run();
}
/// Take a look here for some soundfonts:
///
/// <https://sites.google.com/site/soundfonts4u/>
fn load_sf2(asset_server: Res<AssetServer>, mut synth: ResMut<Synth>) {
    synth.use_soundfont(asset_server.load("soundfont.sf2"));
}

struct Scale {
    timer: Timer,
    current_key: Key,
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
                self.current_key -= 1;
            } else {
                self.incremented_by += 1;
                self.current_key += 1;
            }
        } else if self.incremented_by == 0 {
            self.forward = true;
            self.incremented_by += 1;
            self.current_key += 1;
        } else {
            self.incremented_by -= 1;
            self.current_key -= 1;
        }
    }
}

impl Default for Scale {
    fn default() -> Self {
        let timer = Timer::new(Duration::from_millis(200), TimerMode::Repeating);
        Scale {
            timer,
            current_key: Key::new(Note::C, Octave::new(2)),
            note_on: true,
            forward: true,
            incremented_by: 0,
            max_increment: 11,
        }
    }
}

fn scale_me(synth: Res<Synth>, time: Res<Time>, mut scale: Local<Scale>) {
    // don't do anything until the soundfont has been loaded
    if !synth.is_ready() {
        return;
    }
    scale.timer.tick(time.delta());
    if !scale.timer.just_finished() {
        return;
    }
    if scale.note_on {
        //play note on
        synth.handle_event(ChannelVoiceMessage::new(
            Channel::One,
            VoiceEvent::note_on(scale.current_key, Velocity::max()),
        ));
    } else {
        //turn off the note
        synth.handle_event(ChannelVoiceMessage::new(
            Channel::One,
            VoiceEvent::note_off(scale.current_key, Velocity::max()),
        ));
        scale.calculate_next_key()
    }

    scale.note_on = !scale.note_on;
}
```

See `/examples` for details.


## Acknowledgment

This crate was originally forked from [`bevy_midi`](https://github.com/BlackPhlox/bevy_midi). Please check them out if this crate doesn't suit your needs!

"#]

use bevy::app::Plugin;

use crate::{
    data::{MidiData, MidiDataSettings},
    input::{FromMidiInputData, MidiInputSettings},
};

pub mod data;
pub mod input;

#[cfg(feature = "assets")]
pub mod assets;
#[cfg(feature = "synth")]
pub mod synth;

pub struct MidiPlugin<D: FromMidiInputData = MidiData> {
    pub input_settings: MidiInputSettings,
    pub data_settings: D::Settings,
}
impl Default for MidiPlugin {
    fn default() -> Self {
        Self {
            input_settings: MidiInputSettings::default(),
            data_settings: MidiDataSettings::default(),
        }
    }
}
impl<D: FromMidiInputData> MidiPlugin<D> {
    pub fn new(input_settings: MidiInputSettings, data_settings: D::Settings) -> Self {
        Self {
            input_settings,
            data_settings,
        }
    }
}

impl<D: FromMidiInputData> Plugin for MidiPlugin<D> {
    fn build(&self, app: &mut bevy::app::App) {
        input::midi_io_plugin_inner::<D>(self.input_settings.clone(), &self.data_settings, app);

        #[cfg(feature = "assets")]
        app.add_plugins(crate::assets::MidiAssetsPlugin);

        #[cfg(feature = "synth")]
        app.add_plugins(crate::synth::SynthPlugin::<D>::new());
    }
}

pub mod prelude {
    pub use crate::input::*;

    #[cfg(feature = "assets")]
    pub use crate::assets::*;

    #[cfg(feature = "synth")]
    pub use crate::synth::*;

    pub use crate::MidiPlugin;

    pub use midix::prelude::*;
}
