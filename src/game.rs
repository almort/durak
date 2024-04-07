use crate::definitions::*;
use rand::{thread_rng, Rng};

fn choose_first_player(game: &GameState) -> &Player {

    let mut rng = thread_rng();

    game.players.get(rng.gen_range(0..game.players.len())).unwrap()

}

pub fn game_play(game: GameState) {

    let first_player = choose_first_player(&game);

    println!("The first player is: {:?}", &first_player.name);

}
