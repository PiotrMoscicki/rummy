use std::vec::Vec;

//------------------------------------------------------------------------------------------------
#[derive(Debug, Copy, Clone)]
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
#[derive(Debug, Copy, Clone)]
pub enum Suit {
    Club,
    Spade,
    Diamond,
    Heart
}

//------------------------------------------------------------------------------------------------
#[derive(Debug, Copy, Clone)]
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
    pub fn cards(&self) -> &Vec<Card> { return &self.m_cards; }

    pub fn add(&mut self, card: &Card) {
        self.m_cards.push(*card);
    }
}

//------------------------------------------------------------------------------------------------
fn find_four_of_kind(kind: Kind, deck: &Deck) -> Vec<usize> {
    Vec<usize> indices;
    
    for idx in 0.. deck.cards().len() {
        if card.kind == kind {
            indices.add(idx);
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
    fn find_four_of_kind() {
        let hand : Deck;
        hand.add( &Card { m_rank: Rank::Ace, m_suit: Suit::Spade } );
        assert_eq!(4, 4);
    }
}

