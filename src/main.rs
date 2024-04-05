mod start;
mod game;

use start::*;

fn main() {
    let game = game_setup();

    println!("{:?}", game.deck);
    println!("{:?}", game.players);

}
