// MIT License
// Copyright (c) 2018 Cole Phares
// Last Modified: 5/22/2018
// Server side for online card game Egyptian RatScrew

extern crate card;
extern crate termion;
use std::net::TcpStream;
use std::io::{BufReader, Write, BufRead, stdin, stdout};
use termion::event::Key;
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use std::thread;

/// Players game control
fn play_game(stream: &TcpStream) {
    let network_reader = stream.try_clone().unwrap();
    let mut key_writer = stream.try_clone().unwrap();

    // Thread to control key events
    let key_handler = thread::spawn(move || {
        // Set up standard input for event handling
        let stdin = stdin();
        let mut _stdout = stdout().into_raw_mode().unwrap();
        
        for c in stdin.keys() {
            match c.unwrap() {
                // Press Space bar to slap the pile
                Key::Char(' ') => {
                    write!(key_writer, "space\r\n").ok();
                    key_writer.flush().unwrap();
                },
                // Press 'c' to play a card
                Key::Char('c') => {
                    write!(key_writer, "c\r\n").ok();
                    key_writer.flush().unwrap();
                }
                // Press 'q' to quit at anytime
                Key::Char('q') => {
                    println!();
                    println!("Thank you for playing!\r\n");
                    write!(key_writer, "q").ok();
                    key_writer.flush().unwrap();
                    return;
                },
                _ => println!("Invalid key pressed\r\n")
            }
        }
    });

    // Thread to handle messages from the server
    thread::spawn(move || {
        let mut reader = BufReader::new(&network_reader);
        loop {
            let mut message = String::new();
            match BufRead::read_line(&mut reader, &mut message) {
                Ok(n) => {
                    if n != 0 {
                        print!("{}", message);
                    }
                },
                _ => return,
            }
        }
    });

    // Call key event here to allow for key events and messages from server
    key_handler.join().unwrap();
}

fn main() {
    let mut input = String::new();
    let mut valid_input = false;

    while !valid_input {
        // Print welcome prompt
        println!("Welcome to Egyptian Ratscrew!!\nPress p to play:");
        stdin().read_line(&mut input).ok();
        let prompt = input.bytes().nth(0);
        match prompt.unwrap() as char {
            'p' => valid_input = true,
            'q' => panic!(),
            _ => println!("Incorrect character! Please try again.")
        }
        println!("\nPress Space bar to slap the pile");
        println!("Press 'Q' at anytime to quit");
        println!();
    }
    
    // Connect to the server
    if let Ok(stream) = TcpStream::connect("127.0.0.1:24794") {
        play_game(&stream);
    } else {
        println!("Couldn't connect to server...");
    }
}
