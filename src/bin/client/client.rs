// MIT License
// Copyright (c) 2018 Cole Phares
// Last Modified: 5/22/2018
// Server side for online card game Egyptian RatScrew

extern crate card;
use card::{Card, Rank::*, Suit, Suit::*};
use std::net::TcpStream;
use std::io::{BufReader, BufWriter, Write, BufRead};

fn get_suit(suit: String) -> Suit {
    match suit.trim().to_string().as_ref() {
        "Hearts" => Hearts,
        "Diamonds" => Diamonds,
        "Spades" => Spades,
        "Clubs" => Clubs,
        _ => panic!("Unexpected Suit {}", suit)
    }
}

fn main() {
    // connect to server.rs
    if let Ok(stream) = TcpStream::connect("127.0.0.1:24794") {
        let mut hand: Vec<Card> = Vec::new();

        // read message from server
        let mut reader = BufReader::new(&stream);
        let mut response = String::new();
        reader.read_line(&mut response);
        
        // get rank from response
        let rank: u32 = response.chars()
                                .nth(1)
                                .unwrap() as u32 - '0' as u32;
        // get suit from response
        let suit: String = response.chars()
                                   .skip(4)
                                   .filter(|x| *x != ')' && *x != '\n')
                                   .collect();

        println!("Card: ({}, {})", rank, suit);
        
        let card: Card;

        if rank <= 10 {
            let card = Card::new(Num(rank), get_suit(suit));
            println!("Card: {:?}", card);
            hand.push(card);
        }else {
            let card = match rank {
                11 => Card::new(Jack, get_suit(suit)),
                12 => Card::new(Queen, get_suit(suit)),
                13 => Card::new(King, get_suit(suit)),
                1 => Card::new(Ace, get_suit(suit)),
                _ => panic!("Unexpected Rank {}", rank)
            };
            println!("Card: {:?}", card);
            hand.push(card);
        }

        
        let mut writer = BufWriter::new(&stream);
        writer.write_all(b"client says hello\n").ok();
        
    } else {
        println!("Couldn't connect to server...");
    }
    
}
