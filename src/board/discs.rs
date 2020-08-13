#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Disc {
    Empty,
    Player1,
    Player2,
}

impl PartialEq<Player> for Disc {
    fn eq(&self, other: &Player) -> bool {
        matches!(
            (other, self),
            (Player::Player1, Disc::Player1) | (Player::Player2, Disc::Player2)
        )
    }
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
    pub fn opponent(&self) -> Self {
        match self {
            Player::Player1 => Player::Player2,
            Player::Player2 => Player::Player1,
        }
    }
}

impl PartialEq<Disc> for Player {
    fn eq(&self, other: &Disc) -> bool {
        matches!(
            (self, other),
            (Player::Player1, Disc::Player1) | (Player::Player2, Disc::Player2)
        )
    }
}
