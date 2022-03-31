use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum PokerNumber {
    NumberCard { v: u8 },
    Jack,
    Queen,
    King,
    Ace,
}

impl PokerNumber {
    fn to_unsigned(&self) -> u8 {
        match self {
            PokerNumber::NumberCard { v } => *v,
            PokerNumber::Jack => 11,
            PokerNumber::Queen => 12,
            PokerNumber::King => 13,
            PokerNumber::Ace => 14,
        }
    }
}

impl Ord for PokerNumber {
    fn cmp(&self, other: &PokerNumber) -> Ordering {
        let self_n = self.to_unsigned();
        let other_n = other.to_unsigned();
        if self_n < other_n {
            Ordering::Less
        } else if self_n > other_n {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for PokerNumber {
    fn partial_cmp(&self, other: &PokerNumber) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum PokerSuite {
    Hearts,
    Clubs,
    Spades,
    Diamonds,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct PokerCard {
    number: PokerNumber,
    suite: PokerSuite,
}

impl Ord for PokerCard {
    fn cmp(&self, other: &PokerCard) -> Ordering {
        self.number.cmp(&other.number)
    }
}

impl PartialOrd for PokerCard {
    fn partial_cmp(&self, other: &PokerCard) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
pub struct PokerHand {
    pub cards: [PokerCard; 5],
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum PokerRank<'a> {
    HighCard { high: &'a PokerCard },
    OnePair { high: &'a PokerCard },
    TwoPairs { high: &'a PokerCard },
    ThreeOfAKind { high: &'a PokerCard },
    Straight { high: &'a PokerCard },
    Flush { high: &'a PokerCard },
    FullHouse { high: &'a PokerCard },
    FourOfAKind { high: &'a PokerCard },
    StraightFlush { high: &'a PokerCard },
    RoyalFlush { high: &'a PokerCard },
}

impl<'a> PokerRank<'a> {
    fn to_unsigned(&self) -> u8 {
        match self {
            PokerRank::HighCard { high } => 1,
            PokerRank::OnePair { high } => 2,
            PokerRank::TwoPairs { high } => 3,
            PokerRank::ThreeOfAKind { high } => 4,
            PokerRank::Straight { high } => 5,
            PokerRank::Flush { high } => 6,
            PokerRank::FullHouse { high } => 7,
            PokerRank::FourOfAKind { high } => 8,
            PokerRank::StraightFlush { high } => 9,
            PokerRank::RoyalFlush { high } => 10,
        }
    }

    fn get_high(&self) -> &PokerCard {
        match self {
            PokerRank::HighCard { high } => high,
            PokerRank::OnePair { high } => high,
            PokerRank::TwoPairs { high } => high,
            PokerRank::ThreeOfAKind { high } => high,
            PokerRank::Straight { high } => high,
            PokerRank::Flush { high } => high,
            PokerRank::FullHouse { high } => high,
            PokerRank::FourOfAKind { high } => high,
            PokerRank::StraightFlush { high } => high,
            PokerRank::RoyalFlush { high } => high,
        }
    }
}

impl<'a> Ord for PokerRank<'a> {
    fn cmp(&self, other: &PokerRank) -> Ordering {
        let self_n = self.to_unsigned();
        let other_n = other.to_unsigned();
        if self_n < other_n {
            Ordering::Less
        } else if self_n > other_n {
            Ordering::Greater
        } else {
            let self_h = self.get_high();
            let other_h = other.get_high();
            if self_h < other_h {
                Ordering::Less
            } else if self_h > other_h {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        }
    }
}

impl<'a> PartialOrd for PokerRank<'a> {
    fn partial_cmp(&self, other: &PokerRank) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct HandRank<'a> {
    rank: PokerRank<'a>,
    highests: Vec<PokerCard>,
}

impl<'a> Ord for HandRank<'a> {
    fn cmp(&self, other: &HandRank) -> Ordering {
        if self.rank < other.rank {
            Ordering::Less
        } else if self.rank > other.rank {
            Ordering::Greater
        } else {
            if self.highests < other.highests {
                Ordering::Less
            } else if self.highests > other.highests {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        }
    }
}

impl<'a> PartialOrd for HandRank<'a> {
    fn partial_cmp(&self, other: &HandRank) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn look_for_value<'a, K, V: std::cmp::PartialEq>(
    value: V,
    map: &'a HashMap<K, V>,
) -> Option<Vec<(&'a K, &V)>> {
    let finds: Vec<(&'a K, &V)> = map
        .iter()
        .filter_map(|(key, val)| {
            if val == &value {
                Some((key, val))
            } else {
                None
            }
        })
        .collect();
    if finds.len() > 0 {
        Some(finds)
    } else {
        None
    }
}

impl PokerHand {
    fn cards_numbers<'a>(self: &'a PokerHand) -> Vec<&'a PokerNumber> {
        self.cards.iter().map(|c| &c.number).collect()
    }

    fn highest_card(&self) -> &PokerCard {
        self.cards.iter().max().unwrap()
    }

    fn count_cards(self: &PokerHand) -> HashMap<PokerNumber, u8> {
        let mut cards_count = HashMap::new();
        for n in self.cards.iter().map(|c| c.number) {
            *cards_count.entry(n).or_insert(0) += 1;
        }
        cards_count
    }

    fn is_some_of_kind(self: &PokerHand) -> Option<PokerRank> {
        let cards_count = self.count_cards();
        if let Some(pairs_vec) = look_for_value(4u8, &cards_count) {
            Some(PokerRank::FourOfAKind {
                high: self
                    .cards
                    .iter()
                    .filter(|c| &c.number == pairs_vec[0].0)
                    .next()
                    .unwrap(),
            })
        } else if let Some(pairs_vec) = look_for_value(3u8, &cards_count) {
            if let Some(_v) = look_for_value(2u8, &cards_count) {
                Some(PokerRank::FullHouse {
                    high: self
                        .cards
                        .iter()
                        .filter(|c| &c.number == pairs_vec[0].0)
                        .next()
                        .unwrap(),
                })
            } else {
                Some(PokerRank::ThreeOfAKind {
                    high: self
                        .cards
                        .iter()
                        .filter(|c| &c.number == pairs_vec[0].0)
                        .next()
                        .unwrap(),
                })
            }
        } else if let Some(pairs_vec) = look_for_value(2u8, &cards_count) {
            if pairs_vec.len() > 1 {
                if pairs_vec[0].0 > pairs_vec[1].0 {
                    Some(PokerRank::TwoPairs {
                        high: self
                            .cards
                            .iter()
                            .filter(|c| &c.number == pairs_vec[0].0)
                            .next()
                            .unwrap(),
                    })
                } else {
                    Some(PokerRank::TwoPairs {
                        high: self
                            .cards
                            .iter()
                            .filter(|c| &c.number == pairs_vec[1].0)
                            .next()
                            .unwrap(),
                    })
                }
            } else {
                Some(PokerRank::OnePair {
                    high: self
                        .cards
                        .iter()
                        .filter(|c| &c.number == pairs_vec[0].0)
                        .next()
                        .unwrap(),
                })
            }
        } else {
            Some(PokerRank::HighCard {
                high: self.highest_card(),
            })
        }
    }

    fn is_some_flush(self: &PokerHand) -> Option<PokerRank> {
        let first_suite = &self.cards[0].suite;
        if self.cards.iter().all(|c| &c.suite == first_suite) {
            let royal_ladder = vec![
                PokerNumber::Ace,
                PokerNumber::King,
                PokerNumber::Queen,
                PokerNumber::Jack,
                PokerNumber::NumberCard { v: 10 },
            ];
            let royal_ladder: HashSet<&PokerNumber> = royal_ladder.iter().collect();
            let cards_set: HashSet<&PokerNumber> = self.cards.iter().map(|c| &c.number).collect();
            if royal_ladder.symmetric_difference(&cards_set).count() == 0 {
                Some(PokerRank::RoyalFlush {
                    high: self.highest_card(),
                })
            } else {
                Some(PokerRank::Flush {
                    high: self.highest_card(),
                })
            }
        } else {
            None
        }
    }

    fn is_some_straight(self: &PokerHand) -> Option<PokerRank> {
        let mut sorted_cards = self.cards.clone();
        sorted_cards.sort();
        if sorted_cards
            .windows(2)
            .all(|cs| cs[1].number.to_unsigned() - cs[0].number.to_unsigned() == 1)
        {
            if let Some(high) = self.is_some_flush() {
                Some(PokerRank::StraightFlush {
                    high: self.highest_card(),
                })
            } else {
                Some(PokerRank::Straight {
                    high: self.highest_card(),
                })
            }
        } else {
            None
        }
    }

    pub fn rank_hand(self: &mut PokerHand) -> HandRank {
        let flushed = self.is_some_flush();
        let straighted = self.is_some_straight();
        let ranked = self.is_some_of_kind();

        let mut sorted_cards = self.cards.clone();
        sorted_cards.sort();
        sorted_cards.reverse();
        let highests_cards: Vec<PokerCard> = sorted_cards.iter().map(|c| c.clone()).collect();

        if let Some(PokerRank::RoyalFlush { high: v }) = flushed {
            HandRank {
                rank: flushed.unwrap(),
                highests: highests_cards,
            }
        } else if let Some(PokerRank::StraightFlush { high: v }) = straighted {
            HandRank {
                rank: straighted.unwrap(),
                highests: highests_cards,
            }
        } else if let Some(PokerRank::FourOfAKind { high: v }) = ranked {
            HandRank {
                rank: ranked.unwrap(),
                highests: highests_cards,
            }
        } else if let Some(PokerRank::FullHouse { high: v }) = ranked {
            HandRank {
                rank: ranked.unwrap(),
                highests: highests_cards,
            }
        } else if let Some(PokerRank::Flush { high: v }) = flushed {
            HandRank {
                rank: flushed.unwrap(),
                highests: highests_cards,
            }
        } else if let Some(PokerRank::Straight { high: v }) = straighted {
            HandRank {
                rank: straighted.unwrap(),
                highests: highests_cards,
            }
        } else if let Some(PokerRank::ThreeOfAKind { high: v }) = ranked {
            HandRank {
                rank: ranked.unwrap(),
                highests: highests_cards,
            }
        } else if let Some(PokerRank::TwoPairs { high: v }) = ranked {
            HandRank {
                rank: ranked.unwrap(),
                highests: highests_cards,
            }
        } else if let Some(PokerRank::OnePair { high: v }) = ranked {
            HandRank {
                rank: ranked.unwrap(),
                highests: highests_cards,
            }
        } else {
            HandRank {
                rank: ranked.unwrap(),
                highests: highests_cards,
            }
        }
    }
}

pub fn parse_card(card_str: String) -> Option<PokerCard> {
    let mut card_chars = card_str.chars();

    let number = match card_chars.next() {
        Some('A') => PokerNumber::Ace,
        Some('K') => PokerNumber::King,
        Some('Q') => PokerNumber::Queen,
        Some('J') => PokerNumber::Jack,
        Some('T') => PokerNumber::NumberCard { v: 10 },
        Some(n) => {
            if ('2'..='9').contains(&n) {
                let n = n.to_digit(10).unwrap() as u8;
                PokerNumber::NumberCard { v: n }
            } else {
                return None;
            }
        }
        _ => return None,
    };

    let suite = match card_chars.next() {
        Some('H') => PokerSuite::Hearts,
        Some('C') => PokerSuite::Clubs,
        Some('S') => PokerSuite::Spades,
        Some('D') => PokerSuite::Diamonds,
        _ => return None,
    };

    Some(PokerCard { number, suite })
}

pub fn parse_hand(hand_str: Vec<&str>) -> Option<PokerHand> {
    let mut cards: [PokerCard; 5] = [parse_card("AS".to_string()).unwrap(); 5];
    let parsed_cards: Vec<Option<PokerCard>> = hand_str[0..5]
        .iter()
        .map(|s| parse_card(s.to_string()))
        .collect();
    for i in 0..5 {
        match parsed_cards[i] {
            Some(c) => cards[i] = c,
            None => return None,
        }
    }
    Some(PokerHand { cards })
}
