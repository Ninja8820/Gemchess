use crate::Bitboard;

pub mod bishop_magics;
pub mod rook_magics;

pub mod king_attacks;
pub mod knight_attacks;

pub mod between;
pub mod tilemap;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct MagicEntry {
    pub mask: Bitboard,
    pub magic: Bitboard,
    pub shift: u8,
    pub offset: usize,
}
pub fn magic_index(entry: &MagicEntry, occ: Bitboard) -> usize {
    let blockers = entry.mask & occ;
    let hashed = blockers.wrapping_mul(entry.magic);
    entry.offset + (hashed >> entry.shift).bits() as usize
}
