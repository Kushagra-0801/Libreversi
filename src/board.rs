use crate::position::{Position, MAX_VALID_POS};
use std::ops::Index;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Disc {
    Empty = 0,
    Player1 = 1,
    Player2 = 2,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    p1: [u8; 8],
    p2: [u8; 8],
}

impl Index<Position> for Board {
    type Output = Disc;
    fn index(&self, index: Position) -> &Self::Output {
        if index.idx > MAX_VALID_POS {
            panic!("Index out of bounds")
        }
        let row = (index.idx >> 3) & 0b111;
        let col = index.idx & 0b111;
        let p1 = (self.p1[row as usize] >> col) & 1;
        let p2 = (self.p2[row as usize] >> col) & 1;
        match (p1, p2) {
            (0, 0) => &Disc::Empty,
            (0, 1) => &Disc::Player2,
            (1, 0) => &Disc::Player1,
            _ => unreachable!(),
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        let mut board = Self {
            p1: [0; 8],
            p2: [0; 8],
        };
        board.p1[3] = 0b00010000;
        board.p1[4] = 0b00001000;
        board.p2[3] = 0b00001000;
        board.p2[4] = 0b00010000;
        board
    }
}

impl From<[[Disc; 8]; 8]> for Board {
    fn from(b: [[Disc; 8]; 8]) -> Self {
        let mut board = Board::empty();
        for i in 0..8 {
            for j in 0..8 {
                match b[i][j] {
                    Disc::Player1 => board.p1[i] |= 1 << j,
                    Disc::Player2 => board.p2[i] |= 1 << j,
                    Disc::Empty => (),
                }
            }
        }
        board
    }
}

impl From<[Disc; 64]> for Board {
    fn from(b: [Disc; 64]) -> Self {
        let mut board = Board::empty();
        for i in 0..8 {
            for j in 0..8 {
                match b[i * 8 + j] {
                    Disc::Player1 => board.p1[i] |= 1 << j,
                    Disc::Player2 => board.p2[i] |= 1 << j,
                    Disc::Empty => (),
                }
            }
        }
        board
    }
}

impl Board {
    pub fn empty() -> Self {
        Self {
            p1: [0; 8],
            p2: [0; 8],
        }
    }

    pub fn iter(&self) -> Iter {
        Iter {
            board: self,
            cur_pos: (0, 0).into(),
        }
    }
}

#[derive(Debug)]
pub struct Iter<'a> {
    board: &'a Board,
    cur_pos: Position,
}

impl<'a> Iterator for Iter<'a> {
    type Item = Disc;
    fn next(&mut self) -> Option<Self::Item> {
        if self.cur_pos.idx > MAX_VALID_POS {
            return None;
        }
        let this_one = self.board[self.cur_pos];
        self.cur_pos.idx += 1;
        Some(this_one)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_impl() {
        let board = Board::default();
        assert_eq!(board.p1, [0, 0, 0, 16, 8, 0, 0, 0]);
        assert_eq!(board.p2, [0, 0, 0, 8, 16, 0, 0, 0]);
    }

    #[test]
    fn test_indexing() {
        let board = Board::default();
        assert_eq!(board[(0, 5).into()], Disc::Empty);
        assert_eq!(board[(3, 3).into()], Disc::Player2);
        assert_eq!(board[(3, 4).into()], Disc::Player1);
        assert_eq!(board[(4, 3).into()], Disc::Player1);
        assert_eq!(board[(4, 4).into()], Disc::Player2);
        assert_eq!(board[(7, 7).into()], Disc::Empty);
    }

    #[test]
    fn test_from_disc_array_array_empty() {
        use Disc::Empty;
        let init_board = [[Empty; 8]; 8];
        let new_board = Board::from(init_board);
        assert_eq!(new_board, Board::empty());
    }

    #[test]
    fn test_from_disc_array_array() {
        use Disc::*;
        let init_board = [
            [Empty; 8],
            [Empty; 8],
            [Empty; 8],
            [Empty, Empty, Empty, Player2, Player1, Empty, Empty, Empty],
            [Empty, Empty, Empty, Player1, Player2, Empty, Empty, Empty],
            [Empty; 8],
            [Empty; 8],
            [Empty; 8],
        ];
        let new_board = Board::from(init_board);
        assert_eq!(new_board, Board::default());
    }

    #[test]
    fn test_from_disc_array_empty() {
        use Disc::Empty;
        let init_board = [Empty; 64];
        let new_board = Board::from(init_board);
        assert_eq!(new_board, Board::empty());
    }

    #[test]
    fn test_from_disc_array() {
        use Disc::*;
        let mut init_board = [Empty; 64];
        init_board[27] = Player2;
        init_board[28] = Player1;
        init_board[35] = Player1;
        init_board[36] = Player2;
        let new_board = Board::from(init_board);
        assert_eq!(new_board, Board::default());
    }
}
