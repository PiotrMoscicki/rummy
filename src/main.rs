use std:: {
    ops::Index,
    primitive::usize,
    vec::Vec
};

//------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------
#[derive(Debug, Copy, Clone, PartialEq)]
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
#[derive(Debug, Copy, Clone, PartialEq)]
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
    pub m_rank: Rank,
    pub m_suit: Suit
}

//------------------------------------------------------------------------------------------------
impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Self {
        return Card{ m_rank: rank, m_suit: suit };
    }
}

//------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------
#[derive(Debug, Clone)]
pub struct Deck {
    m_cards: Vec<Card>
}

//------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------
impl Deck {
    pub fn new() -> Deck {
        return Deck {
            m_cards: vec!()
        };
    }

    pub fn cards(&self) -> &Vec<Card> { return &self.m_cards; }

    pub fn len(&self) -> usize { return self.m_cards.len(); }

    pub fn add(&mut self, card:&Card) {
        self.m_cards.push(*card);
    }
}

impl Index<usize> for Deck {
    type Output = Card;

    fn index(&self, idx: usize) -> &Self::Output {
        return &self.m_cards[idx];
    }
}

//------------------------------------------------------------------------------------------------
pub fn find_same_rank(rank: Rank, deck: &Deck) -> Vec<usize> {
    let mut indices : Vec<usize> = vec!();
    
    for idx in 0.. deck.len() {
        if deck[idx].m_rank == rank {
            indices.push(idx);
        }
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
    fn find_four_of_rank() {
        let mut hand = Deck::new();
        hand.add(&Card::new(Rank::Ace, Suit::Spade));
        hand.add(&Card::new(Rank::Ace, Suit::Club));
        hand.add(&Card::new(Rank::Ace, Suit::Diamond));
        hand.add(&Card::new(Rank::Ace, Suit::Heart));
        hand.add(&Card::new(Rank::Queen, Suit::Spade));
        hand.add(&Card::new(Rank::Queen, Suit::Club));
        hand.add(&Card::new(Rank::Queen, Suit::Diamond));
        hand.add(&Card::new(Rank::King, Suit::Heart));
        hand.add(&Card::new(Rank::King, Suit::Spade));
        hand.add(&Card::new(Rank::Jack, Suit::Club));
        hand.add(&Card::new(Rank::Ten, Suit::Diamond));
        hand.add(&Card::new(Rank::Nine, Suit::Heart));
        hand.add(&Card::new(Rank::Seven, Suit::Spade));
        hand.add(&Card::new(Rank::Four, Suit::Club));

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
    }
}

