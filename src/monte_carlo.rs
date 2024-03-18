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

            for i in 0..Board::WIDTH * Board::HEIGHT {
                let max_index = board.get_max_density_index().unwrap();
                board.shoot(max_index % Board::WIDTH, max_index / Board::WIDTH);

                if board.is_over() {
                    results[i] += 1;

                    break;
                }
            }
        }

        let mut sum = 0;
        for i in 0..results.len() {
            sum += results[i];

            results[i] = sum;
        }

        println!("{:?}", results);
        println!("{}", results[44]);
    }
}
