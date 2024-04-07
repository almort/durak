use std::fmt;


#[derive(Debug, Copy, Clone)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let a = self;
        write!(f, "{}", a)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Status {
    Atk,
    Def,
    None,
}

#[derive(Debug, Copy, Clone)]
pub enum PlayerOutcome {
    Win,
    Lose,
    None,
}

#[derive(Debug, Copy, Clone)]
pub struct Card(pub u8, pub Suit);

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{}, {}", self.0, self.1)
    }
}

#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub hand: Vec<Card>,
    pub status: Status,
    pub outcome: PlayerOutcome,
}

#[derive(Debug, Clone)]
pub struct GameState {
    pub players: Vec<Player>,
    pub deck: Vec<Card>,
}
