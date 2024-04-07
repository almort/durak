use std::usize;
use rand::{thread_rng, Rng};

use crate::definitions::*;

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

fn generate_hand(game: &mut GameState) -> Vec<Card> {
    let mut x = 0;
    let mut hand = Vec::new();
    let mut new_deck = game.deck.clone();


    while x < 6 {
        x += 1;

        let random_card_index = random_card(&new_deck);
        let card_hand = new_deck[random_card_index];

        hand.push(card_hand);
        new_deck.remove(random_card_index);

    }
    game.deck = new_deck;

    hand

}

fn generate_players(names: Vec<String>, game: &mut GameState) -> Vec<Player> {

    let mut players = Vec::new();

    for name_from_vec in names {

        let p = Player {
            name: name_from_vec,
            hand: generate_hand(game),
            status: Status::None,
            outcome: PlayerOutcome::None,
        };

        players.push(p)

    }

    players
}

pub fn game_setup() -> GameState {
    let names: Vec<String> = ["Toto".to_string(), "Porco".to_string()].to_vec();

    let mut game = GameState {
        players: Vec::new(),
        deck: generate_deck(),
    };

    game.players = generate_players(names, &mut game);

    game
}

