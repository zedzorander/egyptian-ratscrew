// MIT License
// Copyright (c) 2018 Cole Phares
// Last Modified: 5/22/2018
// Server side for online card game Egyptian RatScrew

extern crate card;
extern crate termion;
use card::{Card, Rank::*};
use std::net::TcpStream;
use std::io::{BufReader, /*Read,*/ BufWriter, Write, BufRead, stdin, stdout};
use termion::event::Key;
use termion::raw::IntoRawMode;
use termion::input::TermRead;

/*
/// Takes the cards the server sends
fn accept_deal(mut hand: Vec<Card>, stream: &TcpStream) -> Vec<Card> {
    for _ in 0..26 {
        // read message from server
        let mut reader = BufReader::new(stream);
        let mut card_string = String::new();
        BufRead::read_line(&mut reader, &mut card_string).ok().expect("Connection lost");
        
        let card = Card::parse_card(card_string);

        hand.push(card);
    }
    hand
}
*/
/// Players game control
//fn play_game(hand: &mut Vec<Card>, stream: &TcpStream) {
fn play_game(stream: &TcpStream) {
    let mut reader = BufReader::new(stream);
    let mut _writer = BufWriter::new(stream);
    let mut message = String::new();
    
    // Print out hand
    /*for i in hand {
        print!("{}\r\n", i);
    }*/

    //while !hand.is_empty() || hand.len() != 52 {
        BufRead::read_line(&mut reader, &mut message);
        // Set up standard input for event handling
        let stdin = stdin();
        let mut stdout = stdout().into_raw_mode().unwrap();
        
        for c in stdin.keys() {
            match c.unwrap() {
                Key::Char(' ') => {
                    write!(stdout, "You hit the space key.\r\n").ok();
                    stdout.flush().unwrap();
                },
                Key::Char('q') => return,
                _ => println!("No key was pressed")
            }
        }
    //}
}

fn main() {
    let mut input = String::new();
    let mut valid_input = false;

    while !valid_input {
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
    
    // Connect to the server
    if let Ok(stream) = TcpStream::connect("127.0.0.1:24794") {
        //let mut hand: Vec<Card> = Vec::new();
        //hand = accept_deal(hand, &stream);
        //play_game(&mut hand, &stream);

        play_game(&stream);

    } else {
        println!("Couldn't connect to server...");
    }
    
}
