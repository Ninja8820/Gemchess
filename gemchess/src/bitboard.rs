use crate::Tile;
use std::fmt;

mod bit_manip;
mod constants;
#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Bitboard(u64);

impl Bitboard {
    pub const EMPTY: Bitboard = Bitboard(0);
    pub const ALL: Bitboard = Bitboard(0xFFFF_FFFF_FFFF_FFFF);

    pub const fn new(bits: u64) -> Self {
        Self(bits)
    }

    pub fn to_bit(&self) -> Option<Tile> {
        if self.count_ones() == 1 {
            Tile::new_index(self.0.trailing_zeros() as u8)
        } else {
            None
        }
    }

    pub fn set_bit(&mut self, bit: Tile, value: bool) {
        let mask = 1u64 << bit.shift();
        if value {
            self.0 |= mask;
        } else {
            self.0 &= !mask;
        }
    }
    pub fn wrapping_mul(&self, value: Bitboard) -> Bitboard {
        Bitboard(self.0.wrapping_mul(value.0))
    }

    pub fn get_bit(&self, bit: Tile) -> bool {
        (self.0 >> bit.shift()) & 1 != 0
    }

    pub fn count_ones(&self) -> u32 {
        self.0.count_ones()
    }

    pub fn some(&self) -> bool {
        self.0 != 0
    }

    pub fn bits(self) -> u64 {
        self.0
    }

    pub fn is_empty(self) -> bool {
        self.0 == 0
    }
}

impl fmt::Display for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for rank in (0..8).rev() {
            for file in 0..8 {
                let idx = rank * 8 + file;
                let bit = (self.0 >> idx) & 1;
                write!(f, "{} ", if bit == 1 { '1' } else { '-' })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl From<Bitboard> for u64 {
    fn from(value: Bitboard) -> Self {
        value.0
    }
}
impl From<u64> for Bitboard {
    fn from(value: u64) -> Self {
        Bitboard(value)
    }
}
impl From<Bitboard> for usize {
    fn from(value: Bitboard) -> Self {
        value.0 as usize
    }
}

impl PartialEq<u64> for Bitboard {
    fn eq(&self, other: &u64) -> bool {
        &self.0 == other
    }
}
