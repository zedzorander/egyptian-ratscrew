// MIT License
// Copyright (c) 2018 Cole Phares
// Last Modified: 5/22/2018
// Server side for online card game Egyptian RatScrew

extern crate card;
extern crate termion;
use card::{Card, Rank::*};
use std::net::TcpStream;
use std::io::{BufReader, /*Read, BufWriter, Write,*/ BufRead};//, stdin, stdout};
//use termion::event::Key;
//use termion::raw::IntoRawMode;
//use termion::input::TermRead;


/// Takes the cards the server sends
fn accept_deal(mut hand: Vec<Card>, stream: &TcpStream) -> Vec<Card> {
    for _ in 0..26 {
        // read message from server
        let mut reader: &mut BufRead = &mut BufReader::new(stream);
        let mut response = String::new();
        reader.read_line(&mut response).ok();
        //println!("Received: {}", response);

        // Gets the suit and rank in that order
        let mut v: Vec<&str> = response.split(", ").collect();
        let suit: String = v.pop()
                            .unwrap()
                            .to_string();
        let rank: u32 = v.pop()
                         .unwrap()
                         .parse::<u32>()
                         .unwrap();

        let _card: Card;
        
        // Adds the card to players hand
        if rank > 1 && rank <= 10 {
            let card = Card::new(Num(rank), Card::get_suit(suit));
            hand.push(card);
        }else {
            let card = match rank {
                11 => Card::new(Jack, Card::get_suit(suit)),
                12 => Card::new(Queen, Card::get_suit(suit)),
                13 => Card::new(King, Card::get_suit(suit)),
                1 => Card::new(Ace, Card::get_suit(suit)),
                _ => panic!("Unexpected Rank {}", rank)
            };
            hand.push(card);
        }
        /*let mut writer = BufWriter::new(stream);
        writer.write_all(b"client says hello\n").ok();
        writer.flush().ok();
        */
    }
    hand
}
/*
/// Players game control
fn play_game(hand: &mut Vec<Card>, stream: &TcpStream) {
    let mut reader = BufReader::new(stream);
    let mut writer = BufWriter::new(stream);
    let mut response = String::new();

    /*while !hand.is_empty() || hand.len() != 52 {
    
    }
    */
}
*/
fn main() {
    //let mut input = String::new();
    //let mut valid_input = false;

    /*while !valid_input {
        // Print welcome prompt
        println!("Welcome to Egyptian Ratscrew!!\nPress p to play\nPress q to quit: ");
        stdin().read_line(&mut input).ok();
        let prompt = input.bytes().nth(0);
        match prompt.unwrap() as char {
            'p' => valid_input = true,
            'q' => panic!(),
            _ => println!("Incorrect character! Please try again.")
        }
    }
    println!("outside of prompt section"); 
    */
    // Connect to the server
    if let Ok(stream) = TcpStream::connect("127.0.0.1:24794") {
        let mut hand: Vec<Card> = Vec::new();
        
        hand = accept_deal(hand, &stream);
        
        let reader: &mut BufRead = &mut BufReader::new(&stream);
        
        let mut response = String::new();
        reader.read_line(&mut response).unwrap();
        println!("Received: {}", response);
        
        // Trying to get event handling working
        //let mut stdin = stdin();
        //let mut stdout = stdout().into_raw_mode().unwrap();

        //write!(stdout, "press space: ");
        //stdout.flush().unwrap();

        /*for c in stdin.keys() {
            match c.unwrap() {
                Key::Char(' ') => {
                    write!(stdout, "You hit the {:?} key.", c);
                    stdout.flush().unwrap();
                },
                _ => println!("No key was pressed")
            }
        }*/

        for i in &hand {
            print!("{}\r\n", i);
        }
    } else {
        println!("Couldn't connect to server...");
    }
    
}
