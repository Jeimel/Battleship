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

    /*
    let mut board = Board::default();
    board.place(Ship::Battleship, 5, 6, Direction::Horizontal);
    board.place(Ship::Carrier, 0, 0, Direction::Vertical);
    board.place(Ship::Cruiser, 3, 3, Direction::Vertical);
    board.place(Ship::Submarine, 0, 9, Direction::Horizontal);
    board.place(Ship::Destroyer, 9, 0, Direction::Vertical);

    println!("{}", board);

    board.shoot(5, 5);
    board.shoot(4, 4);
    board.shoot(6, 6);

    for i in 0..2 {
        let max_index = board.get_max_density_index().unwrap();
        println!(
            "{}",
            get_table(
                board.get_density().iter(),
                |x| format!("{:02}", x),
                Board::WIDTH,
                Board::HEIGHT,
                2
            )
        );

        match board.shoot(max_index % Board::WIDTH, max_index / Board::WIDTH) {
            Hit::Miss => println!("Miss"),
            Hit::Hit => println!("Hit"),
            Hit::Kill => println!("Kill"),
        };
        println!("{max_index}");
        println!(
            "{i}: {} {}",
            max_index / Board::WIDTH,
            max_index % Board::WIDTH
        );
    }
    */
}
