mod connection;
pub(crate) use connection::*;

// you can't actually have multiple MidiInputs on one device, it's really strange.
pub enum MidiInputState {
    Listening(midir::MidiInput),
    Active(MidiInputConnectionHandler),
}

/// SAFETY: This applies to linux alsa.
///
/// There is only one instance of MidiInput at any time using this crate.
///
/// However, this may not satisfy the requirements for safety. If another instance of
/// MidiInput exists in the external program, then UB is possible.
///
/// Therefore, the assumption is, that when using this crate, that the user
/// will NOT instantiate another [`midir::MidiInput`] at any point while
/// [`MidiInput`] has been inserted as a resource
unsafe impl Sync for MidiInputState {}
unsafe impl Send for MidiInputState {}
