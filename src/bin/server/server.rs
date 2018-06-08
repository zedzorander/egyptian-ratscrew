// MIT License
// Copyright (c) 2018 Cole Phares
// Last Modified: 5/22/2018
// Server side for online card game Egyptian RatScrew

extern crate card;
extern crate rand;
use card::{Card, Rank::*, Suit::*};
use rand::Rng;
use std::net::{TcpListener, TcpStream, SocketAddr};
use std::io::{BufReader, BufWriter, /*Write, BufRead,*/ Error};
//use std::process::Command;

struct PlayerState {
    hand: Vec<Card>,
    side_pile: Vec<Card>,
}

impl PlayerState {
    fn new() -> Self {
        PlayerState {
            hand: Vec::new(),
            side_pile: Vec::new(),
        }
    }
}

trait Player {
    fn add_to_hand(&mut self, Card) ->
        Result<(), Error>;

    fn add_to_side_pile(&mut self, &mut Vec<Card>, &mut TcpStream) ->
        Result<(), Error>;

    fn play_card(&mut self, &mut Vec<Card>, &mut TcpStream) -> 
        Result<Card, Error>;

    fn won(&mut self) -> bool;

    fn state(&self) -> &PlayerState;
}

struct HumanPlayer(PlayerState);

impl Player for HumanPlayer {
    fn add_to_hand(&mut self, card: Card) ->
        Result<(), Error> {
            self.0.hand.push(card);
            Ok(())
    }

    fn add_to_side_pile(&mut self, _pile: &mut Vec<Card>, _socket: &mut TcpStream) ->
        Result<(), Error> {
        Ok(())
    }

    fn play_card(&mut self, pile: &mut Vec<Card>, _socket: &mut TcpStream) ->
        Result<Card, Error>
    {
        /*loop {
            
        }*/
        Ok(pile.pop().unwrap())
    }

    fn won(&mut self) -> bool {
        return  self.0.hand.len() == 52
    }

    fn state(&self) -> &PlayerState {
        &self.0
    }
}


struct MachinePlayer(PlayerState);

impl Player for MachinePlayer {
    fn add_to_hand(&mut self, card: Card) ->
        Result<(), Error> {
            self.0.hand.push(card);
            Ok(())
    }
    
    fn add_to_side_pile(&mut self, _pile: &mut Vec<Card>, _socket: &mut TcpStream) ->
        Result<(), Error>{
        Ok(())
    }

    fn play_card(&mut self, pile: &mut Vec<Card>, _socket: &mut TcpStream) ->
        Result<Card, Error>
    {
        /*
        // Send top three cards of pile to server
        // send_pile(*pile, socket);

        // Add card to pile
        let card = machine.hand.pop().unwrap();
        pile.push(card);

        // Send updated top three cards of the pile to client
        send_pile(*pile, socket);

        // Create a stall (may not work because of spawn()),
        // a BufReader, and BufWriter to communicate with client
        let mut child = Command::new("sleep").arg("3").spawn().unwrap();
        let reader = BufReader::new(socket);
        let writer = BufWriter::new(socket);

        //let output = stall.wait().unwrap();
        //println!("output: {:?}", output);
        
        // If combination exists
        if test_pile(&pile) {
            // Stall and wait for client event.
            // Take pile if stall ends w/o event
            let output = stall.wait().unwrap();
            
        } else {
            // Stall and wait for client event.
            // Take pile if event happens
            let output = stall.wait().unwrap();

            // add pile to machine player hand
        }
        */
        Ok(pile.pop().unwrap())
    }

    fn won(&mut self) -> bool {
        return  self.0.hand.len() == 52
    }

    fn state(&self) -> &PlayerState {
        &self.0
    }
}
/*
fn send_pile(pile: Vec<Card>, socket: &TcpStream) {
    let mut writer = BufWriter::new(socket);
    
    // send top three cards of the pile to client
    if pile.len() == 0 {}
    else if pile.len() == 1 {
        // send card
        writeln!(writer, "Pile:").ok();
        //writer.flush();
        writeln!(writer, "{}", pile[0]).unwrap();
        writer.flush().ok();
    }
    else if pile.len() == 2 {
        // send cards
        writeln!(writer, "Pile:").ok();
        for c in 0..2 {
            // send top two cards on pile
            writeln!(writer, "{:?}", pile[c]).unwrap();
            writer.flush().ok();
        }
    }
    else {
        for c in 0..3 {
            // send top three cards on pile
            writeln!(writer, "{:?}", pile[c]).unwrap();
            writer.flush().ok();
        }
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
/*
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
*/
/*
/// Tests for different combinations
fn test_pile(pile: &Vec<Card>) -> bool {
    if pile.len() == 0 {
        return false
    }
    if is_pair(pile) {
        /*println!("There is a pair:");
        for i in &pile[0..2] {
            println!("{}", i);
        }
        println!();
        */
        return true;
    }
    // tests for a pair sandwich
    else if is_sandwich(pile) {
        /*println!("There is a sandwich:");
        for i in &pile[0..3] {
            println!("{}", i);
        }
        println!();
        */
        return true;
    }
    // tests for a sixty-nine combo
    else if is_sixty_nine(pile) {
        /*println!("There is a sixty nine:");
        for i in &pile[0..2] {
            println!("{}", i);
        }
        println!();
        */
        return true;
    }
    // tests for a sixty-nine combo
    else if is_sixty_nine_sandwich(pile) {
        /*println!("There is a sixty nine sandwich:");
        for i in &pile[0..3] {
            println!("{}", i);
        }
        println!();
        */
        return true;
    }
    // tests for a run of three cards
    else if is_run(pile) {
        /*println!("There is a run:");
        for i in &pile[0..3] {
            println!("{}", i);
        }
        println!();
        */
        return true;
    }
    false
}
*/

/// Deals the cards to the client and server hands
//fn deal_hands(deck: &mut Vec<Card>, socket: &TcpStream) -> Vec<Card> {
fn deal_hands(deck: &mut Vec<Card>, machine: &mut Player,
              human: &mut Player)  {

    // deal card to player and self
    for _ in 0..(deck.len()/2) {
        human.add_to_hand(deck.pop().unwrap()).ok();
        machine.add_to_hand(deck.pop().unwrap()).ok();
    }
}

/// Game control function
fn play_game(_socket: &TcpStream) {
    let mut deck: Vec<Card> = shuffle_deck(make_deck());
    let mut machine = MachinePlayer(PlayerState::new());
    let mut human = HumanPlayer(PlayerState::new());

    deal_hands(&mut deck, &mut machine, &mut human);

    
    println!("Machine hand:");
    for i in 0..26 {
        println!("{}", machine.state().hand[i]);
    }
    println!();
    println!("Human hand:");
    for i in 0..26 {
        println!("{}", human.state().hand[i]);
    }
    println!();
    
    /*
    loop {
        let mut _pile: Vec<Card> = Vec::new();
        let mut _reader = BufReader::new(socket);
        let mut _writer = BufWriter::new(socket);
        let mut _response = String::new();
    }
    */
    //test_pile(&pile);
}

fn main() {
    // Creates Tcp connection
    let address = ("0.0.0.0:24794").parse::<SocketAddr>().unwrap();
    let listener = TcpListener::bind(&address).unwrap();

    match listener.accept() {
        Ok((socket, _addr)) => {
            play_game(&socket);
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
