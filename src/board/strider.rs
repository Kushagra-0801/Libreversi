#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

use crate::board::{Board, Disc};
use crate::position::Position;

#[derive(Debug)]
pub struct Strider<'a> {
    pub board: &'a Board,
    pub pos: Position,
    pub dir: Direction,
}

impl<'a> Iterator for Strider<'a> {
    type Item = (Position, Disc);
    fn next(&mut self) -> Option<Self::Item> {
        match self.dir {
            Direction::Up => {
                if self.pos.row() == 0 {
                    return None;
                }
                self.pos.idx -= 1 << 3;
            }
            Direction::UpRight => {
                if self.pos.row() == 0 || self.pos.col() == 7 {
                    return None;
                }
                self.pos.idx -= 1 << 3;
                self.pos.idx += 1;
            }
            Direction::Right => {
                if self.pos.col() == 7 {
                    return None;
                }
                self.pos.idx += 1;
            }
            Direction::DownRight => {
                if self.pos.col() == 7 || self.pos.row() == 7 {
                    return None;
                }
                self.pos.idx += 1;
                self.pos.idx += 1 << 3;
            }
            Direction::Down => {
                if self.pos.row() == 7 {
                    return None;
                }
                self.pos.idx += 1 << 3;
            }
            Direction::DownLeft => {
                if self.pos.row() == 7 || self.pos.col() == 0 {
                    return None;
                }
                self.pos.idx += 1 << 3;
                self.pos.idx -= 1;
            }
            Direction::Left => {
                if self.pos.col() == 0 {
                    return None;
                }
                self.pos.idx -= 1;
            }
            Direction::UpLeft => {
                if self.pos.col() == 0 || self.pos.row() == 0 {
                    return None;
                }
                self.pos.idx -= 1;
                self.pos.idx -= 1 << 3;
            }
        }
        Some((self.pos, self.board[self.pos]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BOARD: Board = Board::empty();
    const POS: Position = Position { idx: 3 * 8 + 4 };

    #[test]
    fn test_strider_up() {
        let mut strider = Strider {
            board: &BOARD,
            pos: POS,
            dir: Direction::Up,
        };
        assert_eq!(strider.next(), Some(((2u8, 4u8).into(), Disc::Empty)));
        assert_eq!(strider.next(), Some(((1u8, 4u8).into(), Disc::Empty)));
        assert_eq!(strider.next(), Some(((0u8, 4u8).into(), Disc::Empty)));
        assert_eq!(strider.next(), None);
    }

    #[test]
    fn test_strider_up_right() {
        let mut strider = Strider {
            board: &BOARD,
            pos: POS,
            dir: Direction::UpRight,
        };
        assert_eq!(strider.next(), Some(((2u8, 5u8).into(), Disc::Empty)));
        assert_eq!(strider.next(), Some(((1u8, 6u8).into(), Disc::Empty)));
        assert_eq!(strider.next(), Some(((0u8, 7u8).into(), Disc::Empty)));
        assert_eq!(strider.next(), None);
    }

    #[test]
    fn test_strider_right() {
        let mut strider = Strider {
            board: &BOARD,
            pos: POS,
            dir: Direction::Right,
        };
        assert_eq!(strider.next(), Some(((3u8, 5u8).into(), Disc::Empty)));
        assert_eq!(strider.next(), Some(((3u8, 6u8).into(), Disc::Empty)));
        assert_eq!(strider.next(), Some(((3u8, 7u8).into(), Disc::Empty)));
        assert_eq!(strider.next(), None);
    }

    #[test]
    fn test_strider_down_right() {
        let mut strider = Strider {
            board: &BOARD,
            pos: POS,
            dir: Direction::DownRight,
        };
        assert_eq!(strider.next(), Some(((4u8, 5u8).into(), Disc::Empty)));
        assert_eq!(strider.next(), Some(((5u8, 6u8).into(), Disc::Empty)));
        assert_eq!(strider.next(), Some(((6u8, 7u8).into(), Disc::Empty)));
        assert_eq!(strider.next(), None);
    }

    #[test]
    fn test_strider_down() {
        let mut strider = Strider {
            board: &BOARD,
            pos: POS,
            dir: Direction::Down,
        };
        assert_eq!(strider.next(), Some(((4u8, 4u8).into(), Disc::Empty)));
        assert_eq!(strider.next(), Some(((5u8, 4u8).into(), Disc::Empty)));
        assert_eq!(strider.next(), Some(((6u8, 4u8).into(), Disc::Empty)));
        assert_eq!(strider.next(), Some(((7u8, 4u8).into(), Disc::Empty)));
        assert_eq!(strider.next(), None);
    }

    #[test]
    fn test_strider_down_left() {
        let mut strider = Strider {
            board: &BOARD,
            pos: POS,
            dir: Direction::DownLeft,
        };
        assert_eq!(strider.next(), Some(((4u8, 3u8).into(), Disc::Empty)));
        assert_eq!(strider.next(), Some(((5u8, 2u8).into(), Disc::Empty)));
        assert_eq!(strider.next(), Some(((6u8, 1u8).into(), Disc::Empty)));
        assert_eq!(strider.next(), Some(((7u8, 0u8).into(), Disc::Empty)));
        assert_eq!(strider.next(), None);
    }

    #[test]
    fn test_strider_left() {
        let mut strider = Strider {
            board: &BOARD,
            pos: POS,
            dir: Direction::Left,
        };
        assert_eq!(strider.next(), Some(((3u8, 3u8).into(), Disc::Empty)));
        assert_eq!(strider.next(), Some(((3u8, 2u8).into(), Disc::Empty)));
        assert_eq!(strider.next(), Some(((3u8, 1u8).into(), Disc::Empty)));
        assert_eq!(strider.next(), Some(((3u8, 0u8).into(), Disc::Empty)));
        assert_eq!(strider.next(), None);
    }

    #[test]
    fn test_strider_up_left() {
        let mut strider = Strider {
            board: &BOARD,
            pos: POS,
            dir: Direction::UpLeft,
        };
        assert_eq!(strider.next(), Some(((2u8, 3u8).into(), Disc::Empty)));
        assert_eq!(strider.next(), Some(((1u8, 2u8).into(), Disc::Empty)));
        assert_eq!(strider.next(), Some(((0u8, 1u8).into(), Disc::Empty)));
        assert_eq!(strider.next(), None);
    }
}
