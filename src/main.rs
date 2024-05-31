mod start;
mod game;
mod definitions;

use console::Term;

use start::*;

use crate::game::game_play;

fn main() {
    let term = Term::stdout();

    term.set_title("Durak");
    term.clear_screen().unwrap();

    let game = game_setup();

    println!("Deck: {:?}", &game.deck);
    println!("Shuffled rotation: {:?}", &game.players);

    game_play(game);

}
