# 🚀 Velo: The Esoteric Programming Language of Cosmic Navigation

Velo is an experimental, work-in-progress Esoteric programming language (Esolang) centered around the concept of a **Vessel** (the program execution point) navigating a **Cosmos** (the Velo code). Program logic is executed based on the Vessel's speed and direction changes as it impacts various **Runes**.

Velo is named after **Velocity**, the core concept of the execution model.

## 🌌 How Velo Works

### The Cosmos (Velo Code)

The Velo code is represented as a grid, the **Cosmos**. The raw input code is harmonized into an $m \times n$ rectangular grid, where both $m$ and $n$ are forced to be odd numbers by padding with spaces (Void runes) to the right and bottom. This ensures a unique starting point at the center.

### The Vessel (Program State)

The program state is embodied by the `Vessel`, which has:
* **Coordinates** (`x`, `y`)
* **Direction** (`Direction`)
* **Velocity** (`i32`)

The Vessel always starts at the exact center of the Cosmos. Its initial state is determined by the center Rune:
* If the center Rune is a `Thrust` rune (`^`, `v`, `<`, `>`), the Vessel starts with a velocity of 1 and the corresponding direction.
* Otherwise, the Vessel starts with no direction and zero velocity, causing the program to immediately terminate as it has no initial thrust.

### Runes (Instructions)

Runes are characters in the Cosmos that modify the Vessel's state:

| Rune Character | Rune Type | Effect on Vessel |
| :--- | :--- | :--- |
| `^`, `v`, `<`, `>` | `Thrust` | **Same Direction:** Velocity +1. **Opposite Direction:** Velocity -1. **Perpendicular Direction:** Direction changes to the Rune's direction, Velocity is unchanged. |
| `+` | `Boost` | Velocity +1 (Direction unchanged). |
| `-` | `Brake` | Velocity -1 (Direction unchanged). If Velocity drops to 0, the program halts. |
| `*` | `Star` | **Rebound:** Velocity unchanged, Direction reverses 180 degrees. |
| `P` | `Parking` | Velocity becomes 1 (Direction unchanged). |
| Other | `Void` | No effect on Velocity or Direction. |

### Execution Flow

The Vessel continuously moves in its current `Direction` by the amount of 1 unit per cycle.

1.  Calculate the next coordinate based on current `Direction`.
2.  Check for boundary conditions (if out of bounds, program halts: `NoSignal`).
3.  The Vessel moves to the new coordinate and **impacts** the Rune.
4.  The Rune modifies the Vessel's `Direction` and `Velocity`, returning a `Rotation` (Straight, Left, Right, UTurn, Stopped).
5.  **Program Logic (TODO):** The `Rotation` value will be used to execute the underlying Velo instruction logic (e.g., a left turn might execute one type of instruction, and a right turn another).
6.  The loop continues as long as `Velocity > 0`.

## 🛠️ Project Structure

* `src/main.rs`: Handles file loading, cosmos setup, Vessel initialization, and managing program termination.
* `src/lib.rs`: Module exports.
* `src/models.rs`: Defines core data structures: `Rune`, `Cosmos`, `Direction`, `Rotation`, and `Vessel`.
* `src/sail.rs`: Contains the main execution logic (`sail` function), which iterates the Vessel's movement through the Cosmos.

## 🚧 Next Steps (TODO)

The main logical gap is the integration of the Esolang's instruction set:

1.  **Implement Velo Code Logic:** The `Rotation` result in `src/sail.rs` needs to trigger the execution of Velo instructions (e.g., modifying an internal register or tape) based on Left/Right turns.
2.  **Define and Implement State:** Add a program memory/tape/stack to the `Vessel` struct to hold the data the program manipulates.
3.  **Output Mechanism:** Define how the final result of the program is printed or outputted.
