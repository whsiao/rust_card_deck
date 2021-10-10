mod components;

//use crate::components::deck52::{build_deck, Deck52};
use crate::components::board::{Board, buildBoard};
//use components::deck52;



fn main() {
    println!("Hello, world!");
    let mut testBoard: Board = buildBoard();
    //let mut testDeck: Deck52 = build_deck();
    //testDeck.shuffle_deck();
}
