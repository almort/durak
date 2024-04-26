use std::io;

use crate::definitions::*;
use console::Term;
use rand::{thread_rng, Rng};

fn play_card(hand: &[Card]) -> u8 {

    let choice = loop {
        println!("Pick a card to play!!! (number from 1 to {})", hand.len());

        let mut card_input = String::new();
        io::stdin().read_line(&mut card_input).expect("gay");
        card_input
            .trim_end()
            .parse::<u8>()
            .expect("I need a number you fool!!!");

        println!(
            "Are you sure of your choice? [y/n]\nSelected: {}",
            &card_input
        );

        let mut sure_check = String::new();
        io::stdin().read_line(&mut sure_check).expect("ultra-gay");
        println!("{}", sure_check);

        if sure_check.trim_end() == "y" {
            break card_input.trim_end().parse();
        } else {
            continue;
        }
    };
    choice.expect("Number needed!")
}

fn turn(attacker: Player, defender: Player) -> Player {
    //let term = Term::stdout();
    //term.clear_screen().unwrap();

    attacker.print_hand();
    defender.print_hand();
    play_card(&attacker.hand);
    defender
}

pub fn game_play(game: GameState) {
    let mut x = 0;

    let mut attacker = game.players.get(x).unwrap().to_owned();
    attacker.status = Status::Atk;

    let mut defender = game.players.get(x + 1).unwrap().to_owned();
    defender.status = Status::Def;

    println!("The fricking attacker man: {:?}", attacker);
    println!("The fricking defender man: {:?}", defender);

    turn(attacker, defender);
}
