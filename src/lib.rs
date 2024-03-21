pub mod board;
pub mod density;

use board::Board;
use std::{
    fmt,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Shr},
};

macro_rules! const_assert {
    ($($tt:tt)*) => {
        const _: () = assert!($($tt)*);
    }
}

pub(crate) use const_assert;

#[derive(Copy, Clone, PartialEq)]
pub struct Bitboard(u128);

impl Bitboard {
    pub const EMPTY: Bitboard = Bitboard(0);

    pub fn count_ones(&self) -> u32 {
        self.0.count_ones()
    }

    pub fn get(&self, index: usize) -> bool {
        ((self.0 >> index) & 1) != 0
    }

    pub fn set(&mut self, index: usize, value: bool) {
        assert!(index < 128);

        if value {
            self.0 |= 1u128 << index;
        } else {
            self.0 &= 0u128 << index;
        }
    }

    pub fn pop_lsb(&mut self) -> Option<usize> {
        let i = self.0.trailing_zeros();

        match self.0.checked_sub(1) {
            Some(value) => {
                *self &= Bitboard(value);

                Some(i as usize)
            }
            None => None,
        }
    }

    pub fn iter(&self) -> BitboardIterator {
        BitboardIterator {
            bitboard: self,
            index: 0,
        }
    }
}

impl BitAnd<Bitboard> for Bitboard {
    type Output = Bitboard;

    fn bitand(self, rhs: Bitboard) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl BitOr<Bitboard> for Bitboard {
    type Output = Bitboard;

    fn bitor(self, rhs: Bitboard) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}

impl BitXor<Bitboard> for Bitboard {
    type Output = Bitboard;

    fn bitxor(self, rhs: Bitboard) -> Self::Output {
        Bitboard(self.0 ^ rhs.0)
    }
}

impl Shr<usize> for Bitboard {
    type Output = Bitboard;

    fn shr(self, rhs: usize) -> Self::Output {
        Bitboard(self.0 >> rhs)
    }
}

impl BitAndAssign<Bitboard> for Bitboard {
    fn bitand_assign(&mut self, rhs: Bitboard) {
        self.0 &= rhs.0;
    }
}

impl BitOrAssign<Bitboard> for Bitboard {
    fn bitor_assign(&mut self, rhs: Bitboard) {
        self.0 |= rhs.0;
    }
}

impl BitXorAssign<Bitboard> for Bitboard {
    fn bitxor_assign(&mut self, rhs: Bitboard) {
        self.0 ^= rhs.0;
    }
}

impl fmt::Display for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            get_table(
                self.iter(),
                |square| match square {
                    true => "X".to_string(),
                    false => " ".to_string(),
                },
                Board::WIDTH,
                Board::HEIGHT,
                1
            )
        )
    }
}

pub struct BitboardIterator<'a> {
    bitboard: &'a Bitboard,
    index: usize,
}

impl<'a> Iterator for BitboardIterator<'a> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= Board::WIDTH * Board::HEIGHT {
            return None;
        }

        let result = self.bitboard.get(self.index);

        self.index += 1;
        Some(result)
    }
}

pub fn get_table<I, T, F>(
    iterator: I,
    formatter: F,
    width: usize,
    height: usize,
    size: usize,
) -> String
where
    I: Iterator<Item = T>,
    F: Fn(T) -> String,
{
    let row = format!("\n{}\n", "-".repeat(1 + Board::WIDTH * (3 + size)));

    iterator
        .enumerate()
        .map(|(i, square)| {
            let mut str = String::new();

            if i % width == 0 {
                str.push_str(&row);
                str.push('|');
            }

            str.push_str(&format!(" {} |", formatter(square)));

            if i == width * height - 1 {
                str.push_str(&row);
            }

            str.to_string()
        })
        .collect::<Vec<_>>()
        .join("")
}
