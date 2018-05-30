extern crate card;
extern crate serde;
use card::{Card, Rank::*, Suit::*};
use std::net::TcpStream;
use std::io::Read;
use serde::ser::{Serialize, Serializer, SerializeStruct};

fn main() {
    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:24794") {
        println!("Connected to server!");
        let mut message = String::new();
        match stream.read_to_string(&mut message) {
            Ok(_n) => {
                println!("buff is: {:?}", message.trim());
            }
            Err(_e) => {
                println!("Error motherfucker");
            }
        }
    } else {
        println!("Couldn't connect to server...");
    }
    
}
