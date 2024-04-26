use std::fmt;
use rand::{seq::SliceRandom, thread_rng};


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

impl Player {
    pub fn print_hand(&self) {
        println!("{:?}'s hand: {:?}", self.name, self.hand);
    }

    pub fn check_hand_empty(&self) -> bool {
        self.hand.is_empty()
    }
}

#[derive(Debug, Clone)]
pub struct GameState {
    pub players: Vec<Player>,
    pub deck: Vec<Card>,
}

impl GameState {
    pub fn generate_rotation(mut self) -> Self {
        let mut rng = thread_rng();

        self.players.shuffle(&mut rng);
        self
    }

    pub fn shuffle_deck(&mut self) -> Self {
        let mut rng = thread_rng();

        self.deck.shuffle(&mut rng);
        self.to_owned()
    }
}
