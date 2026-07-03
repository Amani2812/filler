#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    One,
    Two,
}

#[derive(Debug, Clone)]
pub struct Board {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Vec<char>>,
}

#[derive(Debug, Clone)]
pub struct Piece {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Vec<char>>,
}

impl Player {
    pub fn own_chars(self) -> [char; 2] {
        match self {
            Player::One => ['@', 'a'],
            Player::Two => ['$', 's'],
        }
    }

    pub fn opp_chars(self) -> [char; 2] {
        match self {
            Player::One => ['$', 's'],
            Player::Two => ['@', 'a'],
        }
    }
}
