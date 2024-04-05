mod start;

use start::*;

fn main() {
    let game = game_setup();

    println!("{:?}", game.deck);
    println!("{:?}", game.players);

}
