use crate::definitions::*;
use std::io;
use console::Term;
use rand::{thread_rng, Rng};

fn move_is_valid(card: &Card, hand: &[Card]) -> bool {
    let test = hand.binary_search(card);
    test.is_ok()
}

fn move_choose(hand: &[Card]) -> Card {
    println!("Pick a card to play!!! (number from 1 to {})", &hand.len());

    let mut card_input = String::new();
    io::stdin().read_line(&mut card_input).expect("gay");

    let card_number =  card_input.trim_end().parse::<usize>().expect("I need a number you fool");

    hand[card_number - 1]
}

fn turn(attacker: Player, defender: Player) {
    attacker.print_hand();

    let attacker_move = move_choose(&attacker.hand);

//    println!("attacker chose: {}", &attacker_move);

    defender.print_hand();

    let defender_move = move_choose(&defender.hand);

//    println!("defender chose: {}", &defender_move);

    if defender_move.0 > attacker_move.0 {
        println!("Defender won")
    } else {
        println!("Attacker won")
    }

}

pub fn game_play(game: GameState) {
    let x = 0;

    let attacker = game.players.get(x).unwrap().to_owned();

    let defender = game.players.get(x + 1).unwrap().to_owned();

    println!("The fricking attacker man: {:?}", &attacker);
    println!("The fricking defender man: {:?}", &defender);

    turn(attacker, defender);

}
