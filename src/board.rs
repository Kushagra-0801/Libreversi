use crate::position::{Position, MAX_VALID_POS};
use std::ops::Index;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Disc {
    Empty = 0,
    Player1 = 1,
    Player2 = 2,
}

#[derive(Debug, Clone)]
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

impl Board {
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
