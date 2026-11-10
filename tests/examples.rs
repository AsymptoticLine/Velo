use std::fs;
use std::path::{Path, PathBuf};

use velo::interpreter::{Config, Termination, run};
use velo::io::Io;
use velo::{Cosmos, Vessel};

/// A small in-memory [`Io`] so examples can be exercised without stdin/stdout.
struct MockIo {
    input: Vec<u8>,
    cursor: usize,
    output: String,
    diagnostics: String,
}

impl MockIo {
    fn with_input(input: &[u8]) -> Self {
        Self {
            input: input.to_vec(),
            cursor: 0,
            output: String::new(),
            diagnostics: String::new(),
        }
    }
}

impl Io for MockIo {
    fn read_byte(&mut self) -> Option<u8> {
        let byte = self.input.get(self.cursor).copied();
        if byte.is_some() {
            self.cursor += 1;
        }
        byte
    }

    fn write_char(&mut self, character: char) {
        self.output.push(character);
    }

    fn write_diagnostic(&mut self, message: &str) {
        self.diagnostics.push_str(message);
        self.diagnostics.push('\n');
    }
}

fn examples_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("examples")
}

fn execute(name: &str, input: &[u8]) -> (Termination, String) {
    let source = fs::read_to_string(examples_dir().join(name))
        .unwrap_or_else(|error| panic!("failed to read {name}: {error}"));
    let cosmos = Cosmos::from_source(&source);
    let mut vessel = Vessel::new(0, 0, cosmos.get(0, 0));
    let mut io = MockIo::with_input(input);

    let termination = run(&cosmos, &mut vessel, Config::default(), &mut io);

    (termination, io.output)
}

#[test]
fn examples_produce_expected_output() {
    let cases: &[(&str, &[u8], &str)] = &[
        ("function.velo", b"a", "d"),
        ("input.velo", b"a", "b"),
        ("loop_and_print.velo", b"", "*"),
        ("move_pointer.velo", b"", ""),
        ("steer.velo", b"", ""),
        ("swapcase.velo", b"a", "A"),
        ("thrust.velo", b"", ""),
    ];

    for &(name, input, expected) in cases {
        let (termination, output) = execute(name, input);

        assert_eq!(
            termination,
            Termination::Halted,
            "{name} did not halt cleanly"
        );
        assert_eq!(output, expected, "unexpected output from {name}");
    }
}

#[test]
fn swapcase_handles_uppercase_letters() {
    let (termination, output) = execute("swapcase.velo", b"A");

    assert_eq!(termination, Termination::Halted);
    assert_eq!(output, "a");
}

#[test]
fn every_example_has_a_test_case() {
    let mut found: Vec<String> = fs::read_dir(examples_dir())
        .expect("examples directory should exist")
        .map(|entry| entry.expect("readable directory entry").path())
        .filter(|path| {
            path.extension()
                .is_some_and(|extension| extension == "velo")
        })
        .map(|path| {
            path.file_name()
                .expect("example should have a file name")
                .to_string_lossy()
                .into_owned()
        })
        .collect();
    found.sort();

    let mut tested: Vec<String> = [
        "function.velo",
        "input.velo",
        "loop_and_print.velo",
        "move_pointer.velo",
        "steer.velo",
        "swapcase.velo",
        "thrust.velo",
    ]
    .iter()
    .map(|name| (*name).to_owned())
    .collect();
    tested.sort();

    assert_eq!(
        found, tested,
        "add new example programs to `examples_produce_expected_output`"
    );
}
