# 🚀 Velo: The Esoteric Programming Language of Cosmic Velocity

Velo is a **Turing-complete** Esolang where program execution is modeled as a **Vessel** navigating a **Cosmos**. The program's data and control flow are entirely determined by the Vessel's **Velocity** and **Entropy Level**.

The name Velo emphasizes **Velocity**, which serves a dual purpose:

1.  **Physical Speed**: The actual rate of acceleration/deceleration.
2.  **Program Pointer**: The value of Velocity acts as the **Cosmic Resonance Frequency**, indexing the data memory.

## 🌌 Velo Cosmology and Execution

### The Cosmos (The Code)

Velo code is a 2D grid of Runes. It is harmonized into an $m \times n$ rectangle by padding with spaces (Void Runes). The Vessel always starts at the top left corner.

### The Vessel (Program State)

The core state is stored within the Vessel:

- **Velocity** (`usize`): The data pointer/frequency. The program halts if this value reaches 0.
- **Data Lattice** (`Vec<i32>`): The expandable memory structure (Data Cells).
- **Entropy Level** (`i32`): The value of the data cell currently pointed to by the Velocity.

### Execution

The Vessel moves one unit per cycle based on its current direction. Program logic is executed when the Vessel impacts a Rune, modifying its Velocity, Direction, or the Entropy Level.

## 🔠 Rune Set (Instructions)

Runes are grouped by their primary effect:

| Rune   | Name               | Velo Field Affected | Effect                                                                                                                  |
| :----- | :----------------- | :------------------ | :---------------------------------------------------------------------------------------------------------------------- |
| `^v<>` | **Thrust Runes**   | Direction, Velocity | Modifies Velocity (+1, -1, or no change) and/or Direction based on the alignment of the Rune and the current direction. |
| `P`    | **Parking**        | Velocity            | Resets Velocity (Pointer) to 1.                                                                                         |
| `*`    | **Star**           | Direction           | Reverses the Vessel's direction.                                                                                        |
| `+`    | **Entropy Charge** | Entropy Level       | Increases the current cell's Entropy Level by 1.                                                                        |
| `-`    | **Entropy Drain**  | Entropy Level       | Decreases the current cell's Entropy Level by 1.                                                                        |
| `[`    | **Steer Left**     | Direction           | **Conditional Loop:** If Entropy Level $\neq 0$, forces a 90° left turn, continuing the loop.                           |
| `]`    | **Steer Right**    | Direction           | **Conditional Loop:** If Entropy Level $\neq 0$, forces a 90° right turn, redirecting the Vessel.                       |
| `,`    | **Input**          | Entropy Level       | Reads a byte from stdin into the current cell.                                                                          |
| `.`    | **Output**         | N/A                 | Prints the current cell's Entropy Level as an ASCII character.                                                          |

## 🛑 Termination

The Velo program halts if:

1.  The **Velocity** (Pointer) reaches **0**.
2.  The Vessel attempts to travel **out of the Cosmos boundaries** (NoSignal).
3.  The Vessel starts on a Rune that is **not a Thrust Rune** (NoInitialVelocityOrDirection).
