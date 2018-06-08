// MIT License
// Copyright (c) 2018 Cole Phares
// Last Modified: 5/22/2018
// Server side for online card game Egyptian RatScrew

// Code to create a shuffled deck of cards borrowed and modified from
// http://cultofmetatron.io/2017/03/21/learning-rust-with-blackjack-part-1/

/// Suit of the card
#[derive(Debug, Clone, Copy, Eq)]
pub enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

use::Suit::*;

impl Suit {
    // returns a string of the suit
    pub fn value(self) -> String {
        match self {
            Hearts => "Hearts".to_string(),
            Diamonds => "Diamonds".to_string(),
            Spades => "Spades".to_string(),
            Clubs => "Clubs".to_string(),
        }
    }
}

impl PartialEq<Suit> for Suit {
    fn eq(&self, other: &Suit) -> bool {
        self.value() == other.value()
    }
}

/// enum for card value
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rank {
    Num(u32),
    Jack,
    Queen,
    King,
    Ace,
}

use Rank::*;

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
    
    /// creates a string of the card of the form "(rank, suit)"
    pub fn card_to_string(card: Card) -> String{
        // add rank
        let mut card_string = card.rank.value().to_string();
        card_string.push_str(", ");
    
        // add suit
        card_string.push_str(&card.suit.value());

        card_string
    }

    /// Determines the suit of the card
    pub fn get_suit(suit: String) -> Suit {
        match suit.trim().to_string().as_ref() {
            "Hearts" => Hearts,
            "Diamonds" => Diamonds,
            "Spades" => Spades,
            "Clubs" => Clubs,
            _ => panic!("Unexpected Suit {}", suit)
        }
    }

    /// Returns a card from a String
    pub fn parse_card(card_string: String) -> Card {
        // Gets the suit and rank in that order
        let mut v: Vec<&str> = card_string.split(", ").collect();
        let suit: String = v.pop()
                            .unwrap()
                            .to_string();
        let rank: u32 = v.pop()
                         .unwrap()
                         .parse::<u32>()
                         .unwrap();

        let card: Card;
        
        // Adds the card to players hand
        if rank > 1 && rank <= 10 {
            card = Card::new(Num(rank), Card::get_suit(suit));
        }else {
            card = match rank {
                11 => Card::new(Jack, Card::get_suit(suit)),
                12 => Card::new(Queen, Card::get_suit(suit)),
                13 => Card::new(King, Card::get_suit(suit)),
                1 => Card::new(Ace, Card::get_suit(suit)),
                _ => panic!("Unexpected Rank {}", rank)
            };
        }
        card
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

