use std::fmt::Display;

mod constants;
#[cfg(test)]
mod tests;

use crate::{
    magics::magic_index, Bitboard, Colour, BETWEEN, BISHOP_ATTACKS, BISHOP_MAGICS, KING_ATTACKS, KNIGHT_ATTACKS, ROOK_ATTACKS, ROOK_MAGICS
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tile(u8);

impl Tile {
    // Constructors
    pub const fn new_unchecked(index: u8) -> Tile {
        Tile(index)
    }

    /// Creates a new tile from a given index, 
    /// returning None if the index is outside the board's range
    pub const fn new_index(index: u8) -> Option<Tile> {
        if index >= 64 {
            return None;
        }
        Some(Tile(index))
    }

    /// Creates a new tile from a given x ( file ) & y ( rank ),
    /// may fail if given out of range coordinates
    pub const fn new_xy(x: u8, y: u8) -> Option<Tile> {
        if x >= 8 || y >= 8 { return None; }
        Some(Tile((y * 8) + x))
    }

    /// Creates a new tile from a string, expecting 2 characters.
    /// Character 1 is the file & 2 is the rank
    pub fn from_str(s: &str) -> Option<Tile> {
        if s.len() != 2 {
            return None;
        }

        let mut chars = s.chars();
        let file = chars.next()?;
        let rank = chars.next()?;

        Tile::new_chars(file, rank)
    }

    pub const fn new_chars(file_char: char, rank_char: char) -> Option<Tile> {
        let file = match file_char {
            'a'..='h' => (file_char as u8) - b'a',
            _ => return None,
        };
        let rank = match rank_char {
            '1'..='8' => (rank_char as u8) - b'1',
            _ => return None,
        };
        
        Some(Tile::new_unchecked((rank << 3) | file))
    }

    pub const fn shift(&self) -> u8 {
        self.0
    }
    pub const fn index(&self) -> usize {
        self.0 as usize
    }
    pub const fn mask(&self) -> Bitboard {
        Bitboard::new(1u64 << self.0)
    }

    pub const fn file(&self) -> u8 {
        self.0 % 8
    }
    pub const fn rank(&self) -> u8 {
        self.0 / 8
    }
    /// (file, rank) or (x, y) using a1 is index 0
    pub fn get_coords(&self) -> (u8, u8) {
        (self.file(), self.rank())
    }

    pub fn offset(&self, dx: i8, dy: i8) -> Option<Self> {
        let x = (self.0 % 8) as i8 + dx;
        let y = (self.0 / 8) as i8 + dy;

        if x < 0 || x > 7 || y < 0 || y > 7 { 
            return None;
        }

        Some(Tile((y * 8 + x) as u8))
    }

    pub fn forward(&self, colour: Colour) -> Option<Self> {
        self.offset(0, if colour.white() { 1 } else { -1 })
    }
    pub fn backward(&self, colour: Colour) -> Option<Self> {
        self.offset(0, if colour.white() { -1 } else { 1 })
    }
    pub fn left(&self, colour: Colour) -> Option<Self> {
        self.offset(if colour.white() { -1 } else { 1 }, 0)
    }
    pub fn right(&self, colour: Colour) -> Option<Self> {
        self.offset(if colour.white() { 1 } else { -1 }, 0)
    }

    pub fn mirror_tile(&self) -> Tile {
        let index = self.0;
        let mirrored_index = index ^ 56;
        Tile::new_unchecked(mirrored_index)
    }

    // Board rules
    pub fn is_promotion(&self, colour: Colour) -> bool {
        let y = self.rank();
        match colour {
            Colour::White => y == 7,
            Colour::Black => y == 0,
        }
    }

    pub fn is_pawn_start(&self, colour: Colour) -> bool {
        let y = self.get_coords().1;
        match colour {
            Colour::White => y == 1,
            Colour::Black => y == 6,
        }
    }
    
    pub fn is_light_square(&self) -> bool {
        self.0 % 2 == 0
    }

    // Attack Generation
    pub fn rook_attacks(&self, occ: Bitboard) -> Bitboard {
        let entry = &ROOK_MAGICS[*self];
        let idx = magic_index(entry, occ);
        ROOK_ATTACKS[idx]
    }

    pub fn bishop_attacks(&self, occ: Bitboard) -> Bitboard {
        let entry = &BISHOP_MAGICS[*self];
        let idx = magic_index(entry, occ);
        BISHOP_ATTACKS[idx]
    }

    pub fn queen_attacks(&self, occ: Bitboard) -> Bitboard {
        self.rook_attacks(occ) | self.bishop_attacks(occ)
    }

    pub fn knight_attacks(&self) -> Bitboard {
        KNIGHT_ATTACKS[*self]
    }

    pub fn king_attacks(&self) -> Bitboard {
        KING_ATTACKS[*self]
    }

    pub fn pawn_attacks(&self, colour: Colour) -> Bitboard {
        let mask = self.mask();
        match colour {
            Colour::White => ((mask << 7) & !Bitboard::FILE_H) | ((mask << 9) & !Bitboard::FILE_A),
            Colour::Black => ((mask >> 7) & !Bitboard::FILE_A) | ((mask >> 9) & !Bitboard::FILE_H),
        }
    }
    
    pub fn between(&self, to: Tile) -> Bitboard {
        BETWEEN[*self][to]
    }
}
impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Tile::TILE_STRS[*self])
    }
}