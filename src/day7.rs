use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
    str::FromStr,
};
#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
enum Card {
    Jack, // Jack is Joker amd weakest
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl TryFrom<char> for Card {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '2' => Ok(Card::Two),
            '3' => Ok(Card::Three),
            '4' => Ok(Card::Four),
            '5' => Ok(Card::Five),
            '6' => Ok(Card::Six),
            '7' => Ok(Card::Seven),
            '8' => Ok(Card::Eight),
            '9' => Ok(Card::Nine),
            'T' => Ok(Card::Ten),
            'J' => Ok(Card::Jack),
            'Q' => Ok(Card::Queen),
            'K' => Ok(Card::King),
            'A' => Ok(Card::Ace),
            _ => Err(anyhow::anyhow!("Invalid card: {}", value)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
enum Hand {
    HighCard,
    Pair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl TryFrom<&[Card]> for Hand {
    type Error = anyhow::Error;
    fn try_from(cards: &[Card]) -> Result<Self, Self::Error> {
        if cards.len() != 5 {
            return Err(anyhow::anyhow!("Invalid number of cards: {}", cards.len()));
        }
        let mut groups = HashMap::with_capacity(5);
        for card in cards {
            groups.entry(*card).and_modify(|v| *v += 1).or_insert(1);
        }
        let number_of_jacks = groups.get(&Card::Jack).copied().unwrap_or(0);
        let mut counts: Vec<_> = groups.values().copied().collect();
        counts.sort();
        let hand = match counts.as_slice() {
            [1, 1, 1, 1, 1] => {
                if number_of_jacks >= 1 {
                    Hand::Pair
                } else {
                    Hand::HighCard
                }
            }
            [1, 1, 1, 2] => {
                if number_of_jacks >= 1 {
                    Hand::ThreeOfAKind
                } else {
                    Hand::Pair
                }
            }
            [1, 2, 2] => {
                if number_of_jacks == 1 {
                    Hand::FullHouse
                } else if number_of_jacks == 2 {
                    Hand::FourOfAKind
                } else {
                    Hand::TwoPairs
                }
            }
            [1, 1, 3] => {
                if number_of_jacks >= 1 {
                    Hand::FourOfAKind
                } else {
                    Hand::ThreeOfAKind
                }
            }
            [2, 3] => {
                if number_of_jacks >= 1 {
                    Hand::FiveOfAKind
                } else {
                    Hand::FullHouse
                }
            }
            [1, 4] => {
                if number_of_jacks >= 1 {
                    Hand::FiveOfAKind
                } else {
                    Hand::FourOfAKind
                }
            }
            [5] => Hand::FiveOfAKind,

            _ => unreachable!("INvalid hand: {:?}", counts),
        };
        Ok(hand)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Game {
    bet: u64,
    cards: Vec<Card>,
    hand: Hand,
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut l = s.split_ascii_whitespace();
        let cards = l
            .next()
            .unwrap()
            .chars()
            .map(Card::try_from)
            .collect::<Result<Vec<_>, _>>()?;
        let bet: u64 = l.next().unwrap().parse()?;
        let hand = cards.as_slice().try_into()?;
        Ok(Game { cards, bet, hand })
    }
}

impl PartialOrd for Game {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.hand.cmp(&other.hand) {
            std::cmp::Ordering::Equal => {
                for idx in 0..self.cards.len() {
                    match self.cards[idx].cmp(&other.cards[idx]) {
                        std::cmp::Ordering::Equal => continue,
                        ord => return Some(ord),
                    }
                }
                Some(std::cmp::Ordering::Equal)
            }
            ord => return Some(ord),
        }
    }
}

impl Ord for Game {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub fn seventh_task_2(f: impl BufRead) -> u64 {
    let mut games: Vec<Game> = f.lines().map(|l| l.unwrap().parse().unwrap()).collect();
    games.sort();
    println!("Games: {:?}", games);
    games
        .into_iter()
        .enumerate()
        .fold(0, |acc, (idx, game)| acc + (idx as u64 + 1) * game.bet)
}
