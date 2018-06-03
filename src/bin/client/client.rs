extern crate card;
extern crate ggez;
use ggez::event;
use card::{Card, Rank::*, Suit::*};
use std::net::TcpStream;
use std::io::{BufReader, BufWriter, Write, BufRead};

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

        /*
        let suit: String = response.chars()
                                   .skip(4)
                                   .take(6)
                                   .collect();
        */
        println!("Card: ({}, {})", rank, suit);
        
        /*
        match reader.read_line(&mut response) {
            Ok(_) => {
                println!("message from server: {}", response);
            }
            Err(e) => {
                println!("Error reading message {:?}", e);
            }
        }
        */
        let mut writer = BufWriter::new(&stream);
        writer.write_all(b"client says hello\n").ok();
        
    } else {
        println!("Couldn't connect to server...");
    }
    
}
