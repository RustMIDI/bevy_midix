#![warn(missing_docs)]
#![warn(clippy::print_stdout)]
#![doc = r#"
# bevy_MIDIx

Bevy plugin that uses [`midix`](https://crates.io/crates/midix),
[`midir`](https://github.com/Boddlnagg/midir), and a [`midix_synth`](https://crates.io/crates/midix_synth) fork to play midi sounds!

Read from MIDI devices, MIDI files, and programmable input, and output to user audio with a soundfont!

## Features
- Enable `web` for WASM compatibility

## Example
```rust, no_run
use bevy::{prelude::*, time::common_conditions::on_timer};
use bevy_midix::prelude::*;
use bevy_seedling::prelude::*;
# use std::time::Duration;
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

/// MIDI input handling and event processing.
///
/// This module provides the core functionality for reading MIDI data from devices
/// and converting it into Bevy-compatible events.
pub mod input;

/// Common implementations of [`FromMidiInputData`]
pub mod data;

/// Contains the [`MidiAssetsPlugin`](crate::assets::MidiAssetsPlugin) and other types.
#[cfg(feature = "assets")]
pub mod assets;

/// Contains the [`SynthPlugin`](crate::synth::SynthPlugin) and other types.
#[cfg(feature = "synth")]
pub mod synth;

/// Main plugin for integrating MIDI functionality into your Bevy application.
///
/// This plugin sets up MIDI input handling and optionally enables asset loading
/// and synthesizer features based on enabled cargo features. The generic parameter
/// `D` allows you to customize how MIDI data is processed - by default it uses
/// `MidiData` which provides standard MIDI event handling.
pub struct MidiPlugin<D: FromMidiInputData = MidiData> {
    /// Configuration for MIDI input devices and connections.
    pub input_settings: MidiInputSettings,
    /// Settings specific to how MIDI data is processed and converted.
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
    /// Creates a new MidiPlugin with the specified input and data processing settings.
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

/// Re-exports commonly used types for convenient importing.
///
/// Use `bevy_midix::prelude::*` to import all the essential types needed
/// for working with MIDI in Bevy.
pub mod prelude {
    pub use crate::input::*;

    #[cfg(feature = "assets")]
    pub use crate::assets::*;

    #[cfg(feature = "synth")]
    pub use crate::synth::*;

    pub use crate::MidiPlugin;

    pub use midix::prelude::*;
}
