use crate::{Board, MoveList};

impl Board {
    pub fn positions(&mut self, depth: usize) -> i64 {
        if depth == 0 {
            return 1;
        }

        let mut moves = MoveList::new();
        self.generate_legal_moves(self.turn, &mut moves);

        if depth == 1 {
            return moves.len() as i64;
        }

        let mut positions = 0;
        for &m in moves.iter() {
            self.make_move_unchecked(m);
            positions += self.positions(depth - 1);
            self.undo_move();
        }

        positions
    }
    pub fn positions_divide(&mut self, depth: usize) -> i64 {
        if depth == 0 {
            return 1;
        }

        let mut moves = MoveList::new();
        self.generate_legal_moves(self.turn, &mut moves);

        if depth == 1 {
            return moves.len() as i64;
        }

        let mut positions = 0;
        for &m in moves.iter() {
            self.make_move_unchecked(m);
            positions += self.positions(depth - 1);
            println!("{}, {}", self.move_to_san(&m), &positions);
            self.undo_move();
        }

        positions
    }
}
