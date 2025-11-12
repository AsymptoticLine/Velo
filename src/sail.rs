use crate::models::{Cosmos, Vessel};

/// Defines the reason for the Velo program's execution halt.
pub enum Termination {
    Stopped,                      // Vessel velocity reached zero.
    NoSignal,                     // Vessel traveled out of the Cosmos bounds.
    NoInitialVelocityOrDirection, // Center Rune was not a Thrust rune.
}

/// Runs the Velo program by moving the Vessel through the Cosmos grid.
pub fn sail(cosmos: Cosmos, mut vessel: Vessel) -> Termination {
    let width = cosmos[0].len();
    let height = cosmos.len();

    // Check for initial velocity requirement (must start on a Thrust rune)
    if vessel.velocity() == 0 {
        return Termination::NoInitialVelocityOrDirection;
    }

    // The execution loop: continues as long as the Vessel has speed.
    while vessel.velocity() > 0 {
        if let Ok((x, y)) = vessel.get_next_coordinate() {
            // Check if the next coordinates are within the Cosmos boundaries.
            if x >= width || y >= height {
                return Termination::NoSignal;
            }

            let rune = cosmos[y][x];

            // Update the vessel's position.
            vessel.move_to(x, y);

            /*
            // Debug output: uncomment for tracing the Vessel's movement
            let rotation =
            // */
            vessel.impact_rune(rune);

            /*
            println!(
                "Vessel: {:?}. Rune: {:?}. Rotation: {:?}",
                vessel, rune, rotation
            );
            // */

            // TODO: Implement the Velo code logic based on the rotation (Left/Right turns).
            todo!("Add Velo code logic here.");
        } else {
            // This is primarily an error if `direction` is `None` mid-sail (shouldn't happen)
            // or if a boundary check failed in `get_next_coordinate`.
            return Termination::NoSignal;
        }
    }

    Termination::Stopped
}
