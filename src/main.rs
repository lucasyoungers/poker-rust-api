mod card;
mod hand;
mod deck;
mod game;
mod utils;

use crate::game::Game;

fn main() {
    let names = vec!["Lucas", "Alex", "Jordynn"];
    let mut game = Game::new(&names);

    println!("{}", game);

    game.flop();
    game.turn();
    game.river();

    println!("\n{}", game);

    println!("\nWinner: {}", game.winner())
}
