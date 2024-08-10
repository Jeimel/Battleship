pub mod simulation;

use std::{env, process::exit};

use battleship::board::Ship;
use simulation::Simulation;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <number of games>", args[0]);
        exit(1)
    }

    let n: usize = args.get(1).and_then(|n| n.parse().ok()).unwrap_or_else(|| {
        println!("Can't parse to usize: {}", args[1]);
        exit(1)
    });

    let sim = Simulation {
        n,
        ships: vec![
            Ship::Carrier,
            Ship::Battleship,
            Ship::Cruiser,
            Ship::Submarine,
            Ship::Destroyer,
        ],
    };
    sim.run();
}
