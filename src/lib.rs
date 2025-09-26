#[cfg(feature = "assets")]
pub mod assets;
pub mod input;
#[cfg(feature = "synth")]
pub mod synth;

pub mod prelude {
    pub use crate::input::*;

    #[cfg(feature = "assets")]
    pub use crate::assets::*;

    #[cfg(feature = "synth")]
    pub use crate::synth::*;
}
