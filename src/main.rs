mod start;
mod game;
mod definitions;

use start::*;

fn main() {
    let game = game_setup();

    println!("{:?}", &game.deck);
    println!("Shuffled rotation: {:?}", &game.players);

//    game::game_play(game);

}
