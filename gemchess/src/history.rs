use crate::{Bitboard, Move, SanMove};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct History {
    pub last_move: Move,
    pub last_zobrist: Bitboard,
    pub san_string: SanMove,
}
impl History
{
    pub fn new(mov: Move, hash: Bitboard, san: SanMove) -> Self
    {
        Self { 
            last_move: mov, 
            last_zobrist: hash, 
            san_string: san 
        }
    }
}