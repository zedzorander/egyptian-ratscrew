// MIT License
// Copyright (c) 2018 Cole Phares
// Last Modified: 5/22/2018
// Server side for online card game Egyptian RatScrew

// Code to create a shuffled deck of cards borrowed and modified from
// http://cultofmetatron.io/2017/03/21/learning-rust-with-blackjack-part-1/

// Code to create a shuffled deck of cards borrowed and modified from
// http://cultofmetatron.io/2017/03/21/learning-rust-with-blackjack-part-1/

/// Suit of the card
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

//use Suit::*;

/// enum for card value
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rank {
    Num(u32),
    Jack,
    Queen,
    King,
    Ace,
}

/// Gives the Rank enum a value for ordering
impl Rank {
    pub fn value(self) -> u32 {
        match self {
            Num(n) => n,
            Jack => 11,
            Queen => 12,
            King => 13,
            Ace => 1,
        }
    }
}

use Rank::*;

/// Struct for playing card
#[derive(Debug, Clone, Copy, Eq)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

/// Comparison for cards by rank, ignoring suit
impl PartialEq<Card> for Card {
    fn eq(&self, other: &Card) -> bool {
        self.rank == other.rank
    }
}

/// Comparison for cards to a rank value
impl PartialEq<u32> for Card {
    fn eq(&self, other: &u32) -> bool {
        self.rank.value() == *other
    }
}

/// Create a new card
impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Card {
        Card {
            rank: rank,
            suit: suit,
        }
    }
}

/// Implements display for Card struct
impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.rank {
            Num(n) => write!(f, "{} of {:?}", n, self.suit),
            _ => write!(f, "{:?} of {:?}", self.rank, self.suit),
        }
    }
}

