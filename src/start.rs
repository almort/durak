use core::panic;

use clap::Parser;

use crate::definitions::*;

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

    game.shuffle_deck();

    let mut new_deck = game.deck.clone();


    while x < 6 {
        x += 1;

        let card_hand = game.deck[x];

        hand.push(card_hand);
        new_deck.remove(x);

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
    let cli = Cli::parse();

    //let names: Vec<String> = ["Toto".to_string(), "Porco".to_string()].to_vec();
    let names: Vec<String> = cli.players;

    if names.is_empty() || names.len() == 1 || names.len() > 3 {
        panic!("Rispettate le regole: minimo 2 giocatori e massimo 3!")
    }

    let mut game = GameState {
        players: Vec::new(),
        deck: generate_deck(),
    };

    game.players = generate_players(names, &mut game);

    println!("Unshuffled rotation: {:?}", &game.players);

    game = game.generate_rotation();

    game

}

