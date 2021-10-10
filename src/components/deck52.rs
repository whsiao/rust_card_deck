
// proper import of card

//pub mod deck52 {
use super::card::Card;
use super::card::CardSymbol;
use super::card::SuitType;
use rand;
use rand::Rng;

pub struct Deck52 {
    pub cardList: Vec<Card>,
}

impl Deck52 {
    fn initializeDeck(&mut self) {
        // setup 52 cards 4 suit
        // create the first list and then start randomly shuffling the deck
        // we want to keep track of cards
        self.shuffle_deck()
    }

    fn draw_card(&mut self) -> Card {
        // add card from discard
        if self.cardList.is_empty() {
            self.shuffle_deck();
        }
        self.cardList.remove(self.cardList.len())
    }

    pub fn shuffle_deck(&mut self) {
        let mut picker = rand::thread_rng();
        let mut tempDeck: Vec<Card> = self.cardList.drain(..).collect();

        // shuffle deck
        while !tempDeck.is_empty() {
            let tempCard: Card = tempDeck.remove(picker.gen_range(0..tempDeck.len()));
            self.cardList.push(tempCard)
        }

        // shuffle cards from discard pile

        //print cards
        /*for i in &self.cardList {
            println!("{} {}", i.symbol, i.suit)
        }*/
    }
}

pub fn build_deck() -> Deck52 {
    let mut newDeck: Deck52 = Deck52 {
        cardList: Vec::new()
    };

    // make listSymbol a list of symbols expected in deck
    // make this available in board
    let listSymbol: [SuitType; 4] = [SuitType::Club, SuitType::Diamond, SuitType::Heart, SuitType::Spade];

    // when we get to board game, we need to make sure the suitType is referenced not copied
    for i in 0..3 {
        newDeck.cardList.push(Card { suit: i, symbol: CardSymbol::One(1) });
        newDeck.cardList.push(Card { suit: i, symbol: CardSymbol::Two(2) });
        newDeck.cardList.push(Card { suit: i, symbol: CardSymbol::Three(3) });
        newDeck.cardList.push(Card { suit: i, symbol: CardSymbol::Four(4) });
        newDeck.cardList.push(Card { suit: i, symbol: CardSymbol::Five(5) });
        newDeck.cardList.push(Card { suit: i, symbol: CardSymbol::Six(6) });
        newDeck.cardList.push(Card { suit: i, symbol: CardSymbol::Seven(7) });
        newDeck.cardList.push(Card { suit: i, symbol: CardSymbol::Eight(8) });
        newDeck.cardList.push(Card { suit: i, symbol: CardSymbol::Nine(9) });
        newDeck.cardList.push(Card { suit: i, symbol: CardSymbol::Ten(10) });
        newDeck.cardList.push(Card { suit: i, symbol: CardSymbol::Jack(11) });
        newDeck.cardList.push(Card { suit: i, symbol: CardSymbol::Queen(12) });
        newDeck.cardList.push(Card { suit: i, symbol: CardSymbol::King(13) });
        newDeck.cardList.push(Card { suit: i, symbol: CardSymbol::Jack(14) });
    }
    newDeck
}
//}
