//! Velo: an esoteric programming language in which a vessel sails through a
//! cosmos of runes.
//!
//! The crate exposes the same pieces the command-line interpreter uses:
//! [`Cosmos`] parses source code, [`Vessel`] holds the program state, and
//! [`interpreter::run`] executes a program.

pub mod cosmos;
pub mod direction;
pub mod interpreter;
pub mod io;
pub mod rune;
pub mod vessel;

pub use cosmos::Cosmos;
pub use direction::{Direction, Rotation};
pub use interpreter::{Config, Termination, run};
pub use io::{Io, StdIo};
pub use rune::Rune;
pub use vessel::Vessel;
