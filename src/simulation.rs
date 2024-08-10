use battleship::board::{Board, Direction, Ship};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Simulation {
    pub n: usize,
    pub ships: Vec<Ship>,
}

impl Simulation {
    pub fn run(&self) {
        let mut results = vec![0usize; Board::WIDTH * Board::HEIGHT];

        for _ in 0..self.n {
            let mut board = Simulation::random(&self.ships);

            for result in &mut results {
                let max_index = board.max_density_index().unwrap();
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

    pub fn random(ships: &Vec<Ship>) -> Board {
        let mut board = Board::default();

        let mut rand = Rand::default();
        for ship in ships {
            loop {
                let (x, y) = (
                    (rand.random() * Board::WIDTH as f64) as usize,
                    (rand.random() * Board::HEIGHT as f64) as usize,
                );

                let direction = match rand.random_raw() & 1 {
                    0 => Direction::Horizontal,
                    _ => Direction::Vertical,
                };

                if board.place(*ship, x, y, direction) {
                    break;
                }
            }
        }

        board
    }
}

struct Rand {
    pub seed: u128,
    multiplier: u128,
}

impl Default for Rand {
    fn default() -> Self {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();

        Self {
            seed: nanos | 1,
            multiplier: 0xF1C47040DE494ACC251D055F00F0A1AB,
        }
    }
}

impl Rand {
    pub fn random(&mut self) -> f64 {
        self.random_raw() as f64 / u64::MAX as f64
    }

    pub fn random_raw(&mut self) -> u64 {
        self.seed = self.seed.wrapping_mul(self.multiplier);
        let rot = (self.seed >> 122) as u32;
        let xsl = (self.seed >> 64) as u64 ^ self.seed as u64;
        xsl.rotate_right(rot)
    }
}
