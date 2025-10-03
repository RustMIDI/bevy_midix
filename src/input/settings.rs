use bevy::prelude::*;
pub use midir::Ignore;

/// Settings for [`MidiIoPlugin`](crate::prelude::MidiIoPlugin).
#[derive(Resource, Clone, Debug)]
pub struct MidiInputSettings {
    /// The name of the listening client
    pub client_name: String,

    /// The port name of the listening client.
    ///
    /// This is appended to the port name of a connection essentially.
    pub port_name: String,

    /// Describe what events you want to ignore.
    ///
    /// If you don't care about System Exclusive messages
    /// (manufacturer specific messages to their proprietary devices),
    /// set this value to [`Ignore::Sysex`].
    pub ignore: Ignore,

    /// Size of the internal buffer for handling MIDI events.
    ///
    /// This determines how many MIDI events can be queued before processing.
    /// A larger buffer can handle bursts of MIDI data better but uses more memory.
    pub channel_size: usize,
}

impl Default for MidiInputSettings {
    /// Assigns client name and port name to `bevy_midix`
    ///
    /// ignore is set to [`Ignore::None`]
    fn default() -> Self {
        Self {
            client_name: "bevy_midix".to_string(),
            port_name: "bevy_midix".to_string(),
            ignore: Ignore::None,
            channel_size: 60,
        }
    }
}
