use std::{cmp::Ordering, collections::HashMap, fs};

fn iterate_file(filename: &str) -> String {
    return fs::read_to_string(filename).expect("Something went wrong reading the file");
}

#[derive(Debug)]
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

impl Card {
    fn from_character(c: char) -> Option<Card> {
        match c {
            'A' => Some(Card::Ace),
            'K' => Some(Card::King),
            'Q' => Some(Card::Queen),
            'J' => Some(Card::Jack),
            'T' => Some(Card::Ten),
            '9' => Some(Card::Nine),
            '8' => Some(Card::Eight),
            '7' => Some(Card::Seven),
            '6' => Some(Card::Six),
            '5' => Some(Card::Five),
            '4' => Some(Card::Four),
            '3' => Some(Card::Three),
            '2' => Some(Card::Two),
            _ => None,
        }
    }

    fn value(&self, trick: bool) -> u32 {
        match self {
            Card::Ace => 14,
            Card::King => 13,
            Card::Queen => 12,
            Card::Jack => {
                if trick {
                    1
                } else {
                    11
                }
            }
            Card::Ten => 10,
            Card::Nine => 9,
            Card::Eight => 8,
            Card::Seven => 7,
            Card::Six => 6,
            Card::Five => 5,
            Card::Four => 4,
            Card::Three => 3,
            Card::Two => 2,
        }
    }
}

#[derive(Debug)]
enum SetType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPairs,
    OnePair,
    HighCard,
}

impl SetType {
    fn value(&self) -> u32 {
        match self {
            SetType::FiveOfAKind => 6,
            SetType::FourOfAKind => 5,
            SetType::FullHouse => 4,
            SetType::ThreeOfAKind => 3,
            SetType::TwoPairs => 2,
            SetType::OnePair => 1,
            SetType::HighCard => 0,
        }
    }
}

fn get_card(card: char) -> Card {
    return Card::from_character(card).unwrap();
}

fn get_set_type(set: &str, trick: bool) -> SetType {
    let mut cards: HashMap<char, u32> = HashMap::new();

    set.chars().for_each(|card| {
        cards
            .entry(card)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    });

    let unique_cards: u32 = cards.len() as u32;
    let contains_jack: bool = cards.contains_key(&'J');

    if unique_cards == 1 {
        return SetType::FiveOfAKind;
    }

    if unique_cards == 2 {
        let mut counts: Vec<&u32> = cards.values().collect();
        counts.sort();

        if *counts[0] == 2 && *counts[1] == 3 {
            if trick && contains_jack {
                return SetType::FiveOfAKind;
            }
            return SetType::FullHouse;
        }

        if *counts[0] == 1 && *counts[1] == 4 {
            if trick && contains_jack {
                return SetType::FiveOfAKind;
            }
            return SetType::FourOfAKind;
        }
    }

    if unique_cards == 3 {
        let mut counts: Vec<&u32> = cards.values().collect();
        counts.sort();

        if *counts[0] == 1 && *counts[1] == 1 && *counts[2] == 3 {
            if trick && contains_jack {
                return SetType::FourOfAKind;
            }
            return SetType::ThreeOfAKind;
        }

        if *counts[0] == 1 && *counts[1] == 2 && *counts[2] == 2 {
            if trick && contains_jack {
                let j_count: &u32 = cards.get(&'J').unwrap();
                if *j_count == 2 {
                    return SetType::FourOfAKind;
                }
                if *j_count == 1 {
                    return SetType::FullHouse;
                }
            }
            return SetType::TwoPairs;
        }
    }

    if unique_cards == 4 {
        if trick && contains_jack {
            return SetType::ThreeOfAKind;
        }
        return SetType::OnePair;
    }

    if trick && contains_jack {
        return SetType::OnePair;
    }

    return SetType::HighCard;
}

fn compare_set(set1: &str, set2: &str, trick: bool) -> Ordering {
    let set1_type = get_set_type(set1, trick);
    let set2_type = get_set_type(set2, trick);

    if set1_type.value() > set2_type.value() {
        return Ordering::Greater;
    }
    if set1_type.value() < set2_type.value() {
        return Ordering::Less;
    }

    for i in 0..set1.len() {
        let set1_card = get_card(set1.as_bytes()[i] as char);
        let set2_card = get_card(set2.as_bytes()[i] as char);

        if set1_card.value(trick) > set2_card.value(trick) {
            return Ordering::Greater;
        }
        if set1_card.value(trick) < set2_card.value(trick) {
            return Ordering::Less;
        }
    }

    return Ordering::Equal;
}

fn get_set_and_bids(filename: &str) -> Vec<(String, u32)> {
    let file: String = iterate_file(filename);

    let set_and_bids: Vec<(String, u32)> = file
        .lines()
        .filter_map(|set_and_bid| {
            let parts: Vec<&str> = set_and_bid.split_whitespace().collect();
            if parts.len() == 2 {
                let set = parts[0].to_string();
                let bid = parts[1].parse::<u32>().ok()?;
                Some((set, bid))
            } else {
                None
            }
        })
        .collect();

    return set_and_bids;
}

fn get_result(set_and_bids: &Vec<(String, u32)>) -> u32 {
    set_and_bids
        .iter()
        .enumerate()
        .fold(0, |acc, (index, (_set, bid))| {
            return acc + bid * (index as u32 + 1);
        })
}

pub fn part1(filename: &str) -> u32 {
    let mut set_and_bids: Vec<(String, u32)> = get_set_and_bids(filename);

    set_and_bids.sort_by(|(set1, _bid1), (set2, _bid2)| {
        return compare_set(set1, set2, false);
    });

    get_result(&set_and_bids)
}

pub fn part2(filename: &str) -> u32 {
    let mut set_and_bids: Vec<(String, u32)> = get_set_and_bids(filename);

    set_and_bids.sort_by(|(set1, _bid1), (set2, _bid2)| {
        return compare_set(set1, set2, true);
    });

    get_result(&set_and_bids)
}
