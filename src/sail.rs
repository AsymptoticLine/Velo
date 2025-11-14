use crate::models::{Cosmos, Vessel};

/// Defines the reason for the Velo program's execution halt.
pub enum Termination {
    Stopped,                      // Vessel velocity/pointer reached zero.
    NoSignal,                     // Vessel traveled out of the Cosmos bounds.
    NoInitialVelocityOrDirection, // Start Rune was not a Thrust rune.
}

/// Runs the Velo program by moving the Vessel through the Cosmos grid.
pub fn sail(cosmos: Cosmos, mut vessel: Vessel) -> Termination {
    let width = cosmos[0].len();
    let height = cosmos.len();

    // Check for initial velocity requirement (must start on a Thrust rune)
    if vessel.velocity() == 0 {
        return Termination::NoInitialVelocityOrDirection;
    }

    // The execution loop: continues as long as the Velocity/Pointer is positive.
    while vessel.velocity() > 0 {
        if let Ok((x, y)) = vessel.get_next_coordinate() {
            // Check if the next coordinates are within the Cosmos boundaries.
            if x >= width || y >= height {
                return Termination::NoSignal;
            }

            let rune = cosmos[y][x];

            // Update the vessel's position.
            vessel.move_to(x, y);

            // Impact the Rune and execute the associated instruction/movement.
            vessel.impact_rune(rune);

            /*
            // Debug output: prints the state of the vessel after impacting the rune.
            println!("Vessel: {:?}. Rune: {:?}", vessel, rune);
            // */
        } else {
            // This occurs if a boundary check failed in `get_next_coordinate` (e.g., trying to move from 0 to -1).
            return Termination::NoSignal;
        }
    }

    Termination::Stopped
}
