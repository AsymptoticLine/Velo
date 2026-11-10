//! The input/output boundary used by the interpreter.

use std::io::{self, Read, Write};

/// Input and output used while a Velo program runs.
///
/// Implement this trait to run the interpreter against something other than
/// the process's standard streams, such as a test buffer.
pub trait Io {
    /// Reads one byte, or returns `None` at end of input.
    fn read_byte(&mut self) -> Option<u8>;

    /// Writes one character of program output.
    fn write_char(&mut self, character: char);

    /// Writes a diagnostic message (debug output, trace output, or warnings).
    fn write_diagnostic(&mut self, message: &str);

    /// Flushes buffered output, if any.
    fn flush(&mut self) {}
}

/// Standard input/output implementation of [`Io`].
pub struct StdIo {
    stdin: io::StdinLock<'static>,
    stdout: io::StdoutLock<'static>,
}

impl StdIo {
    pub fn new() -> Self {
        Self {
            stdin: io::stdin().lock(),
            stdout: io::stdout().lock(),
        }
    }
}

impl Default for StdIo {
    fn default() -> Self {
        Self::new()
    }
}

impl Io for StdIo {
    fn read_byte(&mut self) -> Option<u8> {
        let mut buffer = [0_u8; 1];

        match self.stdin.read_exact(&mut buffer) {
            Ok(()) => Some(buffer[0]),
            Err(_) => None,
        }
    }

    fn write_char(&mut self, character: char) {
        let mut buffer = [0_u8; 4];
        // A closed stdout should not crash the interpreter.
        let _ = self
            .stdout
            .write_all(character.encode_utf8(&mut buffer).as_bytes());
    }

    fn write_diagnostic(&mut self, message: &str) {
        eprintln!("{message}");
    }

    fn flush(&mut self) {
        let _ = self.stdout.flush();
    }
}

impl Drop for StdIo {
    fn drop(&mut self) {
        let _ = self.stdout.flush();
    }
}
