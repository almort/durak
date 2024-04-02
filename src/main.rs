use core::fmt;
use std::{clone, fs::copy, u8, vec::Vec};
use rand::{thread_rng, Rng};

#[derive(Debug, Copy, Clone)]
enum Suit {
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
enum Status {
    Idle,
    Atk,
    Def,
}

#[derive(Debug, Copy, Clone)]
enum PlayerOutcome {
    Win,
    Lose,
    Null,
}

#[derive(Debug, Copy, Clone)]
struct Card(u8, Suit);

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{}, {}", self.0, self.1)
    }
}

#[derive(Debug, Clone)]
struct Player {
    name: String,
    hand: Vec<Card>,
    hand_length: u8,
    status: Status,
    outcome: PlayerOutcome,
}

impl Player {
    fn generate_hand(mut self, deck: Vec<Card>) -> Vec<Card> {
        let mut x = 0;
        let mut rng = thread_rng();

        self.hand = Vec::new();

        while x < 6 {
            x += 1;
            let card_hand: Card = deck[rng.gen_range(0..deck.len())];

            self.hand.push(card_hand);

        }

        println!("{:?}", self.hand);

        self.hand

    }
}

#[derive(Debug, Clone)]
struct GameState {
    players: String,
    deck: Vec<Card>,
    deck_length: u8,
}

impl GameState {
    fn initialize_deck(mut self) -> Vec<Card> {

        let values = [1, 2, 3, 4, 5, 6, 7, 8, 9];

        self.deck = Vec::new();

        for i in values {

            let card_spades = Card(i, Suit::Spades);
            self.deck.push(card_spades);
            let card_clubs = Card(i, Suit::Clubs);
            self.deck.push(card_clubs);
            let card_hearts = Card(i, Suit::Hearts);
            self.deck.push(card_hearts);
            let card_diamonds = Card(i, Suit::Diamonds);
            self.deck.push(card_diamonds);

            }

        println!("{:?}", self.deck);

        self.deck
    }
}

fn main() {

    let game = GameState {
        players: String::from("Toto"),
        deck: Vec::new(),
        deck_length: 1,
    };

    let deck = game.initialize_deck();


    let p1 = Player {
        name: String::from("Toto"),
        hand: Vec::new(),
        hand_length: 1,
        status: Status::Atk,
        outcome: PlayerOutcome::Null,
    };

    p1.generate_hand(deck);

}


