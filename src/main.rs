pub mod monte_carlo;

use battleship::board::Ship;
use monte_carlo::MonteCarlo;

fn main() {
    let monte_carlo = MonteCarlo {
        n: 100_000,
        ships: vec![
            Ship::Carrier,
            Ship::Battleship,
            Ship::Cruiser,
            Ship::Submarine,
            Ship::Destroyer,
        ],
    };
    monte_carlo.run();
}
