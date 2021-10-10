use super::card::Card;

pub struct Player {
    hand: Vec<Card>,
    name: String,
}

impl Player {
    fn draw(&mut self, newCard: Card) {
        self.hand.push(newCard)
    }

    //rearrange cards in hand
    fn insertCardInNewPosition(&mut self, oldIndex: usize, newIndex: usize) {
        let tempCard: Card = self.hand.remove(oldIndex);
        self.hand.insert(newIndex, tempCard)
    }

    fn discard(&mut self, index: usize) -> Card {
        self.hand.remove(index)
    }
}

pub fn createPlayer(name: String) -> Player {
    Player {
        hand: Vec::new(),
        name,
    }
}
