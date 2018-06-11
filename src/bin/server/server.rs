// MIT License
// Copyright (c) 2018 Cole Phares
// Last Modified: 5/22/2018
// Server side for online card game Egyptian RatScrew

extern crate card;
extern crate rand;
use card::{Card, Rank::*, Suit::*};
use rand::{Rng, random};
use std::net::{TcpListener, SocketAddr};
use std::io::{BufReader, Write, BufRead, Error, ErrorKind};
use std::time::Duration;

/// Contains the players hand and side pile
struct PlayerState {
    hand: Vec<Card>,
    side_pile: Vec<Card>,
}

/// Creates a PlayerState
impl PlayerState {
    fn new() -> Self {
        PlayerState {
            hand: Vec::new(),
            side_pile: Vec::new(),
        }
    }
}

/// Trait for a Player
trait Player {
    // Adds a single card to the hand
    // Used when dealing
    fn add_to_hand(&mut self, Card) ->
        Result<(), Error>;

    // Adds a vector of cards to the side pile
    fn add_to_side_pile(&mut self, &mut Vec<Card>) ->
        Result<(), Error>;

    // Adds the side pile to the hand when hand is empty
    fn add_side_pile_to_hand(&mut self) -> Result<(), Error>;

    // Plays a card from the players hand
    fn play_card(&mut self, &mut Vec<Card>, &mut BufRead, &mut Write, &mut Player) -> 
        Result<Vec<Card>, Error>;

    // Determines if a player has all the cards
    fn won(&mut self, &mut Write) -> bool;

    // Makes the PlayerState read only visible
    fn state(&self) -> &PlayerState;
}

/// The player
struct HumanPlayer(PlayerState);

/// Player trait implementation for player
impl Player for HumanPlayer {
    // Adds a single card to the hand
    // Used when dealing
    fn add_to_hand(&mut self, card: Card) ->
        Result<(), Error> {
            self.0.hand.push(card);
            Ok(())
    }

    // Adds a vector of cards to the side pile
    fn add_to_side_pile(&mut self, pile: &mut Vec<Card>) ->
        Result<(), Error> {
        for _ in 0..pile.len() {
            self.0.side_pile.push(pile.pop().unwrap());
        }
        Ok(())
    }

    // Adds the side pile to the hand when hand is empty
    fn add_side_pile_to_hand(&mut self) -> Result<(), Error> {
        // If side pile is empty, then player loses
        if self.0.side_pile.is_empty() {
            return Err(Error::new(ErrorKind::Other, "Your hand is empty. Computer wins!!"));
        }
        self.0.side_pile = shuffle_deck(self.0.side_pile.to_vec());
        for _ in 0..self.0.side_pile.len() {
            self.0.hand.push(self.0.side_pile.pop().unwrap());
        }
        Ok(())
    }

    // Plays a card from the players hand
    fn play_card(&mut self, mut pile: &mut Vec<Card>, reader: &mut BufRead, 
        mut writer: &mut Write, opponent: &mut Player) -> Result<Vec<Card>, Error>
    {
        // Read input from player
        loop {
            let mut response = String::new();
            reader.read_line(&mut response).ok();

            match response.trim() {
                "c" => {
                    if self.0.hand.is_empty() {
                        match self.add_side_pile_to_hand() {
                            Ok(()) => {},
                            Err(err) => return Err(err),
                        }
                    }
                    // Add card to pile
                    let card = self.0.hand.pop().unwrap();
                    pile.push(card);
                    break;
                }
                "q" => return Err(Error::new(ErrorKind::Other, "Player quit")),
                _ => {
                    writeln!(writer, "Invalid key! Press c to play card\r\n").ok();
                    writer.flush().ok();
                },
            };
        }

        // Send updated top three cards of the pile to client
        send_pile(&pile, &mut writer);
        
        println!("Current pile:");
        for c in pile.iter() {
            println!("{}", c);
        }
        println!();

        // Wait for response from player
        let mut response = String::new();
        reader.read_line(&mut response).ok();
        
        // Determine response action
        match response.trim() {
            // If player thinks there's a combination
            "space" => {
                // Check if there's a combination and add pile to
                // HumanPlayer's hand, otherwise add pile to 
                // MachinePlayer's pile
                if test_pile(&pile) {
                    writeln!(writer, "Combination found. You won the pot!!\r\n").ok();
                    writer.flush().ok();
                    self.add_to_side_pile(&mut pile).ok();
                } else {
                    writeln!(writer, "No combination. Computer gets the pot!!\r\n").ok();
                    writer.flush().ok();
                    opponent.add_to_side_pile(&mut pile).ok();
                }
                return Ok(pile.to_vec());
            },
            "q" => return Err(Error::new(ErrorKind::Other, "Player quit")),
            _ => {
                if test_pile(&pile) {
                    writeln!(writer, "Combination found. Computer gets the pot!!\r\n").ok();
                    writer.flush().ok();
                    opponent.add_to_side_pile(&mut pile).ok();
                }
            }
        };
        
        Ok(pile.to_vec())
    }

    // Determines if a player has all the cards
    fn won(&mut self, writer: &mut Write) -> bool {
        if self.0.hand.len() == 52 {
            writeln!(writer, "Congratulations!! You won the game!!\r\n").ok();
            writer.flush().ok();
            return true;
        }
        false
    }

    // Makes the PlayerState read only visible
    fn state(&self) -> &PlayerState {
        &self.0
    }
}

/// The machine player
struct MachinePlayer(PlayerState);

/// Player trait implementation for machine player
impl Player for MachinePlayer {
    // Adds a single card to the hand
    // Used when dealing
    fn add_to_hand(&mut self, card: Card) ->
        Result<(), Error> {
            self.0.hand.push(card);
            Ok(())
    }
    
    // Adds a vector of cards to the side pile
    fn add_to_side_pile(&mut self, pile: &mut Vec<Card>) ->
        Result<(), Error> {
        for _ in 0..pile.len() {
            self.0.side_pile.push(pile.pop().unwrap());
        }
        Ok(())
    }

    // Adds the side pile to the hand when hand is empty
    fn add_side_pile_to_hand(&mut self) -> Result<(), Error> {
        // If side pile is empty, then computer loses
        if self.0.side_pile.is_empty() {
            return Err(Error::new(ErrorKind::Other, "Computers hand is empty. You win!!"));
        }
        self.0.side_pile = shuffle_deck(self.0.side_pile.to_vec());
        for _ in 0..self.0.side_pile.len() {
            self.0.hand.push(self.0.side_pile.pop().unwrap());
        }
        Ok(())
    }
    
    // Plays a card from the players hand
    fn play_card(&mut self, mut pile: &mut Vec<Card>, reader: &mut BufRead, 
        mut writer: &mut Write, opponent: &mut Player) -> Result<Vec<Card>, Error>
    {
        // Check if hand is empty
        // Refill with side pile if it is
        if self.0.hand.is_empty() {
            match self.add_side_pile_to_hand() {
                Ok(()) => {},
                Err(err) => return Err(err),
            }
        }

        // Add card to pile
        let card = self.0.hand.pop().unwrap();
        pile.push(card);

        // Send updated top three cards of the pile to client
        send_pile(&pile, &mut writer);

        println!("Current pile:");
        for c in pile.iter() {
            println!("{}", c);
        }
        println!();

        // wait for response from player
        let mut response = String::new();
        let _ = reader.read_line(&mut response);

        // Determine response from player
        match response.trim() {
            "space" => {
                if test_pile(&pile) {
                    writeln!(writer, "Combination found. You get the pot!!\r\n").ok();
                    writer.flush().ok();
                    opponent.add_to_side_pile(&mut pile).ok();
                } else {
                    writeln!(writer, "No combination found. Computer gets the pot!!\r\n").ok();
                    writer.flush().ok();
                    self.add_to_side_pile(&mut pile).ok();
                }
                return Ok(pile.to_vec());
            },
            "q" => return Err(Error::new(ErrorKind::Other, "Player quit")),
            _ => {
                if test_pile(&pile) {
                    writeln!(writer, "Combination found. Computer gets the pot!!\r\n").ok();
                    writer.flush().ok();
                    self.add_to_side_pile(&mut pile).ok();
                }
            }
        }

        Ok(pile.to_vec())
    }

    // Determines if a player has all the cards
    fn won(&mut self, writer: &mut Write) -> bool {
        if self.0.hand.len() == 52 {
            writeln!(writer, "Oh, to bad. You lost!!").ok();
            writer.flush().ok();
            return true;
        }
        false
    }

    // Makes the PlayerState read only visible
    fn state(&self) -> &PlayerState {
        &self.0
    }
}

/// Sends the top cards of the pile to the player (max of three)
fn send_pile<T>(pile: &Vec<Card>, writer: &mut T) where T: Write {
 
    // send top three cards of the pile to client
    if pile.len() == 0 {
        writeln!(writer, "Pile is empty\r\n").ok();
        writer.flush().ok();
    }
    else if pile.len() == 1 {
        // send card
        writeln!(writer, "Pile:\r\n").ok();
        writeln!(writer, "{}\r\n", pile[pile.len() - 1]).unwrap();
        writer.flush().ok();
    }
    else if pile.len() == 2 {
        let rev = &pile[pile.len() - 2..=pile.len() - 1];
        write!(writer, "Pile:\r\n").ok();
        
        // send cards
        for c in rev.iter() {
            // send top two cards on pile
            write!(writer, "{}\r\n", c).unwrap();
        }
        writeln!(writer, "").ok();
        writer.flush().ok();
    }
    else {
        let rev = &pile[pile.len() - 3..=pile.len() - 1];
        write!(writer, "Pile:\r\n").ok();
        
        // send cards
        for c in rev.iter() {
            // send top three cards on pile
            write!(writer, "{}\r\n", c).unwrap();
        }
        writeln!(writer, "").ok();
        writer.flush().ok();
    }
}

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

/// Top and second card have same rank
fn is_pair(pile: &Vec<Card>) -> bool {
    pile[pile.len() - 1] == pile[pile.len() - 2]
}

/// Top and third card have same rank
fn is_sandwich(pile: &Vec<Card>) -> bool {
    pile[pile.len() - 1] == pile[pile.len() - 3]
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
    is_sixty_nine_match(pile[pile.len() - 1], pile[pile.len() - 2])
}

/// Top card and third card have ranks 6 && 9 or 9 && 6
fn is_sixty_nine_sandwich(pile: &Vec<Card>) -> bool {
    is_sixty_nine_match(pile[pile.len() - 1], pile[pile.len() - 3])
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
    // If left is an ace, compare it as a 1 and a 14 and return true if
    // cards are sequential
    if is_ace(left) {
        let value_low = left.rank.value() as i32 - right.rank.value() as i32;
        let value_high = 14 as i32 - right.rank.value() as i32;
        if value_low.abs() == 1 || value_high.abs() == 1 {
            return true;
        }
    }
    // If right is an ace, compare it as a 1 and a 14 and return true if 
    // cards are sequential
    if is_ace(right) {
        let value_low = left.rank.value() as i32 - right.rank.value() as i32;
        let value_high = left.rank.value() as i32 - 14 as i32;
        if value_low.abs() == 1 || value_high.abs() == 1 {
            return true;
        }
    }
    // Otherwise just check for sequentail cards
    let value = left.rank.value() as i32 - right.rank.value() as i32;
    if value.abs() == 1 {
        return true;
    }
    false
}

/// Check a card for an ace value
fn is_ace(card: Card) -> bool {
    if card.rank.value() == 1 {
        return true;
    }
    false
}

/// Top three cards form a run in any order
fn is_run(pile: &Vec<Card>) -> bool {
    let left = pile[pile.len() - 1];
    let middle = pile[pile.len() - 2];
    let right = pile[pile.len() - 3];

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

/// Tests for different combinations
fn test_pile(pile: &Vec<Card>) -> bool {
    // If pile has 0 or 1 card, there is no combination
    if pile.len() < 2 {
        return false
    }
    // If pile has 2 or more cards, check for combinations that
    // require at least 2 cards
    if pile.len() >= 2 {
        if is_pair(pile) {
            return true;
        }
        // tests for a sixty-nine combo
        else if is_sixty_nine(pile) {
            return true;
        }
        // If pile has more than two cards, check for combinations
        // that require at least three cards
        if pile.len() > 2 {
            // tests for a pair sandwich
            if is_sandwich(pile) {
                return true;
            }
            // tests for a sixty-nine combo
            else if is_sixty_nine_sandwich(pile) {
                return true;
            }
            // tests for a run of three cards
            else if is_run(pile) {
                return true;
            }
        }
    } 
    false
}

/// Deals the cards to the client and server hands
//fn deal_hands(deck: &mut Vec<Card>, socket: &TcpStream) -> Vec<Card> {
fn deal_hands(deck: &mut Vec<Card>, machine: &mut MachinePlayer,
              human: &mut HumanPlayer)  {

    // deal card to player and self
    for _ in 0..(deck.len()/2) {
        human.add_to_hand(deck.pop().unwrap()).ok();
        machine.add_to_hand(deck.pop().unwrap()).ok();
    }
}

/// Game control function
fn play_game<T, U>(mut reader: T, mut writer: U) -> 
    Result<(), Error> 
    where T: BufRead, U: Write 
{
    let mut deck: Vec<Card> = shuffle_deck(make_deck());
    let mut pile: Vec<Card> = Vec::new();
    let mut machine = MachinePlayer(PlayerState::new());
    let mut human = HumanPlayer(PlayerState::new());

    deal_hands(&mut deck, &mut machine, &mut human);
    
    // Let the client who plays first
    let mut turn = random::<usize>() % 2;
    if turn == 0 {
        write!(writer, "Computer goes first!\r\n").ok();
        writer.flush().ok();
    } else {
        write!(writer, "You go first!\r\n").ok();
        writer.flush().ok();
    }
    
    
    loop {
        //let mut response = String::new();
        let player: &mut Player;
        let opponent: &mut Player;
        
        // Machine plays if turn == 0
        // Human plays if turn == 1
        if turn % 2 == 0 {
            player = &mut machine;
            opponent = &mut human;
            writeln!(writer, "Computer's turn!\r\n").ok();
            writer.flush().ok();
        } else {
            player = &mut human;
            opponent = &mut machine;
            writeln!(writer, "Your turn! Press c to play card\r\n").ok();
            writer.flush().ok();
        }

        // Play a card from players hand
        match player.play_card(&mut pile, &mut reader, &mut writer, opponent) {
            Ok(updated_pile) => pile = updated_pile,
            Err(err) => {
                writeln!(writer, "{}\r\n", err);
                return Ok(());
            }
        }

        // Determine if a player has won the game
        if player.won(&mut writer) {
            break;
        }
        turn += 1;
    }
    
    Ok(())
}

fn main() {
    // Creates Tcp connection
    let address = ("0.0.0.0:24794").parse::<SocketAddr>().unwrap();
    let listener = TcpListener::bind(&address).unwrap();

    match listener.accept() {
        Ok((socket, _addr)) => {
            let mut writer = socket.try_clone().unwrap();
            let mut reader = BufReader::new(&socket);
            socket.set_read_timeout(Some(Duration::new(3, 0))).ok();

            play_game(reader, writer).ok();
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
