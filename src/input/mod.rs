use bevy::prelude::*;

mod settings;
use midix::{UMicros, events::LiveEvent, prelude::ChannelVoiceMessage};
pub use settings::*;

mod error;
pub use error::*;

mod state;

mod plugin;
pub use plugin::*;

use midir::MidiInputPort;
use trotcast::prelude::*;

use crate::{
    data::MidiData,
    input::state::{MidiInputConnectionHandler, MidiInputState},
};

/// Trait for converting raw MIDI input events into custom data types.
///
/// Implement this trait to define how MIDI events should be transformed into
/// your application-specific data structures. The default implementation `MidiData`
/// provides standard MIDI event handling, but you can create custom types for
/// specialized processing.
pub trait FromMidiInputData: Send + Sync + Clone + 'static {
    /// Configuration type for customizing the MIDI data conversion process.
    ///
    /// This allows you to pass settings that affect how MIDI events are
    /// interpreted and converted into your custom type.
    type Settings: Send + Sync;

    /// Converts a raw MIDI event with timestamp into your custom data type.
    ///
    /// This method is called for each incoming MIDI event. The timestamp
    /// indicates when the event occurred in microseconds.
    fn from_midi_data(timestamp: UMicros, event: LiveEvent<'static>) -> Self;

    #[cfg(feature = "synth")]
    /// Attempts to extract a channel voice message from this MIDI data.
    ///
    /// Returns `Some` if this data represents a channel voice message (like
    /// note on/off, pitch bend, etc.), or `None` if it represents other
    /// types of MIDI data. This is primarily used by the synth module.
    fn to_channel_voice_message(&self) -> Option<ChannelVoiceMessage>;

    /// You can use this to configure stuff for your type in bevy,
    ///
    /// but you don't necessarily need to do this. It's useful for
    /// the default [`MidiData`] message as it derives event.
    #[allow(unused_variables)]
    fn configure_plugin(settings: &Self::Settings, app: &mut App) {}
}

/// The central resource for interacting with midi inputs
///
/// `MidiInput` does many things:
/// - Fetches a list of ports with connected midi devices
/// - Allows one to connect to a particular midi device and read output
/// - Close that connection and search for other devices
#[derive(Resource)]
pub struct MidiInput<D: FromMidiInputData = MidiData> {
    channel: Channel<D>,
    state: Option<MidiInputState>,
    ports: Vec<MidiInputPort>,
    client_name: String,
    port_name: String,
    ignore: Ignore,
}

impl<D: FromMidiInputData> MidiInput<D> {
    /// Creates a new midi input with the provided settings. This is done automatically
    /// by [`MidiIoPlugin`].
    pub fn new(settings: MidiInputSettings) -> Self {
        let mut listener = match midir::MidiInput::new(&settings.client_name) {
            Ok(input) => input,
            Err(e) => {
                panic!("Error initializing midi input! {e:?}");
            }
        };

        listener.ignore(settings.ignore);

        let ports = listener.ports();
        Self {
            channel: Channel::new(settings.channel_size),
            state: Some(MidiInputState::Listening(listener)),
            client_name: settings.client_name,
            port_name: settings.port_name,
            ignore: settings.ignore,
            ports,
        }
    }

    /// The channel use to send and receive midi data
    pub fn channel(&self) -> &Channel<D> {
        &self.channel
    }

    /// Return a list of ports updated since calling [`MidiInput::new`] or
    /// [`MidiInput::refresh_ports`]
    pub fn ports(&self) -> &[MidiInputPort] {
        &self.ports
    }
    /// Attempts to connects to the port at the given index returned by [`MidiInput::ports`]
    ///
    /// # Errors
    /// - If already connected to a device
    /// - If the index is out of bounds
    /// - An input connection cannot be established
    pub fn connect_to_index(&mut self, index: usize) -> Result<(), MidiInputError> {
        if self
            .state
            .as_ref()
            .is_none_or(|s| matches!(s, MidiInputState::Active(_)))
        {
            return Err(MidiInputError::invalid(
                "Cannot connect: not currently active!",
            ));
        }
        let Some(port) = self.ports.get(index) else {
            return Err(MidiInputError::port_not_found(
                "Port was not found at {index}!",
            ));
        };

        let MidiInputState::Listening(listener) = self.state.take().unwrap() else {
            unreachable!()
        };
        let handler =
            MidiInputConnectionHandler::new(listener, port, &self.port_name, self.channel.clone())
                .unwrap();

        self.state = Some(MidiInputState::Active(handler));
        Ok(())
    }

    /// A method you should call if [`MidiInput::is_listening`] and [`MidiInput::is_active`] are both false.
    pub fn reset(&mut self) {
        let mut listener = match midir::MidiInput::new(&self.client_name) {
            Ok(input) => input,
            Err(e) => {
                error!("Failed to reset listening state! {e:?}");
                return;
            }
        };
        listener.ignore(self.ignore);
        self.state = Some(MidiInputState::Listening(listener));
    }
    /// Attempts to connects to the passed port
    ///
    /// # Errors
    /// - If already connected to a device
    /// - An input connection cannot be established
    pub fn connect_to_port(&mut self, port: &MidiInputPort) -> Result<(), MidiInputError> {
        if self
            .state
            .as_ref()
            .is_none_or(|s| matches!(s, MidiInputState::Active(_)))
        {
            return Err(MidiInputError::invalid(
                "Cannot connect: not currently active!",
            ));
        }
        let MidiInputState::Listening(listener) = self.state.take().unwrap() else {
            unreachable!()
        };

        self.state = Some(MidiInputState::Active(
            MidiInputConnectionHandler::new(listener, port, &self.port_name, self.channel.clone())
                .unwrap(),
        ));
        Ok(())
    }

    /// Attempts to connects to the passed port
    ///
    /// # Errors
    /// - If already connected to a device
    /// - If the port ID cannot be currently found
    ///   - Note that this case can occur if you have not refreshed ports
    ///     and the device is no longer available.
    /// - An input connection cannot be established
    pub fn connect_to_id(&mut self, id: String) -> Result<(), MidiInputError> {
        if self
            .state
            .as_ref()
            .is_none_or(|s| matches!(s, MidiInputState::Active(_)))
        {
            return Err(MidiInputError::invalid(
                "Cannot connect: not currently active!",
            ));
        }
        let MidiInputState::Listening(listener) = self.state.take().unwrap() else {
            unreachable!()
        };
        let Some(port) = listener.find_port_by_id(id.clone()) else {
            return Err(MidiInputError::port_not_found(id));
        };
        self.state = Some(MidiInputState::Active(
            MidiInputConnectionHandler::new(listener, &port, &self.port_name, self.channel.clone())
                .unwrap(),
        ));
        Ok(())
    }

    /// True if a device is currently connected
    pub fn is_active(&self) -> bool {
        self.state
            .as_ref()
            .is_some_and(|s| matches!(s, MidiInputState::Active(_)))
    }

    /// True if input is waiting to connect to a device
    pub fn is_listening(&self) -> bool {
        self.state
            .as_ref()
            .is_some_and(|s| matches!(s, MidiInputState::Listening(_)))
    }

    /// Refreshes the available port list
    ///
    /// Does nothing if [`MidiInput::is_active`] is true
    pub fn refresh_ports(&mut self) {
        let Some(MidiInputState::Listening(listener)) = &self.state else {
            return;
        };
        self.ports = listener.ports();
    }

    /// Disconnects from the active device
    ///
    /// Does nothing if the [`MidiInput::is_listening`] is true.
    pub fn disconnect(&mut self) {
        if self
            .state
            .as_ref()
            .is_none_or(|s| matches!(s, MidiInputState::Listening(_)))
        {
            return;
        }
        let MidiInputState::Active(conn) = self.state.take().unwrap() else {
            unreachable!()
        };
        let listener = conn.close();
        self.state = Some(MidiInputState::Listening(listener));
    }
}
