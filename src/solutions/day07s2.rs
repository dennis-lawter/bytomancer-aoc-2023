use super::final_answer;
use super::input_raw;

const DAY: u8 = 7;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq)]
enum Camel2Card {
    J = 0,
    C2 = 1,
    C3 = 2,
    C4 = 3,
    C5 = 4,
    C6 = 5,
    C7 = 6,
    C8 = 7,
    C9 = 8,
    T = 9,
    Q = 10,
    K = 11,
    A = 12,
}
const JOKER_REPLACEMENTS: [Camel2Card; 12] = [
    Camel2Card::C2,
    Camel2Card::C3,
    Camel2Card::C4,
    Camel2Card::C5,
    Camel2Card::C6,
    Camel2Card::C7,
    Camel2Card::C8,
    Camel2Card::C9,
    Camel2Card::T,
    Camel2Card::Q,
    Camel2Card::K,
    Camel2Card::A,
];
impl Ord for Camel2Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as u8).cmp(&(*other as u8))
    }
}
impl Camel2Card {
    fn from_char(c: char) -> Option<Self> {
        match c {
            'A' => Some(Camel2Card::A),
            'K' => Some(Camel2Card::K),
            'Q' => Some(Camel2Card::Q),
            'J' => Some(Camel2Card::J),
            'T' => Some(Camel2Card::T),
            '9' => Some(Camel2Card::C9),
            '8' => Some(Camel2Card::C8),
            '7' => Some(Camel2Card::C7),
            '6' => Some(Camel2Card::C6),
            '5' => Some(Camel2Card::C5),
            '4' => Some(Camel2Card::C4),
            '3' => Some(Camel2Card::C3),
            '2' => Some(Camel2Card::C2),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
enum Camel2HandType {
    HighCard = 0,
    OnePair = 1,
    TwoPair = 2,
    ThreeOfAKind = 3,
    FullHouse = 4,
    FourOfAKind = 5,
    FiveOfAKind = 6,
}

impl Ord for Camel2HandType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as u64).cmp(&(*other as u64))
    }
}

#[derive(Debug, PartialOrd, PartialEq, Eq)]
struct Camel2Hand {
    cards: Vec<Camel2Card>,
    bid: u64,
    hand_type: Camel2HandType,
    score: u64,
}
impl Ord for Camel2Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.hand_type == other.hand_type {
            // hands match, go through cards
            for i in 0..5 {
                if self.cards[i] != other.cards[i] {
                    return self.cards[i].cmp(&other.cards[i]);
                }
            }
        }
        self.hand_type.cmp(&other.hand_type)
    }
}
impl Camel2Hand {
    fn from_string(line: &String) -> Self {
        let parts: Vec<String> = line.split(" ").map(str::to_owned).collect();
        let bid = str::parse::<u64>(parts[1].as_str()).unwrap();
        let cards: Vec<Camel2Card> = parts[0]
            .chars()
            .map(|c| Camel2Card::from_char(c).unwrap())
            .collect();

        Self {
            cards: cards.clone(),
            bid,
            hand_type: Self::determine_type_with_jokers(&cards),
            score: 0,
        }
    }
    fn determine_type_with_jokers(cards: &Vec<Camel2Card>) -> Camel2HandType {
        let mut best_type = Camel2HandType::HighCard;
        if cards.contains(&Camel2Card::J) {
            for i in 0..5 {
                if cards[i] == Camel2Card::J {
                    for new_card in JOKER_REPLACEMENTS {
                        let mut permutation = cards.clone();
                        permutation[i] = new_card;
                        let test_type = Self::determine_type_with_jokers(&permutation);
                        if test_type > best_type {
                            best_type = test_type;
                        }
                    }
                }
            }
        } else {
            return Self::determine_type_concrete(cards);
        }

        best_type
    }
    fn determine_type_concrete(cards: &Vec<Camel2Card>) -> Camel2HandType {
        let mut sorted_hand = cards.clone();
        sorted_hand.sort();

        if sorted_hand[0] == sorted_hand[4] {
            Camel2HandType::FiveOfAKind
        } else if sorted_hand[0] == sorted_hand[3] || sorted_hand[1] == sorted_hand[4] {
            Camel2HandType::FourOfAKind
        } else if (sorted_hand[0] == sorted_hand[2] && sorted_hand[3] == sorted_hand[4])
            || (sorted_hand[0] == sorted_hand[1] && sorted_hand[2] == sorted_hand[4])
        {
            Camel2HandType::FullHouse
        } else if sorted_hand[0] == sorted_hand[2]
            || sorted_hand[1] == sorted_hand[3]
            || sorted_hand[2] == sorted_hand[4]
        {
            Camel2HandType::ThreeOfAKind
        } else if (sorted_hand[0] == sorted_hand[1] && sorted_hand[2] == sorted_hand[3])
            || (sorted_hand[0] == sorted_hand[1] && sorted_hand[3] == sorted_hand[4])
            || (sorted_hand[1] == sorted_hand[2] && sorted_hand[3] == sorted_hand[4])
        {
            Camel2HandType::TwoPair
        } else if sorted_hand[0] == sorted_hand[1]
            || sorted_hand[1] == sorted_hand[2]
            || sorted_hand[2] == sorted_hand[3]
            || sorted_hand[3] == sorted_hand[4]
        {
            Camel2HandType::OnePair
        } else {
            Camel2HandType::HighCard
        }
    }
}

async fn input2(example: bool) -> Vec<Camel2Hand> {
    let raw = input_raw(DAY, example).await;
    let lines: Vec<String> = raw
        .lines()
        .map(|item| item.to_owned())
        .filter(|item| item.len() > 0)
        .collect();

    let mut hands = vec![];

    for line in lines {
        hands.push(Camel2Hand::from_string(&line))
    }

    hands
}

pub async fn d07s2(submit: bool, example: bool) {
    let mut hands = input2(example).await;

    let hand_sort_closure = |left: &Camel2Hand, right: &Camel2Hand| -> std::cmp::Ordering {
        // println!("\nTEST: {:?} vs {:?}", left, right);
        if (left.hand_type as u64) == (right.hand_type as u64) {
            for i in 0..5 {
                if left.cards[i] != right.cards[i] {
                    // println!("PER CARD Result: {:?}", left.cards[i].cmp(&right.cards[i]));
                    return left.cards[i].cmp(&right.cards[i]);
                }
            }
        }
        // println!("DEFAULT: {:?}", left.hand_type.cmp(&right.hand_type));
        left.hand_type.cmp(&right.hand_type)
    };

    hands.sort_by(hand_sort_closure);

    for i in 0..hands.len() {
        hands[i].score = hands[i].bid * (i as u64 + 1);
    }

    // hands.sort();

    let mut accum = 0;

    for hand in hands {
        println!("{:?}", hand);
        accum += hand.score;
    }
    final_answer(accum, submit, DAY, 2).await;
}
