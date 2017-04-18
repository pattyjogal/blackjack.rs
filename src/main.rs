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
struct Game
{
    deck: Vec<Card>,
    player: Player,
    choice: Choice
}
struct Player
{
    name: String,
    hand: Vec<Card>,
    money: u32
}

fn main()
{
    let mut gameState: Game = Game {
        deck: Vec::new(),
        player: Player {
            name: "Patrick".to_string(),
            hand: Vec::new(),
            money: 0
        },
        choice: Choice::None
    };
        
    build_deck(&mut gameState);
    deal(&mut gameState);
    deal(&mut gameState);

    loop {
        println!("Hello friend! I'm your dealer for today's game of Blackjack");
        println!("I'm gonna show you your cards real fast:");
        println!("Your hand: {:?}", gameState.player.hand);
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
                        if hit(&mut gameState) == 0 {
                            break
                        }
                        deal(&mut gameState);
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
                    gameState.player.hand = Vec::new();
                    // And redeal
                    deal(&mut gameState);
                    deal(&mut gameState);
                    println!("Press enter to continue...");
                },
                _ => break
            }
            
        }

    }
}

fn build_deck(gameState: &mut Game)
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
        gameState.deck.push(unshuffled_deck.remove(rand::thread_rng().gen_range(0, length)));
        length -= 1;
    }
    println!("{:?}", gameState.deck);
     
}

// A simple function to give a card off of the top of the deck
// to the player
fn deal(gameState: &mut Game)
{
    gameState.player.hand.push(gameState.deck.pop().unwrap());
    println!("Your hand: {:?}", gameState.player.hand);

}

fn hit(gameState: &mut Game) -> u32
{
    let mut sum = 0;
    for card in &gameState.player.hand {
        sum += card.rank;
        println!("{}", render_card(&card));
    }

    if (sum > 21){
        println!("That's a loss!");
        return 0
    }
    
    println!("Sum: {}", sum);
    1

}

fn strip_input(s: &mut String)
{
    s.pop();
}

fn render_card(card: &Card) -> String
{
    let mut number = "";

    if card.rank > 10 {
        number = format!("{} ", card.rank);
    } else if card.rank == 10 {
        number = "10"
    } else if card.rank == 11 {
        number = "J "
    } else if card.rank == 12 {
        number = "Q "
    } else if card.rank == 13 {
        number = "K "
    } 
                                    
    format!( "┌─────────┐
│{}       │
│         │
│         │
│    {}   │
│         │
│         │
│       {}│
└─────────┘", card.rank, match card.suit {
                                            0 => "♥",
                                            1 => "♦",
                                            2 => "♣",
                                            3 => "♠",
                                            _ => " "
                                            }, card.rank)
}
