// MIT License 
// Copyright (c) 2018 Cole Phares
// Last Modified: 5/17/2018
// Server side for online card game Egyptian RatScrew

// Code to create a shuffled deck of cards borrowed and modified from
// http://cultofmetatron.io/2017/03/21/learning-rust-with-blackjack-part-1/
// version notes: added function to determine a sixty nine pair

extern crate rand;
use rand::Rng;
/// Suit of the card
#[derive(Debug, Clone, Copy)]
enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}
use Suit::*;

use Suit::*;

/// enum for card value
#[derive(Debug, Clone, Copy, PartialEq)]
enum Rank {
    Num(u32),
    Jack,
    Queen,
    King,
    Ace,
}
use Rank::*;

use Rank::*;

/// Struct for playing card
#[derive(Debug, Clone, Copy)]
struct Card {
    rank: Rank,
    suit: Suit,
}

/// Comparison for cards by rank, ignoring suit
impl PartialEq for Card {
    fn eq(&self, other: &Card) -> bool {
        self.rank == other.rank
    }
}

/// Create a new card
impl Card {
    fn new(rank: Rank, suit: Suit) -> Card {
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

/// Creates a deck of cards
fn make_deck() -> Vec<Card> {
    let mut deck: Vec<Card> = Vec::new();

    for suit in [Hearts, Diamonds, Clubs, Spades].iter() {
        for i in 2..11 {
            deck.push(Card::new(Num(i), *suit));
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

/// Top and second card have same rank
fn is_pair(pile: &Vec<Card>) -> bool {
    pile[0] == pile[1]
}

/// Top and third card have same rank
fn is_sandwich(pile: &Vec<Card>) -> bool {
    pile[0] == pile[2]
}

const SIX: Card = Card{ rank: Num(6), suit: Clubs };
const NINE: Card = Card{ rank: Num(9), suit: Clubs };

/// Checks if left and right cards form (6, 9) pairing
fn is_sixty_nine_match(left: Card, right: Card) -> bool {
    if left == SIX && right == NINE {
        return true;
    }
    if right == SIX && left == NINE {
        return true;
    }
    false
}

/// Top card and second card have ranks of 6 && 9 or 9 && 6
fn is_sixty_nine(pile: &Vec<Card>) -> bool {
    is_sixty_nine_match(pile[0], pile[1])
}

/// Top card and third card have ranks 6 && 9 or 9 && 6
fn is_sixty_nine_sandwich(pile: &Vec<Card>) -> bool {
    is_sixty_nine_match(pile[0], pile[2])
}

fn test_pile(pile: Vec<Card>) {
    // temp pile to test is_sixty_nine functon
    let mut temp: Vec<Card> = Vec::new();
    temp.push(Card::new(Num(6), Hearts));
    temp.push(Card::new(Num(9), Diamonds));

    let mut temp2: Vec<Card> = Vec::new();
    temp2.push(Card::new(Num(6), Hearts));
    temp2.push(Card::new(Jack, Spades));
    temp2.push(Card::new(Num(9), Diamonds));

    if is_pair(&pile) {
        println!("There is a pair:");
        for i in &pile[0..2] {
            println!("{}", i);
        }
        println!();
    }
    if is_sandwich(&pile) {
        println!("There is a sandwich:");
        for i in &pile[0..3] {
            println!("{}", i);
        }
        println!();
    }
    if is_sixty_nine(&temp) {
        println!("There is a sixty nine:");
        for i in &temp[0..2] {
            println!("{}", i);
        }
        println!();
    }
    if is_sixty_nine_sandwich(&temp2) {
        println!("There is a sixty nine sandwich:");
        for i in &temp2[0..3] {
            println!("{}", i);
        }
        println!();
    }
}

fn main() {
    let mut deck: Vec<Card> = shuffle_deck(make_deck());
    let mut pile: Vec<Card> = Vec::new();

    for _ in 0..11 {
        pile.push(deck.pop().unwrap());
    }
    test_pile(pile);
}
