# 🚀 Velo: The Esoteric Programming Language of Cosmic Velocity

Velo is a **Turing-complete** esoteric programming language (esolang) in which a **Vessel** sails through a **Cosmos**. Execution is modeled as motion: a 2D grid of runes steers a vessel whose velocity doubles as the data pointer and as the program's halt condition.

## 🌌 Cosmology and Execution

### The Cosmos (the code)

Velo source is a 2D grid of **runes**. Execution starts at the top-left corner `(0, 0)`. Text after `#` on a line is a comment. Rows do not need to have the same length; cells outside a row behave as void.

### The Vessel (program state)

- **Velocity** (`usize`) — the data pointer and cosmic resonance frequency. The program halts when it reaches `0`.
- **Data lattice** (`Vec<u32>`) — expandable data cells, indexed by velocity.
- **Entropy level** (`u32`) — the value of the cell currently selected by velocity.
- **Heading** — up, down, left, or right, supplied initially by the top-left rune.

### Execution

Each cycle the vessel moves exactly one cell in its current heading, then impacts the rune at its new position. The rune may change the vessel's heading, velocity, or entropy level.

## 🔠 Rune Set

| Runes   | Name               | Effect                                                                                                                                                                 |
| :------ | :----------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `^v<>`  | **Thrust**         | Thrusting in the current direction increases velocity by 1; thrusting in the opposite direction decreases it by 1; thrusting perpendicular turns the vessel. Velocity never drops below 0. |
| `P`     | **Parking**        | Resets velocity (the data pointer) to `1`.                                                                                                                             |
| `+`     | **Entropy Charge** | Increases the current cell's entropy level by 1.                                                                                                                       |
| `-`     | **Entropy Drain**  | Decreases the current cell's entropy level by 1, if it is at least 1.                                                                                                  |
| `[`     | **Steer Left**     | If the current cell's entropy level is non-zero, turns the vessel 90° left.                                                                                            |
| `]`     | **Steer Right**    | If the current cell's entropy level is non-zero, turns the vessel 90° right.                                                                                           |
| `,`     | **Input**          | Reads one byte from stdin into the current cell. End of input stores `0`.                                                                                              |
| `.`     | **Output**         | Writes the current cell's entropy level as a character.                                                                                                                |
| `D`     | **Debug**          | With `--debug`, prints the vessel state. Otherwise has no effect.                                                                                                      |
| *other* | **Void**           | No effect.                                                                                                                                                             |

The top-left rune must be a thrust rune: it determines the initial heading and gives the vessel velocity `1`.

## ⚙️ Command Line Interface

```bash
cargo run --release -- <file.velo> [OPTIONS]
```

| Flag            | Name        | Function                                                                                 |
| :-------------- | :---------- | :--------------------------------------------------------------------------------------- |
| `-d`, `--debug` | Debug mode  | Prints the vessel state only when a `D` (Debug) rune is encountered.                     |
| `-t`, `--trace` | Trace mode  | Prints the vessel state at every execution step. Overrides `--debug`.                    |
| `--ignore-void` | Ignore void | Used with `--trace`; skips printing the state when the vessel impacts a void rune.       |

Debug and trace output is written to stderr, so it stays separate from program output.

## 🛑 Termination

A program succeeds when the vessel's velocity reaches `0`. It fails if:

1. The vessel attempts to travel out of the cosmos.
2. The top-left rune is not a thrust rune, so the vessel has no initial heading or velocity.

## 📦 Examples

The [`examples/`](examples) directory contains runnable programs:

| Program                 | Demonstrates                                      | Stdin       | Output |
| :---------------------- | :------------------------------------------------ | :---------- | :----- |
| `thrust.velo`           | Thrust runes, acceleration, and turning           | —           | —      |
| `move_pointer.velo`     | Moving the data pointer, parking, and steering    | —           | —      |
| `loop_and_print.velo`   | Loops and output                                  | —           | `*`    |
| `steer.velo`            | Conditional steering                              | —           | —      |
| `input.velo`            | Reading a byte, incrementing it, and printing it  | one byte    | input + 1 |
| `function.velo`         | Function-like control flow                        | one byte    | input + 3 |
| `swapcase.velo`         | Swapping letter case with helper routines         | one letter  | case-swapped letter |

```bash
# No input needed
cargo run --release -- examples/loop_and_print.velo

# These read one byte (u8) from stdin
printf 'a' | cargo run --release -- examples/input.velo    # b
printf 'a' | cargo run --release -- examples/function.velo # d
printf 'a' | cargo run --release -- examples/swapcase.velo # A
```

On Windows PowerShell, use `"a" | cargo run --release -- examples/input.velo` instead of `printf`.

## 🧪 Development

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

`tests/examples.rs` runs every program in `examples/` through the interpreter with scripted stdin and checks its output, so new examples should be added there too.

### Project layout

```text
src/
  cli.rs          command-line arguments and program loading
  cosmos.rs       source parser and rune grid
  direction.rs    Direction and Rotation
  interpreter.rs  execution loop, Config, Termination
  io.rs           Io trait and standard-stream implementation
  rune.rs         instruction set and per-rune behavior
  vessel.rs       vessel state and data lattice
examples/         sample Velo programs
tests/examples.rs integration tests for the examples
```

## 📄 License

Released under the [MIT No Attribution (MIT-0)](LICENSE) license.

