use super::deck52::{build_deck, Deck52};
use super::card::{Card, SuitType};
use super::player::{Player, createPlayer};
use std::collections::HashMap;
// metadata on card composition.  Should that be in deck or board

pub struct Board {
    pub deck: Deck52,
    pub discard: Vec<Card>,
    pub hashMapCardSymbol: HashMap<i32, SuitType>,
    pub players: Vec<Player>,
}

// initialize board
impl Board {
    pub fn buildBoard(&mut self) {
        self.hashMapCardSymbol.insert(0, SuitType::Club);
        self.hashMapCardSymbol.insert(1, SuitType::Diamond);
        self.hashMapCardSymbol.insert(2, SuitType::Heart);
        self.hashMapCardSymbol.insert(3, SuitType::Spade);

        self.deck = build_deck();
        self.deck.shuffle_deck();

        for i in &self.deck.cardList {
            println!("{} {}", i.symbol, &self.hashMapCardSymbol[&i.suit])
        }
        // player
        // discard pile
        // deck

        //scoring system
    }

    /*fn resetPlayingField(&mut self) {
        // take all cards from players and shuffle the cards back into the deck
    }*/
}

pub fn buildBoard() -> Board {
    let mut newHashmap: HashMap<i32, SuitType> = HashMap::new();
    newHashmap.insert(0, SuitType::Club);
    newHashmap.insert(1, SuitType::Diamond);
    newHashmap.insert(2, SuitType::Heart);
    newHashmap.insert(3, SuitType::Spade);

    let mut newBoard: Board = Board {
        deck: build_deck(),
        discard: Vec::new(),
        hashMapCardSymbol: newHashmap,
        players: Vec::new(),
    };

    newBoard.deck.shuffle_deck();

    for i in &newBoard.deck.cardList {
        println!("{} {}", i.symbol, &newBoard.hashMapCardSymbol[&i.suit])
    }

    newBoard
}