extern crate card;
use card::{Card, Rank::*, Suit::*};
use std::net::TcpStream;
use std::io::{BufReader, BufWriter, Write, BufRead};

fn get_suit(suit: String) -> Suit {
    match suit.as_ref() {
        "Hearts" => Suit::Hearts,
        "Diamonds" => Suit::Diamonds,
        "Spades" => Suit::Spades,
        "Clubs" => Suit::Clubs,
    }
}

fn main() {
    // connect to server.rs
    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:24794") {
        let mut reader = BufReader::new(&stream);
        let mut response = String::new();
        reader.read_line(&mut response);

        let rank: u32 = response.chars()
                                .nth(1)
                                .unwrap() as u32 - '0' as u32;
        
        let suit: String = response.chars()
                                   .skip(4)
                                   .filter(|x| *x != ')' && *x != '\n')
                                   .collect();

        println!("Card: ({}, {})", rank, suit);
        
        let card: Card;

        if rank <= 10 {
            let card = Card::new(Num(rank), get_suit(suit));
        }else {
            let card = match rank {
                11 => Card::new(Jack, get_suit(suit)),
                12 => Card::new(Queen, get_suit(suit)),
                13 => Card::new(King, get_suit(suit)),
                1 => Card::new(Ace, get_suit(suit)),
            };
        }
        
        let mut writer = BufWriter::new(&stream);
        writer.write_all(b"client says hello\n").ok();
        
    } else {
        println!("Couldn't connect to server...");
    }
    
}
