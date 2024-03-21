use crate::{
    board::{Board, Direction, Ship},
    Bitboard,
};
use std::time::{SystemTime, UNIX_EPOCH};

impl Board {
    const DIRECTIONS: [usize; 2] = [1, Board::WIDTH];

    pub fn random(ships: &Vec<Ship>) -> Self {
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

    pub fn fits(&self, ship: Ship, x: usize, y: usize, direction: Direction) -> Option<Bitboard> {
        match self.get_bitboard(ship, x, y, direction) {
            Some(grid) => {
                if self.misses & grid != Bitboard::EMPTY {
                    return None;
                }

                Some(grid)
            }
            None => None,
        }
    }

    pub fn get_max_density_index(&self) -> Option<usize> {
        self.get_density()
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.cmp(b))
            .map(|(index, _)| index)
    }

    pub fn get_density(&self) -> Vec<usize> {
        let mut probability_density = vec![0usize; Board::WIDTH * Board::HEIGHT];

        for (ship, _) in &self.ships {
            for i in 0..probability_density.len() {
                let (x, y) = (i % Board::WIDTH, i / Board::WIDTH);

                self.density_directions(*ship, x, y, &mut probability_density);
            }
        }

        let mut shoots = self.get_shoots();
        while let Some(index) = shoots.pop_lsb() {
            probability_density[index] = 0;
        }

        probability_density
    }

    fn density_directions(
        &self,
        ship: Ship,
        x: usize,
        y: usize,
        probability_density: &mut Vec<usize>,
    ) {
        Direction::into_iter().for_each(|direction| {
            let grid = self.fits(ship, x, y, direction);

            if let Some(mut grid) = grid {
                while let Some(index) = grid.pop_lsb() {
                    probability_density[index] += 1;
                }
            }
        });

        let index = y * Board::WIDTH + x;
        if Bitboard(1 << index) & self.hits == Bitboard::EMPTY {
            return;
        }

        for direction in Board::DIRECTIONS {
            let index_pos = index.checked_add(direction);
            let index_neg = index.checked_sub(direction);

            match index_pos {
                Some(index) if index < probability_density.len() => {
                    probability_density[index] += 100
                }
                Some(_) => (),
                None => (),
            }

            if let Some(index) = index_neg {
                probability_density[index] += 100
            }
        }
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
