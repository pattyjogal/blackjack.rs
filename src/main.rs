extern crate rand;
use rand::Rng;
use std::io;

#[derive(Debug)]
enum Choice {
    Hit,
    Stay,
    None,
}

#[derive(Debug, Copy, Clone)]
struct Card {
    suit: u32,
    rank: u32,
}

struct Game {
    deck: Vec<Card>,
    players: Vec<Player>,
    choice: Choice,
    current_player_index: usize,
}
struct Player {
    name: String,
    hand: Vec<Card>,
    money: u32,
}

impl Game {
    pub fn new(players: Vec<Player>) -> Game {
        Game {
            deck: Vec::new(),
            players: players,
            choice: Choice::None,
            current_player_index: 0,
        }
    }
    // Builds a deck with the standard 52 cards
    pub fn build_deck(&mut self) {
        let mut unshuffled_deck: Vec<Card> = Vec::new();

        // We have to build the deck first!
        for s in 0..4 {
            for r in 0..13 {
                unshuffled_deck.push(Card { suit: s, rank: r });
            }
        }
        // Remove the cards from the unshuffled deck and put
        // them at random into the game deck
        let mut length = unshuffled_deck.len();
        while length > 0 {
            self.deck
                .push(unshuffled_deck.remove(rand::thread_rng().gen_range(0, length)));
            length -= 1;
        }

    }

    // A simple function to give a card off of the top of the deck
    // to the player
    fn deal(&mut self) {
        let next_card = self.deck.pop().unwrap();
        self.get_player().hand.push(next_card);
    }

    // The hit function that adds a card to the players hand, and kills them off
    // if they go over 21
    fn hit(&mut self) -> u32 {
        let mut sum = 0;
        for card in &self.get_player().hand {
            sum += card.rank;

        }

        if (sum > 21) {
            println!("That's a loss!");
            return 0;
        }

        1

    }

    // Gets a mutable reference to the current player
    fn get_player(&mut self) -> &mut Player {
        &mut self.players[self.current_player_index]
    }

    fn print_cards(&mut self) {
        for card in &self.get_player().hand {
            println!("{}", render_card(&card));
        }
    }
}

fn main() {
    let mut game_state = Game::new(vec![Player {
                                            name: "Patrick".to_string(),
                                            hand: Vec::new(),
                                            money: 0,
                                        }]);
    game_state.build_deck();
    game_state.deal();
    game_state.deal();

    loop {
        println!("Alright, {}, it's your turn!", game_state.get_player().name);
        // Let's show the user their cards
        println!("I'm gonna show you your cards real fast:");
        game_state.print_cards();
        println!("Ok, so would you like to [h]it or [s]tay?");

        let mut choice = String::new();

        loop {
            io::stdin()
                .read_line(&mut choice)
                .expect("Failed to read line");

            choice = choice.to_lowercase();

            strip_input(&mut choice);
            game_state.choice = match choice.as_str() {
                "h" | "hit" => Choice::Hit,
                "s" | "stay" => Choice::Stay,
                _ => {
                    Choice::None;
                    continue;
                }
            };
            break;
        }
        match game_state.choice {
            Choice::Hit => {
                println!("Aww yes! Let's play!");
                println!("So I'm too lazy to implement betting at this moment, so we're gonna bet like 1 dollar");
                let bet = 1;
                'hitLoop: loop {
                    if game_state.hit() == 0 {
                        break;
                    }
                    game_state.deal();
                    game_state.print_cards();
                    let mut cont = String::new();

                    println!("Hit again? [y]es or [n]o?");

                    'input: loop {
                        io::stdin()
                            .read_line(&mut cont)
                            .expect("Failed to read line");

                        cont = cont.to_lowercase();

                        strip_input(&mut cont);

                        match cont.as_str() {
                            "y" | "yes" => break 'input,
                            "n" | "no" => break 'hitLoop,
                            // Clear cont so we can input "y"/"n" again if we fat finger it the first time
                            _ => cont = String::new(),
                        }
                    }

                }
                // Empty the hand now
                game_state.get_player().hand = Vec::new();
                // And redeal
                game_state.deal();
                game_state.deal();
            }
            _ => break,
        }
    }
}

fn strip_input(s: &mut String) {
    s.pop();
}

fn render_card(card: &Card) -> String {
    let mut number = String::new();

    if card.rank < 10 {
        number = format!("{} ", card.rank);
    } else if card.rank == 10 {
        number = "10".to_string()
    } else if card.rank == 11 {
        number = "J ".to_string()
    } else if card.rank == 12 {
        number = "Q ".to_string()
    } else if card.rank == 13 {
        number = "K ".to_string()
    }

    format!("┌─────────┐
│{}       │
│         │
│         │
│    {}   │
│         │
│         │
│       {}│
└─────────┘",
            number,
            match card.suit {
                0 => "♥ ",
                1 => "♦ ",
                2 => "♣ ",
                3 => "♠ ",
                _ => " ",
            },
            number)
}
