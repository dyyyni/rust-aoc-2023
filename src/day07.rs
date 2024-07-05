use crate::utils::read_lines;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Cards {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

struct Hand {
    cards: Vec<Cards>, 
    bid: u64,
}

impl Hand {
    fn new(cards: Vec<Cards>, bid: u64) -> Hand {
        Hand { cards, bid }
    }
    
}

pub fn run_part1() {
    println!("Running day07 part 1 solution.");
}