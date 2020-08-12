pub(crate) const MAX_VALID_POS: u8 = 0b00111111; // (7, 7)

#[derive(Debug, Copy, Clone)]
pub struct Position {
    /// Use a single bit for indexing
    ///
    /// The last three bits are the column and the first three are the row
    pub(crate) idx: u8,
}

impl Position {
    pub fn row(&self) -> usize {
        ((self.idx >> 3) & 0b111) as usize
    }

    pub fn col(&self) -> usize {
        (self.idx & 0b111) as usize
    }
}

impl From<(u8, u8)> for Position {
    fn from(p: (u8, u8)) -> Self {
        if p.0 > 7 || p.1 > 7 {
            panic!("Index out of bounds")
        }
        Self {
            idx: (p.0 << 3) + p.1,
        }
    }
}

impl From<(usize, usize)> for Position {
    fn from(p: (usize, usize)) -> Self {
        if p.0 > 7 || p.1 > 7 {
            panic!("Index out of bounds")
        }
        let p = (p.0 as u8, p.1 as u8);
        p.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_getters() {
        let pos: Position = Position { idx: 3 * 8 + 6 };
        assert_eq!(pos.row(), 3);
        assert_eq!(pos.col(), 6);
    }

    #[test]
    fn test_from_impl() {
        let pos: Position = (5u8, 7u8).into();
        assert_eq!(pos.row(), 5);
        assert_eq!(pos.col(), 6);
        let pos: Position = (5usize, 7usize).into();
        assert_eq!(pos.row(), 5);
        assert_eq!(pos.col(), 6);
    }

    #[should_panic(expected = "Index out of bounds")]
    #[test]
    fn test_from_impl_out_of_bounds() {
        let _pos: Position = (9u8, 20u8).into();
        let _pos: Position = (9usize, 20usize).into();
    }
}
