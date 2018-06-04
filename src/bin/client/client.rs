// MIT License
// Copyright (c) 2018 Cole Phares
// Last Modified: 5/22/2018
// Server side for online card game Egyptian RatScrew

extern crate card;
extern crate termion;
use card::{Card, Rank::*, Suit, Suit::*};
use std::net::TcpStream;
use std::io::{BufReader, Read, BufWriter, Write, BufRead, stdin, stdout};
use termion::event::Key;
use termion::raw::IntoRawMode;
//use termion::input::TermRead;

fn get_suit(suit: String) -> Suit {
    match suit.trim().to_string().as_ref() {
        "Hearts" => Hearts,
        "Diamonds" => Diamonds,
        "Spades" => Spades,
        "Clubs" => Clubs,
        _ => panic!("Unexpected Suit {}", suit)
    }
}

fn accept_deal(mut hand: Vec<Card>, stream: &TcpStream) -> Vec<Card> {
    for _ in 0..26 {
        // read message from server
        let mut reader = BufReader::new(stream);
        let mut response = String::new();
        reader.read_line(&mut response).ok();
        //println!("Received: {}", response);

        let mut v: Vec<&str> = response.split(", ").collect();
        let suit: String = v.pop()
                            .unwrap()
                            .to_string();
        let rank: u32 = v.pop()
                         .unwrap()
                         .parse::<u32>()
                         .unwrap();

        let _card: Card;

        if rank > 1 && rank <= 10 {
            let card = Card::new(Num(rank), get_suit(suit));
            //println!("Card: {:?}", card);
            hand.push(card);
        }else {
            let card = match rank {
                11 => Card::new(Jack, get_suit(suit)),
                12 => Card::new(Queen, get_suit(suit)),
                13 => Card::new(King, get_suit(suit)),
                1 => Card::new(Ace, get_suit(suit)),
                _ => panic!("Unexpected Rank {}", rank)
            };
            //println!("Card: {:?}", card);
            hand.push(card);
        }

        
        let mut writer = BufWriter::new(stream);
        writer.write_all(b"client says hello\n").ok();
    }
    hand
}

fn main() {
    // connect to server.rs
    if let Ok(stream) = TcpStream::connect("127.0.0.1:24794") {
        let mut hand: Vec<Card> = Vec::new();
        
        hand = accept_deal(hand, &stream);

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
            println!("{}", i);
        }
    } else {
        println!("Couldn't connect to server...");
    }
    
}
