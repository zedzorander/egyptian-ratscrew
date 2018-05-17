// MIT License 
// Copyright (c) 2018 Cole Phares
// Last Modified: 5/16/2018
// Server side for online card game Egyptian RatScrew

// Code to create a shuffled deck of cards borrowed from
// http://cultofmetatron.io/2017/03/21/learning-rust-with-blackjack-part-1/
// version notes: added ability to test for multiple scenarios (pair and
// sandwich)

extern crate rand;
use rand::Rng;
// Suit of the card
#[derive(Debug, Clone, Copy)]
enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}
use Suit::*;

// enum for card value
#[derive(Debug, Clone, Copy, PartialEq)]
enum Rank {
    Num(u32),
    Jack,
    Queen,
    King,
    Ace,
}
use Rank::*;

// Struct for playing card
#[derive(Debug, Clone, Copy)]
struct Card {
    rank: Rank,
    suit: Suit,
}

// Comparison for cards by rank, ignoring suit
impl PartialEq for Card {
    fn eq(&self, other: &Card) -> bool {
        self.rank == other.rank
    }
}

// Create a new card
impl Card {
    fn new(rank: Rank, suit: Suit) -> Card {
        Card {
            rank: rank,
            suit: suit,
        }
    }
}

// Implements display for Card struct
impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.rank {
            Num(n) => write!(f, "{} of {:?}", n, self.suit),
            _ => write!(f, "{:?} of {:?}", self.rank, self.suit),
        }
    }
}

// Build a fresh deck.
fn make_deck() -> Vec<Card> {
    let mut deck = Vec::new();
    for suit in [Clubs, Diamonds, Hearts, Spades].iter() {
        for number in 2..11 {
            deck.push(Card::new(Num(number), *suit));
        }
        for face in [Jack, Queen, King, Ace].iter() {
            deck.push(Card::new(*face, *suit));
        }
    }
    deck
}

/// Shuffles the deck of cards
fn shuffle_deck(mut deck: Vec<Card>) -> Vec<Card> {
    let mut rng = rand::thread_rng();
    rng.shuffle(&mut deck);
    deck
}

/// Cards have same rank.
fn is_equal(left: Card, right: Card) -> bool {
    left.rank == right.rank
}

/// Top and second card have same rank
fn is_pair(pile: &Vec<Card>) -> bool {
    is_equal(pile[0], pile[1])
}

/// Top and third card have same rank
fn is_sandwich(pile: &Vec<Card>) -> bool {
    is_equal(pile[0], pile[2])
}

fn main() {
    let mut deck: Vec<Card> = shuffle_deck(make_deck());
    let mut pile: Vec<Card> = Vec::new();

    for _ in 0..11 {
        pile.push(deck.pop().unwrap());
    }

    if is_pair(&pile) {
        println!("There is a pair:");
        for i in &pile[0..2] {
            println!("{}", i);
        }
        println!();
    }
    else if is_sandwich(&pile) {
        println!("There is a sandwich:");
        for i in &pile[0..3] {
            println!("{}", i);
        }
        println!();
    }
    else {
        println!("There is NO pair or sandwich");
        for i in &pile[0..3] {
            println!("{}", i);
        }
        println!();
    }
}
