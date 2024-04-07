mod start;
mod game;
mod definitions;

use start::*;

use crate::game::game_play;

fn main() {
    let game = game_setup();

    println!("{:?}", &game.deck);
    println!("{:?}", &game.players);

    game_play(game);

}
