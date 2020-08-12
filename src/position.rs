pub(crate) const MAX_VALID_POS: u8 = 0b00111111; // (7, 7)

#[derive(Debug, Copy, Clone)]
pub struct Position {
    /// Use a single bit for indexing
    ///
    /// The last three bits are the column and the first three are the row
    pub(crate) idx: u8,
}

impl From<(u8, u8)> for Position {
    fn from(p: (u8, u8)) -> Self {
        if p.0 > 7 || p.1 > 7 {
            panic!("Index out of bounds")
        }
        Self {
            idx: p.0 << 3 + p.1,
        }
    }
}
