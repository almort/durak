use std::fmt;
use rand::{thread_rng, Rng};

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
    Idle,
    Atk,
    Def,
}

#[derive(Debug, Copy, Clone)]
pub enum PlayerOutcome {
    Win,
    Lose,
    Null,
}

#[derive(Debug, Copy, Clone)]
pub struct Card(u8, Suit);

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{}, {}", self.0, self.1)
    }
}

#[derive(Debug, Clone)]
pub struct Player {
    name: String,
    hand: Vec<Card>,
    hand_length: u8,
    status: Status,
    outcome: PlayerOutcome,
}

#[derive(Debug, Clone)]
pub struct GameState {
    players: Vec<Player>,
    deck: Vec<Card>,
    deck_length: u8,
}

fn random_card(deck: &[Card]) -> usize {
    let mut rng = thread_rng();
    rng.gen_range(0..deck.len())
}

fn generate_deck() -> Vec<Card> {

    let mut deck = Vec::new();

    for n in 1..=9 {

        let card_spades = Card(n, Suit::Spades);
        deck.push(card_spades);
        let card_clubs = Card(n, Suit::Clubs);
        deck.push(card_clubs);
        let card_hearts = Card(n, Suit::Hearts);
        deck.push(card_hearts);
        let card_diamonds = Card(n, Suit::Diamonds);
        deck.push(card_diamonds);

        }

    deck

}

fn generate_hand(game: &GameState) -> Vec<Card> {
    let mut x = 0;
    let mut hand = Vec::new();

    while x < 6 {
        x += 1;

        let random_card_index = random_card(&game.deck);
        let card_hand: Card = game.deck[random_card_index];

        hand.push(card_hand);

//        deck_remove(game, random_card_index);

    }

    hand

}

pub fn generate_players(names: Vec<String>, game: &GameState) -> Vec<Player> {

    let mut players = Vec::new();

    for name_from_vec in names {

        let p = Player {
            name: name_from_vec,
            hand: generate_hand(game),
            hand_length: 0,
            status: Status::Idle,
            outcome: PlayerOutcome::Null,
        };

        players.push(p)

    }

    players
}

pub fn game_setup() {
    let names: Vec<String> = ["Toto".to_string(), "Porco".to_string()].to_vec();

    let mut game = GameState {
        players: Vec::new(),
        deck: generate_deck(),
        deck_length: 0,
    };

    game.players = generate_players(names, &game);

    println!("{:?}", game.deck);
    println!("{:?}", game.players);
    println!("{:?}", game.players);
}

//fn deck_remove(mut game: &GameState, i: usize) -> GameState {
//    game.deck.remove(i);
//    game
//}
