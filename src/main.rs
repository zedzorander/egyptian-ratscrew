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

// enum for card value
#[derive(Debug, Clone, Copy, PartialEq)]
enum Rank {
    Num(u32),
    Jack,
    Queen,
    King,
    Ace,
}

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
        write!(f, "{:?} of {:?}", self.rank, self.suit)
    }
}

fn make_deck() -> Vec<Card> {
    vec![
        Card::new(Rank::Num(2),  Suit::Clubs),
        Card::new(Rank::Num(3),  Suit::Clubs),
        Card::new(Rank::Num(4),  Suit::Clubs),
        Card::new(Rank::Num(5),  Suit::Clubs),
        Card::new(Rank::Num(6),  Suit::Clubs),
        Card::new(Rank::Num(7),  Suit::Clubs),
        Card::new(Rank::Num(8),  Suit::Clubs),
        Card::new(Rank::Num(9),  Suit::Clubs),
        Card::new(Rank::Num(10), Suit::Clubs),
        Card::new(Rank::Jack,    Suit::Clubs),
        Card::new(Rank::Queen,   Suit::Clubs),
        Card::new(Rank::King,    Suit::Clubs),
        Card::new(Rank::Ace,     Suit::Clubs),
        
        Card::new(Rank::Num(2),  Suit::Spades),
        Card::new(Rank::Num(3),  Suit::Spades),
        Card::new(Rank::Num(4),  Suit::Spades),
        Card::new(Rank::Num(5),  Suit::Spades),
        Card::new(Rank::Num(6),  Suit::Spades),
        Card::new(Rank::Num(7),  Suit::Spades),
        Card::new(Rank::Num(8),  Suit::Spades),
        Card::new(Rank::Num(9),  Suit::Spades),
        Card::new(Rank::Num(10), Suit::Spades),
        Card::new(Rank::Jack,    Suit::Spades),
        Card::new(Rank::Queen,   Suit::Spades),
        Card::new(Rank::King,    Suit::Spades),
        Card::new(Rank::Ace,     Suit::Spades),
        
        Card::new(Rank::Num(2),  Suit::Diamonds),
        Card::new(Rank::Num(3),  Suit::Diamonds),
        Card::new(Rank::Num(4),  Suit::Diamonds),
        Card::new(Rank::Num(5),  Suit::Diamonds),
        Card::new(Rank::Num(6),  Suit::Diamonds),
        Card::new(Rank::Num(7),  Suit::Diamonds),
        Card::new(Rank::Num(8),  Suit::Diamonds),
        Card::new(Rank::Num(9),  Suit::Diamonds),
        Card::new(Rank::Num(10), Suit::Diamonds),
        Card::new(Rank::Jack,    Suit::Diamonds),
        Card::new(Rank::Queen,   Suit::Diamonds),
        Card::new(Rank::King,    Suit::Diamonds),
        Card::new(Rank::Ace,     Suit::Diamonds),

        Card::new(Rank::Num(2),  Suit::Hearts),
        Card::new(Rank::Num(3),  Suit::Hearts),
        Card::new(Rank::Num(4),  Suit::Hearts),
        Card::new(Rank::Num(5),  Suit::Hearts),
        Card::new(Rank::Num(6),  Suit::Hearts),
        Card::new(Rank::Num(7),  Suit::Hearts),
        Card::new(Rank::Num(8),  Suit::Hearts),
        Card::new(Rank::Num(9),  Suit::Hearts),
        Card::new(Rank::Num(10), Suit::Hearts),
        Card::new(Rank::Jack,    Suit::Hearts),
        Card::new(Rank::Queen,   Suit::Hearts),
        Card::new(Rank::King,    Suit::Hearts),
        Card::new(Rank::Ace,     Suit::Hearts),
    ]
}

/// Shuffles the deck of cards
fn shuffle_deck(mut deck: Vec<Card>) -> Vec<Card> {
    let mut rng = rand::thread_rng();
    rng.shuffle(&mut deck);
    deck
}

/// Checks if cards have same rank
fn is_equal(left: &[Card], right: &[Card]) -> bool {
    if (&left[0..1]).eq(&right[0..1]) {
        return true;
    }
    false
}

/// Top and second card have same rank
fn is_pair(pile: &Vec<Card>) -> bool {
    let (left, right) = pile.split_at(1);
    is_equal(&left[0..1], &right[0..1])
}

/// Top and third card have same rank
fn is_sandwich(pile: &Vec<Card>) -> bool {
    let (left, right) = pile.split_at(2);
    is_equal(&left[0..1], &right[0..1])
}

fn main() {
    // bind needs an address
    let mut deck: Vec<Card> = shuffle_deck(make_deck());
    let mut pile: Vec<Card> = vec![];

    //for _ in 0..deck.len() {
    for _ in 0..11 {
        pile.push(deck.remove(1));
    }

    if is_pair(&pile) {
        println!("There is a pair:");
        for i in &pile[0..2] {
            println!("{}", i);
        }
        println!("");
    }
    else if is_sandwich(&pile) {
        println!("There is a sandwich:");
        for i in &pile[0..3] {
            println!("{}", i);
        }
        println!("");
    }
    else {
        println!("There is NO pair or sandwich");
        for i in &pile[0..3] {
            println!("{}", i);
        }
        println!("");
    }
}
