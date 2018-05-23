// MIT License 
// Copyright (c) 2018 Cole Phares
// Last Modified: 5/17/2018
// Server side for online card game Egyptian RatScrew

// Code to create a shuffled deck of cards borrowed and modified from
// http://cultofmetatron.io/2017/03/21/learning-rust-with-blackjack-part-1/

extern crate rand;
use rand::Rng;

/// Suit of the card
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

use Suit::*;

/// enum for card value
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Rank {
    Num(u32),
    Jack,
    Queen,
    King,
    Ace,
}

/// Gives the Rank enum a value for ordering
impl Rank {
    fn value(self)-> u32 {
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
struct Card {
    rank: Rank,
    suit: Suit,
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

/// COMBINATION TESTS BELOW HERE

/// Top and second card have same rank
fn is_pair(pile: &Vec<Card>) -> bool {
    pile[0] == pile[1]
}

/// Top and third card have same rank
fn is_sandwich(pile: &Vec<Card>) -> bool {
    pile[0] == pile[2]
}

/// Checks if left and right cards form (6, 9) pairing
fn is_sixty_nine_match(left: Card, right: Card) -> bool {
    if left == 6 && right == 9 {
        return true;
    }
    if right == 6 && left == 9 {
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

/// Determines if any of the three cards form a pair
/// returns true if their is a pair
fn find_pair_run(left: Card, middle: Card, right: Card) -> bool {
    if left == middle || left == right || middle == right {
        return true;
    }
    false
}

/// Determines if the ranks of cards differ by one
fn find_abs(left: Card, right: Card) -> bool {
    let value = left.rank.value() as i32 - right.rank.value() as i32;
    if value.abs() == 1 {
        return true;
    }
    false
}

/// Top three cards form a run in any order
fn is_run(pile: &Vec<Card>) -> bool {
    let left = pile[0];
    let middle = pile[1];
    let right = pile[2];

    // If no cards have equal rank, search for a run
    if !find_pair_run(left, middle, right) {
        let one_two = find_abs(left, middle);
        let one_three = find_abs(left, right);
        let two_three = find_abs(middle, right);

        // If the first and second cards differ by one rank
        // and the second and third cards differ by one rank
        // (e.g. 3->4->5)
        if one_two && two_three  {
            return true;
        }

        // If the first and third cards differ by one rank
        // and the second and third cards differ by one rank
        // (e.g. 3->5->4)
        if one_three && two_three {
            return true;
        }

        // If the first and second cards differ by one rank
        // and the first and third card differ by one rank
        // (e.g. 4->5->3)
        if one_two && one_three {
            return true;
        }
    }
    false
}

/// Tests for different combinations
fn test_pile(pile: Vec<Card>) {
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
    if is_sixty_nine(&pile) {
        println!("There is a sixty nine:");
        for i in &pile[0..2] {
            println!("{}", i);
        }
        println!();
    }
    if is_sixty_nine_sandwich(&pile) {
        println!("There is a sixty nine sandwich:");
        for i in &pile[0..3] {
            println!("{}", i);
        }
        println!();
    }
    if is_run(&pile) {
        println!("There is a run:");
        for i in &pile[0..3] {
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
