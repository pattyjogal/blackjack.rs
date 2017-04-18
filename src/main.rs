extern crate rand;
use rand::Rng;
use std::io;

#[derive(Debug)]
enum Choice
{
    Hit,
    Stay,
    None
}

#[derive(Debug, Copy, Clone)]
struct Card
{
    suit: u32,
    rank: u32
}

struct Game<'a>
{
    deck: Vec<Card>,
    players: Vec<Player>,
    choice: Choice,
    currentPlayer: &'a Player
}
struct Player
{
    name: String,
    hand: Vec<Card>,
    money: u32
}

impl <'a> Game<'a> {
    pub fn new(players: Vec<Player>) -> Game<'a> {
        Game {
            deck: Vec::new(),
            players: players,
            choice: Choice::None,
            currentPlayer: &players[0]
        }
    }
    // Builds a deck with the standard 52 cards
    pub fn build_deck(&mut self)
    {
        let mut unshuffled_deck:Vec<Card> = Vec::new();

        // We have to build the deck first!
        for s in 0..4 {
            for r in 0..13 {
                unshuffled_deck.push(Card {suit:s, rank:r});
            }
        }
        // Remove the cards from the unshuffled deck and put
        // them at random into the game deck
        let mut length = unshuffled_deck.len();
        while length > 0 {
            self.deck.push(unshuffled_deck.remove(rand::thread_rng().gen_range(0, length)));
            length -= 1;
        }
        println!("{:?}", self.deck);
        
    }

    // A simple function to give a card off of the top of the deck
    // to the player
    fn deal(&mut self)
    {
        self.currentPlayer.hand.push(self.deck.pop().unwrap());
        println!("Your hand: {:?}", &self.currentPlayer.hand);

    }

    //
    fn hit(&self) -> u32
    {
        let mut sum = 0;
        for card in &self.currentPlayer.hand {
            sum += card.rank;
            
        }

        if (sum > 21){
            println!("That's a loss!");
            return 0
        }
        
        println!("Sum: {}", sum);
        1

    }
}

fn main()
{
    // TODO: See if there are constructors in Rust
    let mut gameState = Game::new(vec!(
        Player {
            name: "Patrick".to_string(),
            hand: Vec::new(),
            money: 0
        }));
            
    gameState.build_deck();
    gameState.deal();
    gameState.deal();

    loop {
        println!("Hello friend! I'm your dealer for today's game of Blackjack");
        println!("I'm gonna show you your cards real fast:");
        println!("Your hand: {:?}", gameState.currentPlayer.hand);
        println!("Ok, so would you like to [h]it or [s]tay?");
        
        let mut choice = String::new();

        loop {
            io::stdin().read_line(&mut choice)
                .expect("Failed to read line");

            choice = choice.to_lowercase();
            
            println!("Choice: {}", choice.as_str());
            strip_input(&mut choice);
            gameState.choice = match choice.as_str() {
                "h" | "hit" => Choice::Hit,
                "s" | "stay" => Choice::Stay,
                _ => Choice::None
            };
            println!("{:?}", gameState.choice);
            match gameState.choice {
                Choice::Hit => {
                    println!("Aww yes! Let's play!");
                    println!("So I'm too lazy to implement betting at this moment, so we're gonna bet like 1 dollar");
                    let bet = 1;
                    'hitLoop: loop {
                        for card in &gameState.currentPlayer.hand {
                            print!("{}", render_card(&card));
                        }
                        if gameState.hit() == 0 {
                            break
                        }
                        gameState.deal();
                        let mut cont = String::new();

                        println!("Hit again? [y]es or [n]o?");

                        'input: loop {
                            io::stdin().read_line(&mut cont)
                                .expect("Failed to read line");

                            cont = cont.to_lowercase();

                            strip_input(&mut cont);
                            
                            match cont.as_str()  {
                                "y" | "yes" => break 'input,
                                "n" | "no"  => break 'hitLoop,
                                _ => continue
                            }                          
                        }

                    }
                    // Empty the hand now
                    gameState.currentPlayer.hand = Vec::new();
                    // And redeal
                    gameState.deal();
                    gameState.deal();
                    println!("Press enter to continue...");
                },
                _ => break
            }
            
        }

    }
}







fn strip_input(s: &mut String)
{
    s.pop();
}

fn render_card(card: &Card) -> String
{
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
                                    
    format!( "┌─────────┐
│{}       │
│         │
│         │
│    {}   │
│         │
│         │
│       {}│
└─────────┘", number, match card.suit {
                                            0 => "♥ ",
                                            1 => "♦ ",
                                            2 => "♣ ",
                                            3 => "♠ ",
                                            _ => " "
                                            }, number)
}
