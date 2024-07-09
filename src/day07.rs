use crate::utils::read_lines;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
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


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAkind,
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    hand_type: HandType,
    cards: Vec<Cards>, 
    bid: u64,
}

impl Hand {
    fn new(cards: Vec<Cards>, bid: u64) -> Hand {
        let hand_type = Self::find_hand_type(&cards);  
        Hand { cards, bid, hand_type }
    }

    fn find_hand_type(cards: &Vec<Cards>) -> HandType {
        let mut counts = std::collections::HashMap::new();
        for &card in cards {
            *counts.entry(card).or_insert(0) += 1;
        }

        let mut values: Vec<&u32> = counts.values().collect();
        values.sort();

        match values.as_slice() {
            [1, 1, 1, 1, 1] => HandType::HighCard,
            [1, 1, 1, 2]    => HandType::OnePair,
            [1, 2, 2]       => HandType::TwoPair,
            [1, 1, 3]       => HandType::ThreeOfAKind,
            [2, 3]          => HandType::FullHouse,
            [1, 4]          => HandType::FourOfAKind,
            [5]             => HandType::FiveOfAkind,
            _               => panic!("Invalid hand"),

        }
    }
    
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hand_type
            .cmp(&other.hand_type)
            .then_with(|| {
                self.cards
                    .iter()
                    .cmp(other.cards.iter())  
            })
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn char_to_card(c: char) -> Cards {
    match c {
        '2' => Cards::Two,
        '3' => Cards::Three,
        '4' => Cards::Four,
        '5' => Cards::Five,
        '6' => Cards::Six,
        '7' => Cards::Seven,
        '8' => Cards::Eight,
        '9' => Cards::Nine,
        'T' => Cards::Ten,
        'J' => Cards::Jack,
        'Q' => Cards::Queen,
        'K' => Cards::King,
        'A' => Cards::Ace,
        _   => panic!("Invalid card character"),
        
    }
}

fn parse_line_to_hand(input: &str) -> Hand {
    let parts: Vec<&str> = input.split_whitespace().collect();
    let cards_str = parts[0];
    let bid = parts[1].parse().expect("Invalid bid");

    let cards: Vec<Cards> = cards_str.chars().map(char_to_card).collect();

    Hand::new(cards, bid)
}

pub fn run_part1() {
    println!("Running day07 part 1 solution.");

    match read_lines("input/day07.txt") {
        Ok(lines) => {
            let mut hands: Vec<Hand> = Vec::new();
            let mut total_winnings = 0;
            let mut rank = 1;
            
            for line in lines {
                hands.push(parse_line_to_hand(&line));    
            }
            hands.sort();
            for hand in hands {
                println!("{:?}", hand);
                total_winnings += hand.bid * rank;
                rank += 1;
            }
            println!("The total winnings from the set: {}", total_winnings);
        }
        Err(e) => println!("Error : {}", e)
    }
}