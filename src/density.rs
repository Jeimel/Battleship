use crate::{
    board::{Board, Direction, Ship},
    Bitboard, Rand,
};

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

            match index_neg {
                Some(index) => probability_density[index] += 100,
                None => (),
            }
        }
    }
}
