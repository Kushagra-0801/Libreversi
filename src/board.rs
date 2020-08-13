use crate::position::{Position, MAX_VALID_POS};
use std::ops::Index;

mod neighbours;
mod strider;

use neighbours::Neighbours;
use strider::{Direction, Strider};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Disc {
    Empty,
    Player1,
    Player2,
}

impl From<Player> for Disc {
    fn from(p: Player) -> Self {
        match p {
            Player::Player1 => Disc::Player1,
            Player::Player2 => Disc::Player2,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Player {
    Player1,
    Player2,
}

impl Player {
    fn opponent(&self) -> Self {
        match self {
            Player::Player1 => Player::Player2,
            Player::Player2 => Player::Player1,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    p1: [u8; 8],
    p2: [u8; 8],
}

impl<T: Into<Position>> Index<T> for Board {
    type Output = Disc;
    fn index(&self, index: T) -> &Self::Output {
        let index = index.into();
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
    pub const fn empty() -> Self {
        Self {
            p1: [0; 8],
            p2: [0; 8],
        }
    }

    pub fn iter(&self) -> Iter {
        Iter {
            board: self,
            cur_pos: (0u8, 0u8).into(),
        }
    }

    pub fn get_piece<T: Into<Position>>(&self, pos: T) -> Disc {
        self[pos.into()]
    }

    pub fn set_piece<T: Into<Position>>(&mut self, pos: T, val: Disc) {
        let pos = pos.into();
        let row = pos.row();
        let col = pos.col();
        match val {
            Disc::Empty => {
                self.p1[row] &= !(1 << col);
                self.p2[row] &= !(1 << col);
            }
            Disc::Player1 => {
                self.p1[row] |= 1 << col;
                self.p2[row] &= !(1 << col);
            }
            Disc::Player2 => {
                self.p1[row] &= !(1 << col);
                self.p2[row] |= 1 << col;
            }
        }
    }

    pub fn neighbours<T: Into<Position>>(&self, pos: T) -> Neighbours {
        let pos = pos.into();
        let row = pos.row();
        let col = pos.col();
        match (row, col) {
            (0, 0) => Neighbours::top_left_corner(self),
            (0, 7) => Neighbours::top_right_corner(self),
            (7, 0) => Neighbours::bottom_left_corner(self),
            (7, 7) => Neighbours::bottom_right_corner(self),
            (0, _) => Neighbours::top_edge(self, pos),
            (_, 7) => Neighbours::right_edge(self, pos),
            (7, _) => Neighbours::bottom_edge(self, pos),
            (_, 0) => Neighbours::left_edge(self, pos),
            (_, _) => Neighbours::inner_point(self, pos),
        }
    }

    pub fn get_points_in_line<T: Into<Position>, U: Into<Position>>(
        &self,
        pos: T,
        neighbour_pos: U,
    ) -> Strider {
        let pos = pos.into();
        let neighbour_pos = neighbour_pos.into();
        let dir = match pos.idx as i8 - neighbour_pos.idx as i8 {
            8 => Direction::Up,
            7 => Direction::UpRight,
            -1 => Direction::Right,
            -9 => Direction::DownRight,
            -8 => Direction::Down,
            -7 => Direction::DownLeft,
            1 => Direction::Left,
            9 => Direction::UpLeft,
            _ => unreachable!(),
        };
        Strider {
            board: self,
            pos,
            dir,
        }
    }

    pub fn is_legal_move<T: Into<Position>>(&self, pos: T, player: Player) -> bool {
        let pos = pos.into();
        if self[pos] != Disc::Empty {
            return false;
        }
        let opponent = player.opponent().into();
        let player = player.into();
        for (neighbour_pos, neighbour_disc) in self.neighbours(pos) {
            if neighbour_disc == opponent {
                for (_, disc) in self.get_points_in_line(pos, neighbour_pos) {
                    match disc {
                        // All cases are covered as opponent and player are different player discs
                        Disc::Empty => break,
                        x if x == opponent => continue,
                        x if x == player => return true,
                        _ => unreachable!(),
                    }
                }
            }
        }
        false
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

    fn size_hint(&self) -> (usize, Option<usize>) {
        (64, Some(64))
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
        assert_eq!(board[(0u8, 5u8)], Disc::Empty);
        assert_eq!(board[(3u8, 3u8)], Disc::Player2);
        assert_eq!(board[(3u8, 4u8)], Disc::Player1);
        assert_eq!(board[(4u8, 3u8)], Disc::Player1);
        assert_eq!(board[(4u8, 4u8)], Disc::Player2);
        assert_eq!(board[(7u8, 7u8)], Disc::Empty);
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

    #[test]
    fn test_board_iterator() {
        let board = Board::empty();
        assert_eq!(board.iter().count(), 64);
        println!("{:?}", board.iter().size_hint());
    }

    #[test]
    fn test_value_getter_setter() {
        let mut board = Board::default();
        assert_eq!(board[(0u8, 3u8)], Disc::Empty);
        assert_eq!(board.get_piece((0u8, 3u8)), Disc::Empty);
        board.set_piece((0u8, 3u8), Disc::Player1);
        assert_eq!(board[(0u8, 3u8)], Disc::Player1);
        assert_eq!(board.get_piece((0u8, 3u8)), Disc::Player1);
        assert_eq!(board[(3u8, 3u8)], Disc::Player2);
        assert_eq!(board.get_piece((3u8, 3u8)), Disc::Player2);
        board.set_piece((3u8, 3u8), Disc::Player1);
        assert_eq!(board[(3u8, 3u8)], Disc::Player1);
        assert_eq!(board.get_piece((3u8, 3u8)), Disc::Player1);
    }

    #[test]
    fn test_neighbours() {
        let board = Board::default();
        assert!(matches!(
            board.neighbours((0u8, 0u8)),
            Neighbours::TopLeftCorner(_)
        ));
        assert!(matches!(
            board.neighbours((0u8, 7u8)),
            Neighbours::TopRightCorner(_)
        ));
        assert!(matches!(
            board.neighbours((7u8, 0u8)),
            Neighbours::BottomLeftCorner(_)
        ));
        assert!(matches!(
            board.neighbours((7u8, 7u8)),
            Neighbours::BottomRightCorner(_)
        ));
        assert!(matches!(
            board.neighbours((0u8, 5u8)),
            Neighbours::TopEdge(_)
        ));
        assert!(matches!(
            board.neighbours((4u8, 7u8)),
            Neighbours::RightEdge(_)
        ));
        assert!(matches!(
            board.neighbours((7u8, 5u8)),
            Neighbours::BottomEdge(_)
        ));
        assert!(matches!(
            board.neighbours((5u8, 0u8)),
            Neighbours::LeftEdge(_)
        ));
        assert!(matches!(
            board.neighbours((4u8, 6u8)),
            Neighbours::InnerPoint(_)
        ));
    }

    #[test]
    fn test_striders() {
        let board = Board::default();
        assert!(matches!(
            board.get_points_in_line((3u8, 4u8), (2u8, 4u8)),
            Strider {dir: Direction::Up, ..}
        ));
        assert!(matches!(
            board.get_points_in_line((3u8, 4u8), (2u8, 5u8)),
            Strider {dir: Direction::UpRight, ..}
        ));
        assert!(matches!(
            board.get_points_in_line((3u8, 4u8), (3u8, 5u8)),
            Strider {dir: Direction::Right, ..}
        ));
        assert!(matches!(
            board.get_points_in_line((3u8, 4u8), (4u8, 5u8)),
            Strider {dir: Direction::DownRight, ..}
        ));
        assert!(matches!(
            board.get_points_in_line((3u8, 4u8), (4u8, 4u8)),
            Strider {dir: Direction::Down, ..}
        ));
        assert!(matches!(
            board.get_points_in_line((3u8, 4u8), (4u8, 3u8)),
            Strider {dir: Direction::DownLeft, ..}
        ));
        assert!(matches!(
            board.get_points_in_line((3u8, 4u8), (3u8, 3u8)),
            Strider {dir: Direction::Left, ..}
        ));
        assert!(matches!(
            board.get_points_in_line((3u8, 4u8), (2u8, 3u8)),
            Strider {dir: Direction::UpLeft, ..}
        ));
    }

    #[test]
    fn test_legal_moves() {
        let board = Board::default();
        let player1 = Player::Player1;
        let player2 = Player::Player2;
        assert!(board.is_legal_move((2u8, 3u8), player1));
        assert!(!board.is_legal_move((2u8, 3u8), player2));
        assert!(board.is_legal_move((2u8, 4u8), player2));
        assert!(!board.is_legal_move((2u8, 4u8), player1));
        assert!(!board.is_legal_move((3u8, 4u8), player1));
        assert!(!board.is_legal_move((3u8, 4u8), player2));
    }
}
