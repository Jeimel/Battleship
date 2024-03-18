use std::fmt;

use crate::Bitboard;

crate::const_assert!(Board::WIDTH * Board::HEIGHT < 128);

pub struct Board {
    pub grid: Bitboard,
    pub hits: Bitboard,
    pub misses: Bitboard,
    pub ships: Vec<(Ship, Bitboard)>,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            grid: Bitboard(0),
            hits: Bitboard(0),
            misses: Bitboard(0),
            ships: Vec::new(),
        }
    }
}

impl Board {
    pub const WIDTH: usize = 10;
    pub const HEIGHT: usize = 10;

    pub fn place(&mut self, ship: Ship, x: usize, y: usize, direction: Direction) -> bool {
        match self.get_bitboard(ship, x, y, direction) {
            Some(grid) => {
                if self.grid & grid != Bitboard::EMPTY {
                    return false;
                }

                self.grid |= grid;
                self.ships.push((ship, grid));

                true
            }
            None => false,
        }
    }

    pub fn shoot(&mut self, x: usize, y: usize) -> Hit {
        assert!(x < Board::WIDTH && y < Board::HEIGHT);

        let grid = Bitboard(1 << y * Board::WIDTH + x);

        for i in 0..self.ships.len() {
            let board = self.ships[i].1;

            if grid & board == Bitboard::EMPTY {
                continue;
            }

            self.hits |= grid;

            if (self.get_shoots() & board).count_ones() == board.count_ones() {
                self.grid ^= board;
                self.hits ^= board;
                self.misses |= board;

                self.ships.swap_remove(i);
                return Hit::Kill;
            }

            return Hit::Hit;
        }

        self.misses |= grid;
        Hit::Miss
    }

    pub fn is_over(&self) -> bool {
        self.ships.is_empty()
    }

    pub fn get_bitboard(
        &self,
        ship: Ship,
        x: usize,
        y: usize,
        direction: Direction,
    ) -> Option<Bitboard> {
        if !self.in_bound(&ship, x, y, &direction) {
            return None;
        }

        let mut grid = Bitboard(0);
        for i in 0..ship.value() {
            match direction {
                Direction::Vertical => grid.set((y + i) * Board::WIDTH + x, true),
                Direction::Horizontal => grid.set(y * Board::WIDTH + x + i, true),
            }
        }

        Some(grid)
    }

    pub fn get_shoots(&self) -> Bitboard {
        self.hits | self.misses
    }

    fn in_bound(&self, ship: &Ship, x: usize, y: usize, direction: &Direction) -> bool {
        match direction {
            Direction::Vertical => x < Board::WIDTH && y + ship.value() <= Board::HEIGHT,
            Direction::Horizontal => y < Board::HEIGHT && x + ship.value() <= Board::WIDTH,
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.grid)
    }
}

#[derive(Clone, Copy)]
pub enum Ship {
    Carrier,
    Battleship,
    Cruiser,
    Submarine,
    Destroyer,
}

impl Ship {
    fn value(&self) -> usize {
        match *self {
            Ship::Carrier => 5,
            Ship::Battleship => 4,
            Ship::Cruiser => 3,
            Ship::Submarine => 3,
            Ship::Destroyer => 2,
        }
    }
}

pub enum Hit {
    Miss,
    Hit,
    Kill,
}

pub enum Direction {
    Vertical,
    Horizontal,
}

impl Direction {
    pub fn into_iter() -> core::array::IntoIter<Direction, 2> {
        [Direction::Vertical, Direction::Horizontal].into_iter()
    }
}
