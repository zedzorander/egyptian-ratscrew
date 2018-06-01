// MIT License
// Copyright (c) 2018 Cole Phares
// Last Modified: 5/22/2018
// Server side for online card game Egyptian RatScrew

// Code to create a shuffled deck of cards borrowed and modified from
// http://cultofmetatron.io/2017/03/21/learning-rust-with-blackjack-part-1/

extern crate serde;
use serde::ser::{Serialize, Serializer, SerializeStruct};
use serde::{Deserialize, Deserializer};
use serde::de::EnumAccess;

/// Suit of the card
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

/*
/// Serializer for Suit
impl Serialize for Suit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer, {
        let mut state = serializer.serialize_struct("Suit", 1)?;
        state.serialize_field("suit", &self)?;
        state.end()
    }
}

/// Deserializer for Suit
impl<'de> Deserialize<'de> for Suit {
    fn deserialize<D>(deserialize: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        Ok(
            let s: Suit
*/

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
/*
/// Serializer for Rank
impl Serialize for Rank {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer, {
        let mut state = serializer.serialize_struct("Rank", 1)?;
        state.serialize_field("rank", &self.value())?;
        state.end()
    }
}

/// Deserializer for Rank
impl<'de> Deserialize<'de> for Rank {
    fn deserialize<D>(deserialize: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        let helper = Helper::deserialize(deserializer)?;
        
            
        //??? = helper.rank;
    }
}
*/
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
/*
/// Serializer for Card
impl Serialize for Card {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer, 
    {
        let mut state = serializer.serialize_struct("Card", 2)?;
        state.serialize_field("rank", &self.rank)?;
        state.serialize_field("suit", &self.suit)?;
        state.end()
    }
}

/// Deserializer for Card
impl<'de> Deserialize<'de> for Card {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        let helper = Deserialize::deserialize(deserializer)?;
        Ok(Card {
            rank = helper.rank;
            suit = helper.suit;
        })
    }
}
*/
/// Implements display for Card struct
impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.rank {
            Num(n) => write!(f, "{} of {:?}", n, self.suit),
            _ => write!(f, "{:?} of {:?}", self.rank, self.suit),
        }
    }
}

