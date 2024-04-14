use std::fs;
use std::str::FromStr;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
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
    hand_string: String,
    card_map: HashMap<Card, usize>,
    bid: i32,
    hand_type: HandType,
    hand_value: i32,
    n_jokers: usize,
}

impl Default for Hand {
    fn default() -> Self {
        Self {
            hand_string: String::new(),
            card_map: HashMap::new(),
            bid: 0,
            hand_type: HandType::HighCard,
            hand_value: 0,
            n_jokers: 0,
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
            *card_map.entry(card).or_insert(0) += 1;
        }

        let hand = Hand{
            hand_string: card_str,
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
                    hand.hand_value = 6;
                    break; // Highest rank found, no need to check further
                },
                4 => {
                    hand.hand_type = HandType::FourOfAKind;
                    hand.hand_value = 5;
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
                hand.hand_value = 4;
            } else if has_three_of_a_kind {
                hand.hand_type = HandType::ThreeOfAKind;
                hand.hand_value = 3;
            } else if pairs_count == 2 {
                hand.hand_type = HandType::TwoPair;
                hand.hand_value = 2;

            } else if has_pair {
                hand.hand_type = HandType::OnePair;
                 hand.hand_value = 1;

            } else {
                hand.hand_type = HandType::HighCard; // Default to HighCard if no other pattern matches
                hand.hand_value = 0;

            }
        }    
    }
}

fn card_rank_value(card: char) -> u8 {
    match card {
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => 0, // default case, possibly error handling
    }
}

fn card_rank_value_two(card: char) -> u8 {
    match card {
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => 1, // J now lowest in card value
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => 0, // default case, possibly error handling
    }
}

fn sort_hands(hands: &mut Vec<Hand>) {
    hands.sort_by(|a, b| {
        a.hand_value.cmp(&b.hand_value)
            .then_with(|| {
                let sorted_a = a.hand_string.chars().map(card_rank_value).collect::<Vec<_>>();
                let sorted_b = b.hand_string.chars().map(card_rank_value).collect::<Vec<_>>();
                sorted_a.cmp(&sorted_b)
            })
    });
}

fn sort_hands_two(hands: &mut Vec<Hand>) {
    hands.sort_by(|a, b| {
        a.hand_value.cmp(&b.hand_value)
            .then_with(|| {
                let sorted_a = a.hand_string.chars().map(card_rank_value_two).collect::<Vec<_>>();
                let sorted_b = b.hand_string.chars().map(card_rank_value_two).collect::<Vec<_>>();
                sorted_a.cmp(&sorted_b)
            })
    });
}


fn rank_hands(hands: &mut Vec<Hand>) -> Vec<(i32,&i32)>{
    let mut ranked = Vec::new();
    for (index, hand) in hands.iter_mut().enumerate() {
        let rank: i32 = (index + 1) as i32;
        ranked.push((rank,&hand.bid));
    }
    ranked
}

fn reclassify_jokers(hands: &mut Vec<Hand>) {

    for hand in hands.iter_mut() {

        if hand.hand_string == "2JJJJ" {
            println!("Debugging {}",hand.hand_string);
        }
        let mut has_three_of_a_kind = false;
        let mut has_pair = false;
        let mut pairs_count = 0;

        match hand.card_map.get(&Card::Jack) {
            Some(&count) => {
                hand.card_map.remove(&Card::Jack);
                hand.n_jokers = count;
            }
            None => {}
        };

        //reset 
        hand.hand_type = HandType::HighCard;
        hand.hand_value = 0;
        
        for &count in hand.card_map.values() {
            match count {
                5 => {
                    hand.hand_type = HandType::FiveOfAKind;
                    hand.hand_value = 6;
                    break; // Highest rank found, no need to check further
                },
                4 => {
                    hand.hand_type = HandType::FourOfAKind;
                    hand.hand_value = 5;

                    if hand.n_jokers > 0 {
                        hand.hand_type = HandType::FiveOfAKind;
                        hand.hand_value = 6;
                    }
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
                hand.hand_value = 4;
                
                //This hand could contain 1 joker, which would upgrade to FourOfAKind
                if hand.n_jokers > 0 {
                    hand.hand_type = HandType::FourOfAKind;
                    hand.hand_value = 5;
                }

            } else if has_three_of_a_kind {
                hand.hand_type = HandType::ThreeOfAKind;
                hand.hand_value = 3;
                if hand.n_jokers == 2 {
                    hand.hand_type = HandType::FiveOfAKind;
                    hand.hand_value = 6;
                } else if hand.n_jokers == 1 {
                    hand.hand_type = HandType::FourOfAKind;
                    hand.hand_value = 5;
                }
            } else if pairs_count == 2 {
                hand.hand_type = HandType::TwoPair;
                hand.hand_value = 2;

                //Can have 1 joker here, could upgrade to FullHouse
                if hand.n_jokers > 0 {
                    hand.hand_type = HandType::FullHouse;
                    hand.hand_value = 4;
                } 

            } else if has_pair { 
                hand.hand_type = HandType::OnePair;
                hand.hand_value = 1;

                //Could have 3 jokers
                match hand.n_jokers {
                    3 => {
                        hand.hand_type = HandType::FiveOfAKind;
                        hand.hand_value = 6;
                    },
                    2 => {
                        hand.hand_type = HandType::FourOfAKind;
                        hand.hand_value = 5;
                    },
                    1 => {
                        hand.hand_type = HandType::ThreeOfAKind;
                        hand.hand_value = 3;
                    }
                    _ => {}
                }

            } else {
                hand.hand_type = HandType::HighCard; // Default to HighCard if no other pattern matches
                hand.hand_value = 0;

                //Could have 5 jokers 
                match hand.n_jokers {
                    5 => {
                        hand.hand_type = HandType::FiveOfAKind;
                        hand.hand_value = 6;
                    },
                    4 => {
                        hand.hand_type = HandType::FiveOfAKind;
                        hand.hand_value = 6;
                    },
                    3 => {
                        hand.hand_type = HandType::FourOfAKind;
                        hand.hand_value = 5;
                    },
                    2 => {
                        hand.hand_type = HandType::ThreeOfAKind;
                        hand.hand_value = 3;
                    },
                    1 => {
                        hand.hand_type = HandType::OnePair;
                        hand.hand_value = 1;
                    }
                    _ => {}
                }

            }
        }
    }
 
}

fn solve_part_one(hands: &mut Vec<Hand>){
    classify_hands(hands);
    
    // We now need to sort the hands, primarily by hand_:q
    // value, then hand_string
    sort_hands(hands);

     //Now assign overall rank
    let score = rank_hands(hands);   

    let winnings: i32 = score.iter()
                             .map(|(a,b)| a * *b)
                             .sum();

    println!("Part 1 - Total winnings = {winnings}");
}

fn solve_part_two(hands: &mut Vec<Hand>){
    reclassify_jokers(hands);

    sort_hands_two(hands);

    let score = rank_hands(hands);   
    let winnings: i32 = score.iter()
                             .map(|(a,b)| a * *b)
                             .sum();

    println!("Part 2 - Total winnings = {winnings}");
}

fn main() {

    let file_path = "input_main";

    let mut hands = parse_file(&file_path);

    solve_part_one(&mut hands);
  
    solve_part_two(&mut hands);
    
}
