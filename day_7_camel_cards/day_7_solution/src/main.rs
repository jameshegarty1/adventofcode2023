use std::fs;
use std::str::FromStr;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    One
}

impl FromStr for Card {
    type Err =  ();// Use a more descriptive error type in a real application

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Card::Ace),
            "K" => Ok(Card::King),
            "Q" => Ok(Card::Queen),
            "J" => Ok(Card::Jack),
            "T" => Ok(Card::Ten),
            "9" => Ok(Card::Nine),
            "8" => Ok(Card::Eight),
            "7" => Ok(Card::Seven),
            "6" => Ok(Card::Six),
            "5" => Ok(Card::Five),
            "4" => Ok(Card::Four),
            "3" => Ok(Card::Three),
            "2" => Ok(Card::Two),
            "1" => Ok(Card::One),
            _ => Err(()), // Return an error for unrecognized inputs
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard
}

#[derive(Debug)]
struct Hand {
    card_map: HashMap<Card, usize>,
    bid: i32,
    card_rank: Vec<i32>,
    hand_type: HandType,
    hand_value: i32, 
    overall_rank: i32
}

impl Default for Hand {
    fn default() -> Self {
        Self {
            card_map: HashMap::new(),
            bid: 0,
            card_rank: Vec::new(),
            hand_type: HandType::HighCard,
            hand_value: 0,
            overall_rank: 0,
        }
    }
}

fn parse_file(file_path: &str) -> Vec<Hand> {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");


    let mut hands: Vec<Hand> = Vec::new();


    
    contents.lines().for_each(|line| {

        let elements: Vec<&str> = line.split_whitespace().collect();

        let bid = elements[1].parse::<i32>().unwrap();
        let card_str = elements[0].to_string();

        let card_vec_str: Vec<&str> = card_str.split_whitespace().collect();

        let cards: Vec<Card> = card_vec_str.iter().flat_map(|hand| { 
            hand.chars().filter_map(|char| {
                char.to_string().parse::<Card>().ok()
            })
        }).collect();

        let mut card_map: HashMap<Card, usize> = HashMap::new();

        for card in cards {
            *card_map.entry(card).or_insert(1) += 1;
        }

        let hand = Hand{
            card_map,
            bid,
            ..Default::default()
        };
        hands.push(hand);
    });

    hands
}

fn classify_hands(hands: &mut Vec<Hand>) {
    for hand in hands.iter_mut() {
        let mut has_three_of_a_kind = false;
        let mut has_pair = false;
        let mut pairs_count = 0;

        for &count in hand.card_map.values() {
            match count {
                5 => {
                    hand.hand_type = HandType::FiveOfAKind;
                    break; // Highest rank found, no need to check further
                },
                4 => {
                    hand.hand_type = HandType::FourOfAKind;
                    break; // Next highest rank found, no need to check further
                },
                3 => has_three_of_a_kind = true, // Mark that we found a Three of a Kind
                2 => {
                    has_pair = true; // Mark that we found a Pair
                    pairs_count += 1; // Count the pairs for checking Two Pair
                },
                _ => {}
            }
        }

        // Determine the hand type based on what was found
        if hand.hand_type != HandType::FiveOfAKind && hand.hand_type != HandType::FourOfAKind {
            if has_three_of_a_kind && has_pair {
                hand.hand_type = HandType::FullHouse;
            } else if has_three_of_a_kind {
                hand.hand_type = HandType::ThreeOfAKind;
            } else if pairs_count == 2 {
                hand.hand_type = HandType::TwoPair;
            } else if has_pair {
                hand.hand_type = HandType::OnePair;
            } else {
                hand.hand_type = HandType::HighCard; // Default to HighCard if no other pattern matches
            }
        }    }
}

fn main() {

    let file_path = "test_input";

    let mut hands = parse_file(&file_path);
 
    classify_hands(&mut hands);

    for hand in &hands {
        println!("{:?}", hand);
    }


}
