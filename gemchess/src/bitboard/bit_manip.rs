use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, Shr};

use crate::{Bitboard, Tile};

// AND ( & )
impl BitAnd for Bitboard {
    type Output = Bitboard;
    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard::new(self.0 & rhs.0)
    }
}

impl BitAndAssign for Bitboard {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

// OR ( | )
impl BitOr for Bitboard {
    type Output = Self;
    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard::new(self.0 | rhs.0)
    }
}

impl BitOrAssign for Bitboard {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

// XOR ( ^ )
impl BitXor for Bitboard {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Bitboard::new(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0
    }
}

// NOT ( ! )
impl Not for Bitboard {
    type Output = Self;
    #[inline(always)]
    fn not(self) -> Self::Output {
        Bitboard::new(!self.0)
    }
}

// SHIFTS ( << | >> )
impl Shl<u8> for Bitboard {
    type Output = Self;
    #[inline(always)]
    fn shl(self, rhs: u8) -> Self::Output {
        Bitboard::new(self.0 << rhs)
    }
}

impl Shr<u8> for Bitboard {
    type Output = Self;
    #[inline(always)]
    fn shr(self, rhs: u8) -> Self::Output {
        Bitboard::new(self.0 >> rhs)
    }
}

impl Shl<Tile> for Bitboard {
    type Output = Self;
    #[inline(always)]
    fn shl(self, rhs: Tile) -> Self::Output {
        Bitboard::new(self.0 << rhs.shift())
    }
}

impl Shr<Tile> for Bitboard {
    type Output = Self;
    #[inline(always)]
    fn shr(self, rhs: Tile) -> Self::Output {
        Bitboard::new(self.0 >> rhs.shift())
    }
}

impl Iterator for Bitboard {
    type Item = Tile;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            None
        } else {
            let s = self.0.trailing_zeros() as u8;
            self.0 &= self.0 - 1;
            Tile::new_index(s)
        }
    }
}
