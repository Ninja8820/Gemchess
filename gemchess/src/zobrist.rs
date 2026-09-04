use crate::{Bitboard, Board, CastlingRights, Colour, Piece};

pub mod consts;
use consts::{CASTLING, EN_PASSANT, PIECE_SQUARE, SIDE_TO_MOVE};
impl Board {
    pub fn to_zobrist_hash(&self) -> Bitboard {
        let mut hash: Bitboard = Bitboard::EMPTY;

        for p in Piece::ALL_PIECES {
            let z_white = p.to_zobrist_index(Colour::White);
            let z_black = p.to_zobrist_index(Colour::Black);
            for t in self.white.bb[p as usize] {
                hash ^= PIECE_SQUARE[z_white][t];
            }
            for t in self.black.bb[p as usize] {
                hash ^= PIECE_SQUARE[z_black][t];
            }
        }

        if self.turn == Colour::White {
            hash ^= SIDE_TO_MOVE;
        }

        if self.castling.contains(CastlingRights::WHITE_KINGSIDE) {
            hash ^= CASTLING[0];
        }
        if self.castling.contains(CastlingRights::WHITE_QUEENSIDE) {
            hash ^= CASTLING[1];
        }
        if self.castling.contains(CastlingRights::BLACK_KINGSIDE) {
            hash ^= CASTLING[2];
        }
        if self.castling.contains(CastlingRights::BLACK_QUEENSIDE) {
            hash ^= CASTLING[3];
        }

        if let Some(tile) = self.en_passant {
            let (player, _) = self.current_players();
            for t in player.bb[Piece::Pawn as usize] {
                if t.pawn_attacks(self.turn).get_bit(tile) {
                    let (file, _) = tile.get_coords();

                    hash ^= EN_PASSANT[file as usize];
                    break;
                }
            }
        }

        hash
    }
}
