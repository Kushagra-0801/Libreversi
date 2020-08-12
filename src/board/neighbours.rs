use crate::board::{Board, Disc};
use crate::position::Position;

#[derive(Debug)]
pub enum Neighbours<'a> {
    TopLeftCorner(TopLeftCorner<'a>),
    TopRightCorner(TopRightCorner<'a>),
    BottomLeftCorner(BottomLeftCorner<'a>),
    BottomRightCorner(BottomRightCorner<'a>),
    TopEdge(TopEdge<'a>),
    RightEdge(RightEdge<'a>),
    BottomEdge(BottomEdge<'a>),
    LeftEdge(LeftEdge<'a>),
    InnerPoint(InnerPoint<'a>),
}

impl<'a> Neighbours<'a> {
    pub fn top_left_corner(board: &'a Board) -> Self {
        Neighbours::TopLeftCorner(TopLeftCorner { board, count: 0 })
    }

    pub fn top_right_corner(board: &'a Board) -> Self {
        Neighbours::TopRightCorner(TopRightCorner { board, count: 0 })
    }

    pub fn bottom_left_corner(board: &'a Board) -> Self {
        Neighbours::BottomLeftCorner(BottomLeftCorner { board, count: 0 })
    }

    pub fn bottom_right_corner(board: &'a Board) -> Self {
        Neighbours::BottomRightCorner(BottomRightCorner { board, count: 0 })
    }

    pub fn top_edge(board: &'a Board, pos: Position) -> Self {
        Neighbours::TopEdge(TopEdge {
            board,
            pos,
            count: 0,
        })
    }

    pub fn right_edge(board: &'a Board, pos: Position) -> Self {
        Neighbours::RightEdge(RightEdge {
            board,
            pos,
            count: 0,
        })
    }

    pub fn bottom_edge(board: &'a Board, pos: Position) -> Self {
        Neighbours::BottomEdge(BottomEdge {
            board,
            pos,
            count: 0,
        })
    }

    pub fn left_edge(board: &'a Board, pos: Position) -> Self {
        Neighbours::LeftEdge(LeftEdge {
            board,
            pos,
            count: 0,
        })
    }

    pub fn inner_point(board: &'a Board, pos: Position) -> Self {
        Neighbours::InnerPoint(InnerPoint {
            board,
            pos,
            count: 0,
        })
    }
}

impl<'a> Iterator for Neighbours<'a> {
    type Item = (Position, Disc);
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Neighbours::TopLeftCorner(a) => a.next(),
            Neighbours::TopRightCorner(a) => a.next(),
            Neighbours::BottomLeftCorner(a) => a.next(),
            Neighbours::BottomRightCorner(a) => a.next(),
            Neighbours::TopEdge(a) => a.next(),
            Neighbours::RightEdge(a) => a.next(),
            Neighbours::BottomEdge(a) => a.next(),
            Neighbours::LeftEdge(a) => a.next(),
            Neighbours::InnerPoint(a) => a.next(),
        }
    }
}

#[derive(Debug)]
pub struct TopLeftCorner<'a> {
    board: &'a Board,
    count: u8,
}

impl<'a> Iterator for TopLeftCorner<'a> {
    type Item = (Position, Disc);
    fn next(&mut self) -> Option<Self::Item> {
        let out_idx: (usize, usize) = match self.count {
            0 => (0, 1),
            1 => (1, 1),
            2 => (1, 0),
            _ => return None,
        };
        self.count += 1;
        Some((out_idx.into(), self.board[out_idx]))
    }
}

#[derive(Debug)]
pub struct TopRightCorner<'a> {
    board: &'a Board,
    count: u8,
}

impl<'a> Iterator for TopRightCorner<'a> {
    type Item = (Position, Disc);
    fn next(&mut self) -> Option<Self::Item> {
        let out_idx: (usize, usize) = match self.count {
            0 => (1, 7),
            1 => (1, 6),
            2 => (0, 6),
            _ => return None,
        };
        self.count += 1;
        Some((out_idx.into(), self.board[out_idx]))
    }
}

#[derive(Debug)]
pub struct BottomLeftCorner<'a> {
    board: &'a Board,
    count: u8,
}

impl<'a> Iterator for BottomLeftCorner<'a> {
    type Item = (Position, Disc);
    fn next(&mut self) -> Option<Self::Item> {
        let out_idx: (usize, usize) = match self.count {
            0 => (6, 0),
            1 => (6, 1),
            2 => (7, 1),
            _ => return None,
        };
        self.count += 1;
        Some((out_idx.into(), self.board[out_idx]))
    }
}

#[derive(Debug)]
pub struct BottomRightCorner<'a> {
    board: &'a Board,
    count: u8,
}

impl<'a> Iterator for BottomRightCorner<'a> {
    type Item = (Position, Disc);
    fn next(&mut self) -> Option<Self::Item> {
        let out_idx: (usize, usize) = match self.count {
            0 => (6, 7),
            1 => (7, 6),
            2 => (6, 6),
            _ => return None,
        };
        self.count += 1;
        Some((out_idx.into(), self.board[out_idx]))
    }
}

#[derive(Debug)]
pub struct TopEdge<'a> {
    board: &'a Board,
    pos: Position,
    count: u8,
}

impl<'a> Iterator for TopEdge<'a> {
    type Item = (Position, Disc);
    fn next(&mut self) -> Option<Self::Item> {
        let out_idx: (usize, usize) = match self.count {
            0 => (0, self.pos.col() + 1),
            1 => (1, self.pos.col() + 1),
            2 => (1, self.pos.col()),
            3 => (1, self.pos.col() - 1),
            4 => (0, self.pos.col() - 1),
            _ => return None,
        };
        self.count += 1;
        Some((out_idx.into(), self.board[out_idx]))
    }
}

#[derive(Debug)]
pub struct RightEdge<'a> {
    board: &'a Board,
    pos: Position,
    count: u8,
}

impl<'a> Iterator for RightEdge<'a> {
    type Item = (Position, Disc);
    fn next(&mut self) -> Option<Self::Item> {
        let out_idx: (usize, usize) = match self.count {
            0 => (self.pos.row() - 1, 7),
            1 => (self.pos.row() + 1, 7),
            2 => (self.pos.row() + 1, 6),
            3 => (self.pos.row(), 6),
            4 => (self.pos.row() - 1, 6),
            _ => return None,
        };
        self.count += 1;
        Some((out_idx.into(), self.board[out_idx]))
    }
}

#[derive(Debug)]
pub struct BottomEdge<'a> {
    board: &'a Board,
    pos: Position,
    count: u8,
}

impl<'a> Iterator for BottomEdge<'a> {
    type Item = (Position, Disc);
    fn next(&mut self) -> Option<Self::Item> {
        let out_idx: (usize, usize) = match self.count {
            0 => (6, self.pos.col()),
            1 => (6, self.pos.col() + 1),
            2 => (7, self.pos.col() + 1),
            3 => (7, self.pos.col() - 1),
            4 => (6, self.pos.col() - 1),
            _ => return None,
        };
        self.count += 1;
        Some((out_idx.into(), self.board[out_idx]))
    }
}

#[derive(Debug)]
pub struct LeftEdge<'a> {
    board: &'a Board,
    pos: Position,
    count: u8,
}

impl<'a> Iterator for LeftEdge<'a> {
    type Item = (Position, Disc);
    fn next(&mut self) -> Option<Self::Item> {
        let out_idx: (usize, usize) = match self.count {
            0 => (self.pos.row() - 1, 0),
            1 => (self.pos.row() - 1, 1),
            2 => (self.pos.row(), 1),
            3 => (self.pos.row() + 1, 1),
            4 => (self.pos.row() + 1, 0),
            _ => return None,
        };
        self.count += 1;
        Some((out_idx.into(), self.board[out_idx]))
    }
}

#[derive(Debug)]
pub struct InnerPoint<'a> {
    board: &'a Board,
    pos: Position,
    count: u8,
}

impl<'a> Iterator for InnerPoint<'a> {
    type Item = (Position, Disc);
    fn next(&mut self) -> Option<Self::Item> {
        let out_idx: (usize, usize) = match self.count {
            0 => (self.pos.row() - 1, self.pos.col()),
            1 => (self.pos.row() - 1, self.pos.col() + 1),
            2 => (self.pos.row(), self.pos.col() + 1),
            3 => (self.pos.row() + 1, self.pos.col() + 1),
            4 => (self.pos.row() + 1, self.pos.col()),
            5 => (self.pos.row() + 1, self.pos.col() - 1),
            6 => (self.pos.row(), self.pos.col() - 1),
            7 => (self.pos.row() - 1, self.pos.col() - 1),
            _ => return None,
        };
        self.count += 1;
        Some((out_idx.into(), self.board[out_idx]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn top_left() {
        let board = Board::default();
        let mut neighbours = Neighbours::top_left_corner(&board);
        assert_eq!(neighbours.next(), Some(((0u8, 1u8).into(), Disc::Empty)));
        assert_eq!(neighbours.next(), Some(((1u8, 1u8).into(), Disc::Empty)));
        assert_eq!(neighbours.next(), Some(((1u8, 0u8).into(), Disc::Empty)));
        assert_eq!(neighbours.next(), None);
    }

    #[test]
    fn top_right() {
        let board = Board::default();
        let mut neighbours = Neighbours::top_right_corner(&board);
        assert_eq!(neighbours.next(), Some(((1u8, 7u8).into(), Disc::Empty)));
        assert_eq!(neighbours.next(), Some(((1u8, 6u8).into(), Disc::Empty)));
        assert_eq!(neighbours.next(), Some(((0u8, 6u8).into(), Disc::Empty)));
        assert_eq!(neighbours.next(), None);
    }

    #[test]
    fn bot_left() {
        let board = Board::default();
        let mut neighbours = Neighbours::bottom_left_corner(&board);
        assert_eq!(neighbours.next(), Some(((6u8, 0u8).into(), Disc::Empty)));
        assert_eq!(neighbours.next(), Some(((6u8, 1u8).into(), Disc::Empty)));
        assert_eq!(neighbours.next(), Some(((7u8, 1u8).into(), Disc::Empty)));
        assert_eq!(neighbours.next(), None);
    }

    #[test]
    fn bot_right() {
        let board = Board::default();
        let mut neighbours = Neighbours::bottom_right_corner(&board);
        assert_eq!(neighbours.next(), Some(((6u8, 7u8).into(), Disc::Empty)));
        assert_eq!(neighbours.next(), Some(((7u8, 6u8).into(), Disc::Empty)));
        assert_eq!(neighbours.next(), Some(((6u8, 6u8).into(), Disc::Empty)));
        assert_eq!(neighbours.next(), None);
    }

    #[test]
    fn top_edge() {
        let board = Board::default();
        let mut neighbours = Neighbours::top_edge(&board, (0u8, 4u8).into());
        assert_eq!(neighbours.next(), Some(((0u8, 5u8).into(), Disc::Empty)));
        assert_eq!(neighbours.next(), Some(((1u8, 5u8).into(), Disc::Empty)));
        assert_eq!(neighbours.next(), Some(((1u8, 4u8).into(), Disc::Empty)));
        assert_eq!(neighbours.next(), Some(((1u8, 3u8).into(), Disc::Empty)));
        assert_eq!(neighbours.next(), Some(((0u8, 3u8).into(), Disc::Empty)));
        assert_eq!(neighbours.next(), None);
    }

    #[test]
    fn right_edge() {
        let board = Board::default();
        let mut neighbours = Neighbours::right_edge(&board, (4u8, 7u8).into());
        assert_eq!(neighbours.next(), Some(((3u8, 7u8).into(), Disc::Empty)));
        assert_eq!(neighbours.next(), Some(((5u8, 7u8).into(), Disc::Empty)));
        assert_eq!(neighbours.next(), Some(((5u8, 6u8).into(), Disc::Empty)));
        assert_eq!(neighbours.next(), Some(((4u8, 6u8).into(), Disc::Empty)));
        assert_eq!(neighbours.next(), Some(((3u8, 6u8).into(), Disc::Empty)));
        assert_eq!(neighbours.next(), None);
    }

    #[test]
    fn bot_edge() {
        let board = Board::default();
        let mut neighbours = Neighbours::bottom_edge(&board, (7u8, 4u8).into());
        assert_eq!(neighbours.next(), Some(((6u8, 4u8).into(), Disc::Empty)));
        assert_eq!(neighbours.next(), Some(((6u8, 5u8).into(), Disc::Empty)));
        assert_eq!(neighbours.next(), Some(((7u8, 5u8).into(), Disc::Empty)));
        assert_eq!(neighbours.next(), Some(((7u8, 3u8).into(), Disc::Empty)));
        assert_eq!(neighbours.next(), Some(((6u8, 3u8).into(), Disc::Empty)));
        assert_eq!(neighbours.next(), None);
    }

    #[test]
    fn left_edge() {
        let board = Board::default();
        let mut neighbours = Neighbours::left_edge(&board, (4u8, 0u8).into());
        assert_eq!(neighbours.next(), Some(((3u8, 0u8).into(), Disc::Empty)));
        assert_eq!(neighbours.next(), Some(((3u8, 1u8).into(), Disc::Empty)));
        assert_eq!(neighbours.next(), Some(((4u8, 1u8).into(), Disc::Empty)));
        assert_eq!(neighbours.next(), Some(((5u8, 1u8).into(), Disc::Empty)));
        assert_eq!(neighbours.next(), Some(((5u8, 0u8).into(), Disc::Empty)));
        assert_eq!(neighbours.next(), None);
    }

    #[test]
    fn inner_point() {
        let board = Board::default();
        let mut neighbours = Neighbours::inner_point(&board, (3u8, 3u8).into());
        assert_eq!(neighbours.next(), Some(((2u8, 3u8).into(), Disc::Empty)));
        assert_eq!(neighbours.next(), Some(((2u8, 4u8).into(), Disc::Empty)));
        assert_eq!(neighbours.next(), Some(((3u8, 4u8).into(), Disc::Player1)));
        assert_eq!(neighbours.next(), Some(((4u8, 4u8).into(), Disc::Player2)));
        assert_eq!(neighbours.next(), Some(((4u8, 3u8).into(), Disc::Player1)));
        assert_eq!(neighbours.next(), Some(((4u8, 2u8).into(), Disc::Empty)));
        assert_eq!(neighbours.next(), Some(((3u8, 2u8).into(), Disc::Empty)));
        assert_eq!(neighbours.next(), Some(((2u8, 2u8).into(), Disc::Empty)));
        assert_eq!(neighbours.next(), None);
    }
}
