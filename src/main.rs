use std:: {
    ops::Index,
    primitive::usize,
    vec::Vec
};

//------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Rank {
    Ace,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King 
}

//------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Suit {
    Club,
    Spade,
    Diamond,
    Heart
}

//------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit
}

//------------------------------------------------------------------------------------------------
impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Self {
        return Card{ rank: rank, suit: suit };
    }
}

//------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------
#[derive(Debug, Clone)]
pub struct Deck {
    cards: Vec<Card>
}

//------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------
impl Deck {
    pub fn new() -> Deck {
        return Deck {
            cards: vec!()
        };
    }

    pub fn cards(&self) -> &Vec<Card> { return &self.cards; }

    pub fn len(&self) -> usize { return self.cards.len(); }

    pub fn push(&mut self, card: &Card) {
        self.cards.push(*card);
    }

    pub fn insert_ordered_by_rank(&mut self, card: &Card) {
        for idx in 0.. self.cards.len() {
            if self.cards[idx].rank > card.rank {
                self.cards.insert(idx, *card);
                return;
            }
        }

        self.cards.push(*card);
    }
}

impl Index<usize> for Deck {
    type Output = Card;

    fn index(&self, idx: usize) -> &Self::Output {
        return &self.cards[idx];
    }
}

//------------------------------------------------------------------------------------------------
pub fn find_set(rank: Rank, deck: &Deck) -> Vec<usize> {
    let mut indices : Vec<usize> = vec!();
    
    for idx in 0.. deck.len() {
        if deck[idx].rank == rank {
            indices.push(idx);
        }
    }

    if indices.len() < 3 {
        indices.clear();
    }

    return indices;
}

//------------------------------------------------------------------------------------------------
pub fn find_run(element: &Card, deck: &Deck) -> Vec<usize> {
    let mut sorted_same_suit_indices: Vec<usize> = vec!();

    for idx in 0.. deck.len() {
        if deck[idx].suit == element.suit {
            let dst 
                = sorted_same_suit_indices.iter().position(|&x| deck[x].suit > element.suit);

            match dst {
                Some(dst) => {
                    sorted_same_suit_indices.insert(dst, idx);
                },
                None => {
                    sorted_same_suit_indices.push(idx);
                }
            }
        }
    }
    
    let mut indices : Vec<usize> = vec!();

    for idx in sorted_same_suit_indices {
        
        indices.push(idx);
    }
    
    return indices;
}

//------------------------------------------------------------------------------------------------
fn main() {
    println!("Hello, world!");
}

//------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    //--------------------------------------------------------------------------------------------
    #[test]
    fn find_set_test() {
        let mut hand = Deck::new();
        hand.push(&Card::new(Rank::Ace, Suit::Spade));
        hand.push(&Card::new(Rank::Ace, Suit::Club));
        hand.push(&Card::new(Rank::Ace, Suit::Diamond));
        hand.push(&Card::new(Rank::Ace, Suit::Heart));
        hand.push(&Card::new(Rank::Queen, Suit::Spade));
        hand.push(&Card::new(Rank::Queen, Suit::Club));
        hand.push(&Card::new(Rank::Queen, Suit::Diamond));
        hand.push(&Card::new(Rank::King, Suit::Heart));
        hand.push(&Card::new(Rank::King, Suit::Spade));
        hand.push(&Card::new(Rank::Jack, Suit::Club));
        hand.push(&Card::new(Rank::Ten, Suit::Diamond));
        hand.push(&Card::new(Rank::Nine, Suit::Heart));
        hand.push(&Card::new(Rank::Seven, Suit::Spade));
        hand.push(&Card::new(Rank::Four, Suit::Club));

        let aces = find_set(Rank::Ace, &hand);
        assert_eq!(aces.len(), 4);
        assert_eq!(aces[0], 0);
        assert_eq!(aces[1], 1);
        assert_eq!(aces[2], 2);
        assert_eq!(aces[3], 3);

        let queens = find_set(Rank::Queen, &hand);
        assert_eq!(queens.len(), 3);
        assert_eq!(queens[0], 4);
        assert_eq!(queens[1], 5);
        assert_eq!(queens[2], 6);

        let kings = find_set(Rank::King, &hand);
        assert_eq!(kings.len(), 0);

        let jacks = find_set(Rank::Jack, &hand);
        assert_eq!(jacks.len(), 0);
    }

    //--------------------------------------------------------------------------------------------
    /*#[test]
    fn find_run() {
        let mut hand = Deck::new();
        hand.push(&Card::new(Rank::Ace, Suit::Spade));
        hand.push(&Card::new(Rank::Ace, Suit::Club));
        hand.push(&Card::new(Rank::Ace, Suit::Diamond));
        hand.push(&Card::new(Rank::Ace, Suit::Heart));
        hand.push(&Card::new(Rank::Queen, Suit::Spade));
        hand.push(&Card::new(Rank::Queen, Suit::Club));
        hand.push(&Card::new(Rank::Queen, Suit::Diamond));
        hand.push(&Card::new(Rank::King, Suit::Heart));
        hand.push(&Card::new(Rank::King, Suit::Spade));
        hand.push(&Card::new(Rank::Jack, Suit::Club));
        hand.push(&Card::new(Rank::Ten, Suit::Diamond));
        hand.push(&Card::new(Rank::Nine, Suit::Heart));
        hand.push(&Card::new(Rank::Seven, Suit::Spade));
        hand.push(&Card::new(Rank::Four, Suit::Club));

        let aces = find_same_rank(Rank::Ace, &hand);
        assert_eq!(aces.len(), 4);
        assert_eq!(aces[0], 0);
        assert_eq!(aces[1], 1);
        assert_eq!(aces[2], 2);
        assert_eq!(aces[3], 3);

        let queens = find_same_rank(Rank::Queen, &hand);
        assert_eq!(queens.len(), 3);
        assert_eq!(queens[0], 4);
        assert_eq!(queens[1], 5);
        assert_eq!(queens[2], 6);

        let kings = find_same_rank(Rank::King, &hand);
        assert_eq!(kings.len(), 2);
        assert_eq!(kings[0], 7);
        assert_eq!(kings[1], 8);

        let jacks = find_same_rank(Rank::Jack, &hand);
        assert_eq!(jacks.len(), 1);
        assert_eq!(jacks[0], 9);
    }*/
}

