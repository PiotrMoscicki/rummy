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
    Ace = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13 
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
    pub fn new() -> Deck { return Deck { cards: vec!() }; }

    pub fn cards(&self) -> &Vec<Card> { return &self.cards; }
    pub fn len(&self) -> usize { return self.cards.len(); }
    pub fn push(&mut self, card: &Card) { self.cards.push(*card); }
}

impl Index<usize> for Deck {
    type Output = Card;

    fn index(&self, idx: usize) -> &Self::Output {
        return &self.cards[idx];
    }
}

//------------------------------------------------------------------------------------------------
pub fn find_set(card_idx: usize, deck: &Deck) -> Vec<usize> {
    let rank = deck[card_idx].rank;
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
pub fn find_run(card_idx: usize, deck: &Deck) -> Vec<usize> {
    let element = deck[card_idx];
    let mut sorted_same_suit_indices: Vec<usize> = vec!();

    for idx in 0.. deck.len() {
        if deck[idx].suit == element.suit {
            let dst 
                = sorted_same_suit_indices.iter().position(|&x| deck[x].rank > deck[idx].rank);

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
        match indices.last() {
            Some(prev_idx) => {
                if deck[*prev_idx].rank as usize + 1 != deck[idx].rank as usize {
                    if indices.iter().find(|x| **x == card_idx).is_some() {
                        break;
                    }
                    indices.clear();
                }
            },
            None => {}
        }

        indices.push(idx);
    }

    if indices.len() < 3 {
        indices.clear();
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

        let aces = find_set(0, &hand);
        assert_eq!(aces.len(), 4);
        assert_eq!(aces[0], 0);
        assert_eq!(aces[1], 1);
        assert_eq!(aces[2], 2);
        assert_eq!(aces[3], 3);

        let queens = find_set(4, &hand);
        assert_eq!(queens.len(), 3);
        assert_eq!(queens[0], 4);
        assert_eq!(queens[1], 5);
        assert_eq!(queens[2], 6);

        let kings = find_set(7, &hand);
        assert_eq!(kings.len(), 0);

        let jacks = find_set(9, &hand);
        assert_eq!(jacks.len(), 0);
    }

    //--------------------------------------------------------------------------------------------
    #[test]
    fn find_run_test() {
        {
            let mut hand = Deck::new();
            hand.push(&Card::new(Rank::Ace, Suit::Spade));
            hand.push(&Card::new(Rank::Two, Suit::Spade));
            hand.push(&Card::new(Rank::Three, Suit::Spade));

            let run = find_run(0, &hand);
            assert_eq!(run.len(), 3);
            assert_eq!(run[0], 0);
            assert_eq!(run[1], 1);
            assert_eq!(run[2], 2);
        }
        {
            let mut hand = Deck::new();
            hand.push(&Card::new(Rank::Ace, Suit::Spade));
            hand.push(&Card::new(Rank::Four, Suit::Spade));
            hand.push(&Card::new(Rank::Three, Suit::Spade));

            let run = find_run(0, &hand);
            assert_eq!(run.len(), 0);
        }
        {
            let mut hand = Deck::new();
            hand.push(&Card::new(Rank::Ace, Suit::Spade));
            hand.push(&Card::new(Rank::Two, Suit::Club));
            hand.push(&Card::new(Rank::Three, Suit::Spade));

            let run = find_run(0, &hand);
            assert_eq!(run.len(), 0);
        }
        {
            let mut hand = Deck::new();
            hand.push(&Card::new(Rank::Ace, Suit::Spade));
            hand.push(&Card::new(Rank::Three, Suit::Spade));
            hand.push(&Card::new(Rank::Two, Suit::Spade));

            let run = find_run(0, &hand);
            assert_eq!(run.len(), 3);
            assert_eq!(run[0], 0);
            assert_eq!(run[1], 2);
            assert_eq!(run[2], 1);
        }
        {
            let mut hand = Deck::new();
            hand.push(&Card::new(Rank::Queen, Suit::Heart));
            hand.push(&Card::new(Rank::Ace, Suit::Spade));
            hand.push(&Card::new(Rank::Three, Suit::Spade));
            hand.push(&Card::new(Rank::Two, Suit::Spade));
            hand.push(&Card::new(Rank::Seven, Suit::Spade));
            hand.push(&Card::new(Rank::Six, Suit::Club));

            let run = find_run(1, &hand);
            assert_eq!(run.len(), 3);
            assert_eq!(run[0], 1);
            assert_eq!(run[1], 3);
            assert_eq!(run[2], 2);
        }
    }
}

