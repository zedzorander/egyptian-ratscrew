# Rust Egyptian Ratscrew Card Game

Copyright (c) 2018 Cole Phares

## About

This program implements a Linux command line TCP model of the card game 
Egyptian Ratscrew. Link for description of game: 
https://en.wikipedia.org/wiki/Egyptian_Ratscrew.
This program only implements pairs, sandwiches, sixty-nines, sixty-nine sandwiches
and runs of three.

## Getting Started

### Installation (Linux)

1. Clone the repository with: <br />
`git clone https://github.com/zedzorander/egyptian-ratscrew.git`

2. Open second terminal

3. In first terminal, start server with: <br />
`cargo run --bin server`

4. In second terminal, start client with: <br />
`cargo run --bin client`

### Known Issues

1. Prints out `Invalid key! ...` message if human player takes longer than 3 seconds to play a card.

2. Ends game if one player has an empty hand but the main pile isn't empty. In that event, whichever player still has cards in their hand should deal until a slap occurs. If the player dealing wins the slap, the game is over. If the player without any cards left wins the slap, they get the pile and the game continues.

### License

This program is licensed under the "MIT License". Please see the file LICENSE in the source distribution of this software for license terms.
