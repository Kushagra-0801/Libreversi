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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_getters() {
        let pos: Position = (3, 6).into();
        assert_eq!(pos.col(), 6);
        assert_eq!(pos.row(), 3);
    }
}
