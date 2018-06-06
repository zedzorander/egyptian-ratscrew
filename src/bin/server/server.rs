// MIT License
// Copyright (c) 2018 Cole Phares
// Last Modified: 5/22/2018
// Server side for online card game Egyptian RatScrew

extern crate card;
extern crate rand;
use card::{Card, Rank::*, Suit::*};
use rand::Rng;
use std::net::{TcpListener, TcpStream, SocketAddr};
use std::io::{BufReader, BufWriter, Write, BufRead};
use std::process::Command;
/*
struct Server<P: Fn(Vec<Card>, &mut Vec<Card>, &TcpStream) -> Vec<Card>> {
    play_card: P,
}

fn send_pile(pile: Vec<Card>, socket: &TcpStream) {
    let mut writer = BufWriter::new(socket);
    
    // send top three cards of the pile to client
    if pile.len() == 0 {}
    else if pile.len() == 1 {
        // send card
        writeln!(writer, "Pile:");
        writer.flush();
        writeln!(writer, "{}", pile[0]).unwrap();
        writer.flush();
    }
    else if pile.len() == 2 {
        // send cards
        writeln!(writer, "Pile:");
        for c in 0..2 {
            // send top two cards on pile
            writeln!(writer, "{:?}", pile[c]).unwrap();
            writer.flush();
        }
    }
    else {
        for c in 0..3 {
            // send top three cards on pile
            writeln!(writer, "{:?}", pile[c]).unwrap();
            writer.flush();
        }
    }
}

impl<P: Fn(Vec<Card>, &mut Vec<Card>, &TcpStream) -> Vec<Card>> Server<P> 
{
    fn play_card(&self, mut pile: Vec<Card>, hand: &mut Vec<Card>, 
                 socket: &TcpStream) -> Vec<Card> {
        let card = hand.pop().unwrap();

        // send top three cards on the pile to client
        send_pile(pile, socket);

        // add card to pile
        pile.push(card);

        // stall
        let mut child = Command::new("sleep").arg("3").spawn().unwrap();
        let _result = child.wait().unwrap();

        // send updated top three cards of the pile to client
        send_pile(pile, socket);

        // if combination exists
        if test_pile(&pile) {
            // stall and wait for client event (take pile if stall ends w/o event)
            let _result = child.wait().unwrap();
        }
        else {
            // stall and wait for client event (take pile if event happens)
            let _result = child.wait().unwrap();
        }

        pile
    }
}
*/
/*
struct Client<P: Fn(&mut Vec<Card>, BufReader, &mut BufWriter) -> Vec<Card>> {
    play_card: P,
}

impl<P: Fn(&mut Vec<Card>, BufReader, &mut BufWriter) -> Vec<Card>> Server<P> {
    fn play_card(&self, pile: &mut Vec<Card>, reader: &BufReader,
                 writer: &mut BufWriter) -> Vec<Card> {
        // send top three cards of the pile to client
        for c in &pile.len()..0 {
            println!("card: {:?}", c);
        }

        // receive card from client
    

        // add card to pile


        // send updated top three cards of the pile to client


        // test pile for combinations


        // if combination exists


        // else

    }
}
*/
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

/// Shuffles the deck of cards a random amount of times
/// between 5 and 15
fn shuffle_deck(mut deck: Vec<Card>) -> Vec<Card> {
    let rand: usize = rand::thread_rng().gen_range(5, 15);
    for _ in 0..rand {
        rand::thread_rng().shuffle(&mut deck);
    }
    deck
}

// COMBINATION TESTS BELOW HERE

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
        if one_two && two_three {
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
fn test_pile(pile: &Vec<Card>) -> bool {
    if pile.len() == 0 {
        return false
    }
    if is_pair(pile) {
        println!("There is a pair:");
        for i in &pile[0..2] {
            println!("{}", i);
        }
        println!();
        return true;
    }
    // tests for a pair sandwich
    else if is_sandwich(pile) {
        println!("There is a sandwich:");
        for i in &pile[0..3] {
            println!("{}", i);
        }
        println!();
    }
    // tests for a sixty-nine combo
    else if is_sixty_nine(pile) {
        println!("There is a sixty nine:");
        for i in &pile[0..2] {
            println!("{}", i);
        }
        println!();
    }
    // tests for a sixty-nine combo
    else if is_sixty_nine_sandwich(pile) {
        println!("There is a sixty nine sandwich:");
        for i in &pile[0..3] {
            println!("{}", i);
        }
        println!();
    }
    // tests for a run of three cards
    else if is_run(pile) {
        println!("There is a run:");
        for i in &pile[0..3] {
            println!("{}", i);
        }
        println!();
    }
    false
}


/// Deals the cards to the client and server hands
fn deal_hands(deck: &mut Vec<Card>, socket: &TcpStream) -> Vec<Card> {
    let mut hand: Vec<Card> = Vec::new();

    // deal card to player and self
    for _ in 0..(deck.len()/2) {
        
        // get card from pile
        let card: Card = deck.pop().unwrap();
        println!("sending {:?}", card);
        // send card to client
        let mut writer = BufWriter::new(socket);
        
        writeln!(writer, "{}\n", Card::card_to_string(card)).unwrap();
        writer.flush().ok();
        
        hand.push(deck.pop().unwrap());
    }
    hand
}
/*
/// Adds server card to the pile
fn servers_turn() {

}

/// Waits for card from client to add to pile
fn clients_turn(pile: mut Vec<Card>, reader: &mut BufReader
                writer: &mut BufWriter) -> Vec<Card> {

}
*/

/// Game control function
fn play_game(hand: &mut Vec<Card>, socket: &TcpStream) {
    let mut pile: Vec<Card> = Vec::new();
    //let client: Client;
    let mut reader = BufReader::new(socket);
    let mut writer = BufWriter::new(socket);
    let mut response = String::new();

    //pile = Server.play_card(pile, hand, socket);
    // Randomly choose first player (if time allows)


    // client plays first
    /*while !hand.is_empty() || hand.len() != 52 {
        reader.read_line(&mut response);
    }*/
    
    // get response from client
    // use when figuring out event handler
    /*let mut reader = BufReader::new(socket);
    let mut message = String::new();
    match reader.read_line(&mut message) {
        Ok(_) => {
            //println!("message from client: {}", message);
        }
        Err(e) => {
            println!("Error reading message: {:?}", e);
        }
    }*/
}

fn main() {
    let mut deck: Vec<Card> = shuffle_deck(make_deck());
    let mut pile: Vec<Card> = Vec::new();
    
    // Creates Tcp connection
    let address = ("0.0.0.0:24794").parse::<SocketAddr>().unwrap();
    let listener = TcpListener::bind(&address).unwrap();

    match listener.accept() {
        Ok((socket, _addr)) => {
            let mut hand = deal_hands(&mut deck, &socket);
            
            //play_game(&mut hand, &socket);
            //pile.push(deck.pop().unwrap());
            test_pile(&pile);
            
            for i in &hand {
                println!("{}", i);
            }
        }
        Err(e) => {
            println!("Error {}", e);
        }
    }
}


#[cfg(test)]
mod tests {
    extern crate card;
    use card::{Card, Rank, Rank::*, Suit, Suit::*};

    #[test]
    fn test_card_creation() {
        let card = Card::new(Num(6), Suit::Clubs);

        assert_eq!(card.rank.value(), 6);
        assert_eq!(card.suit, Suit::Clubs);
    }

    #[test]
    fn test_different_rank() {
        let card = Card::new(Num(6), Suit::Clubs);

        assert_ne!(card.rank.value(), 11);
    }

    #[test]
    fn test_different_suit() {
        let card = Card::new(Num(6), Suit::Clubs);

        assert_ne!(card.suit, Suit::Diamonds);
    }
}
