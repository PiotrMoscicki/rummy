use std::vec::Vec;

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
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Suit {
    Club,
    Spade,
    Diamond,
    Heart
}

//------------------------------------------------------------------------------------------------
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Card {
    pub m_rank: Rank,
    pub m_suit: Suit
}

//------------------------------------------------------------------------------------------------
#[derive(Debug, Clone)]
pub struct Deck {
    m_cards: Vec<Card>
}

//------------------------------------------------------------------------------------------------
impl Deck {
    pub fn new() -> Deck {
        return Deck {
            m_cards: vec!()
        };
    }

    pub fn cards(&self) -> &Vec<Card> { return &self.m_cards; }

    pub fn add(&mut self, card: &Card) {
        self.m_cards.push(*card);
    }
}

//------------------------------------------------------------------------------------------------
pub fn find_same_rank(rank: Rank, deck: &Deck) -> Vec<std::primitive::usize> {
    let mut indices : Vec<std::primitive::usize> = vec!();
    
    for idx in 0.. deck.cards().len() {
        if deck.cards()[idx].m_rank == rank {
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
        hand.add( &Card { m_rank: Rank::Ace, m_suit: Suit::Spade } );
        assert_eq!(4, 4);
    }
}

