use battleship::board::{Board, Ship};

pub struct MonteCarlo {
    pub n: usize,
    pub ships: Vec<Ship>,
}

impl MonteCarlo {
    pub fn run(&self) {
        let mut results = vec![0usize; Board::WIDTH * Board::HEIGHT];

        for _ in 0..self.n {
            let mut board = Board::random(&self.ships);

            for result in &mut results {
                let max_index = board.get_max_density_index().unwrap();
                board.shoot(max_index % Board::WIDTH, max_index / Board::WIDTH);

                if board.is_over() {
                    *result += 1;

                    break;
                }
            }
        }

        let mut sum = 0;
        for value in &mut results {
            sum += *value;

            *value = sum;
        }

        println!("{:?}", results);
    }
}
