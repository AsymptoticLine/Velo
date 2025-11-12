use crate::models::{Cosmos, Vessel};

pub enum Termination {
    Stopped,
    NoSignal,
}

pub fn sail(cosmos: Cosmos, mut vessel: Vessel) -> Termination {
    let width = cosmos[0].len();
    let height = cosmos.len();
    while vessel.vel() > 0 {
        if let Ok((x, y)) = vessel.get_next_coordinate() {
            if x >= width || y >= height {
                return Termination::NoSignal;
            }

            let rune = cosmos[y][x];

            vessel.move_to(x, y);
            let rotation = vessel.meet_rune(rune);

            println!(
                "Vessel: {:?}. Rune: {:?}. Rotation: {:?}",
                vessel, rune, rotation
            );

            todo!("Add Velo code logic here.");
        } else {
            return Termination::NoSignal;
        }
    }

    Termination::Stopped
}
