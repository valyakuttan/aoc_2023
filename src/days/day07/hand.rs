use std::collections::HashMap;
use std::fmt;

use super::card::Card;
use HandType::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hand {
    cards: [Card; 5],
}

impl Hand {
    pub fn new(cards: [Card; 5]) -> Self {
        Self { cards }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        let xs: Option<Vec<_>> = s.chars().map(Card::from_char).collect();

        if let Some(v) = xs {
            if v.len() == 5 {
                let cards = [v[0], v[1], v[2], v[3], v[4]];
                return Some(Hand { cards });
            } else {
                return None;
            }
        }
        None
    }
}
impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hand: String = self.cards.iter().map(|c| c.to_char()).collect();
        write!(f, "{}", hand)
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let sht = self.hand_type();
        let oht = other.hand_type();
        if sht == oht {
            self.cards.cmp(&other.cards)
        } else {
            sht.cmp(&oht)
        }
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Hand {
    fn hand_type(&self) -> HandType {
        let mut hmap = HashMap::new();
        for c in self.cards {
            hmap.entry(c).and_modify(|v| *v += 1).or_insert(1);
        }

        let mut card_counts: Vec<_> = hmap.iter().map(|(c, n)| (*c, *n)).collect();
        card_counts.sort_by(|a, b| {
            if b.1 == a.1 {
                b.0.cmp(&a.0)
            } else {
                b.1.cmp(&a.1)
            }
        });

        match card_counts.as_slice() {
            [(c, 5)] => FiveOfAKind(*c),

            [(c1, 4), (c2, 1)] => FourOfAKind {
                four: *c1,
                single: *c2,
            },

            [(c1, 3), (c2, 2)] => FullHouse {
                three: *c1,
                pair: *c2,
            },

            [(c1, 3), (c2, 1), (c3, 1)] => ThreeOfAKind {
                three: *c1,
                rest: [*c2, *c3],
            },

            [(c1, 2), (c2, 2), (c3, 1)] => TwoPair {
                pair_cards: [*c1, *c2],
                single: *c3,
            },

            [(c1, 2), (c2, 1), (c3, 1), (c4, 1)] => OnePair {
                pair: *c1,
                rest: [*c2, *c3, *c4],
            },

            _ => HighCard(self.cards),
        }
    }
}

#[derive(Debug, Eq)]
enum HandType {
    HighCard([Card; 5]),

    OnePair { pair: Card, rest: [Card; 3] },

    TwoPair { pair_cards: [Card; 2], single: Card },

    ThreeOfAKind { three: Card, rest: [Card; 2] },

    FullHouse { three: Card, pair: Card },

    FourOfAKind { four: Card, single: Card },

    FiveOfAKind(Card),
}

impl PartialEq for HandType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (HighCard(_), HighCard(_)) => true,
            (HighCard(_), _) => false,
            (_, HighCard(_)) => false,

            (OnePair { pair: _, rest: _ }, OnePair { pair: _, rest: _ }) => true,
            (OnePair { pair: _, rest: _ }, _) => false,
            (_, OnePair { pair: _, rest: _ }) => false,

            (
                TwoPair {
                    pair_cards: _,
                    single: _,
                },
                TwoPair {
                    pair_cards: _,
                    single: _,
                },
            ) => true,
            (
                TwoPair {
                    pair_cards: _,
                    single: _,
                },
                _,
            ) => false,
            (
                _,
                TwoPair {
                    pair_cards: _,
                    single: _,
                },
            ) => false,

            (ThreeOfAKind { three: _, rest: _ }, ThreeOfAKind { three: _, rest: _ }) => true,
            (ThreeOfAKind { three: _, rest: _ }, _) => false,
            (_, ThreeOfAKind { three: _, rest: _ }) => false,

            (FullHouse { three: _, pair: _ }, FullHouse { three: _, pair: _ }) => true,
            (FullHouse { three: _, pair: _ }, _) => false,
            (_, FullHouse { three: _, pair: _ }) => false,

            (FourOfAKind { four: _, single: _ }, FourOfAKind { four: _, single: _ }) => true,
            (FourOfAKind { four: _, single: _ }, _) => false,
            (_, FourOfAKind { four: _, single: _ }) => false,

            (FiveOfAKind(_), FiveOfAKind(_)) => true,
        }
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (HighCard(_), HighCard(_)) => std::cmp::Ordering::Equal,
            (HighCard(_), _) => std::cmp::Ordering::Less,
            (_, HighCard(_)) => std::cmp::Ordering::Greater,

            (OnePair { pair: _, rest: _ }, OnePair { pair: _, rest: _ }) => {
                std::cmp::Ordering::Equal
            }
            (OnePair { pair: _, rest: _ }, _) => std::cmp::Ordering::Less,
            (_, OnePair { pair: _, rest: _ }) => std::cmp::Ordering::Greater,

            (
                TwoPair {
                    pair_cards: _,
                    single: _,
                },
                TwoPair {
                    pair_cards: _,
                    single: _,
                },
            ) => std::cmp::Ordering::Equal,
            (
                TwoPair {
                    pair_cards: _,
                    single: _,
                },
                _,
            ) => std::cmp::Ordering::Less,
            (
                _,
                TwoPair {
                    pair_cards: _,
                    single: _,
                },
            ) => std::cmp::Ordering::Greater,

            (ThreeOfAKind { three: _, rest: _ }, ThreeOfAKind { three: _, rest: _ }) => {
                std::cmp::Ordering::Equal
            }
            (ThreeOfAKind { three: _, rest: _ }, _) => std::cmp::Ordering::Less,
            (_, ThreeOfAKind { three: _, rest: _ }) => std::cmp::Ordering::Greater,

            (FullHouse { three: _, pair: _ }, FullHouse { three: _, pair: _ }) => {
                std::cmp::Ordering::Equal
            }
            (FullHouse { three: _, pair: _ }, _) => std::cmp::Ordering::Less,
            (_, FullHouse { three: _, pair: _ }) => std::cmp::Ordering::Greater,

            (FourOfAKind { four: _, single: _ }, FourOfAKind { four: _, single: _ }) => {
                std::cmp::Ordering::Equal
            }
            (FourOfAKind { four: _, single: _ }, _) => std::cmp::Ordering::Less,
            (_, FourOfAKind { four: _, single: _ }) => std::cmp::Ordering::Greater,

            (FiveOfAKind(_), FiveOfAKind(_)) => std::cmp::Ordering::Equal,
        }
    }
}
impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn main() {
    let h1 = Hand::from_str("2AAAA").unwrap();
    let h2 = Hand::from_str("JJJJ2").unwrap();
    let ht1 = h1.hand_type();
    let ht2 = h2.hand_type();

    println!("{}", ht1 == ht2);
    println!("{:?}, {:?}", ht1, ht2);
}

#[cfg(test)]
mod test {
    use std::cmp::Ordering;

    use super::Card::*;
    use super::*;

    #[test]
    fn test_more_hand_order() {
        // 32T3K 765
        let h1 = Hand::from_str("2AAAA").unwrap();
        let h2 = Hand::from_str("JJJJ2").unwrap();
        let ht1 = h1.hand_type();
        let ht2 = h2.hand_type();
        assert_eq!(ht1, ht2);
        assert!(h1 <= h2);
        assert!(h1 < h2);
    }

    #[test]
    fn test_hand_order() {
        // 32T3K 765
        let hand1 = Hand::new([Three, Two, Ten, Three, King]);

        // T55J5 684
        let hand2 = Hand::new([Ten, Five, Five, Jack, Five]);

        // KK677 28
        let hand3 = Hand::new([King, King, Six, Seven, Seven]);

        // KTJJT 220
        let hand4 = Hand::new([King, Ten, Jack, Jack, Ten]);

        // QQQJA 483
        let hand5 = Hand::new([Queen, Queen, Queen, Jack, Ace]);

        let sorted_hands = [
            hand1.clone(),
            hand4.clone(),
            hand3.clone(),
            hand2.clone(),
            hand5.clone(),
        ];

        let mut hands = [hand1, hand2, hand3, hand4, hand5];
        hands.sort();

        assert_eq!(hands, sorted_hands);

        // 32T3K 765
        let hand1 = [Three, Two, Ten, Three, King];

        // T55J5 684
        let hand2 = [Ten, Five, Five, Jack, Five];

        // KK677 28
        let hand3 = [King, King, Six, Seven, Seven];
        // KTJJT 220
        let hand4 = [King, Ten, Jack, Jack, Ten];

        // QQQJA 483
        let hand5 = [Queen, Queen, Queen, Jack, Ace];

        let sorted_hands = [hand1, hand2, hand5, hand4, hand3];

        let mut hands = [hand1, hand2, hand3, hand4, hand5];
        hands.sort();

        assert_eq!(hands, sorted_hands);

        let five = [Ace, Ace, Ace, Ace, Ace];
        let four = [Ace, Ace, Ace, Ace, Ten];
        let fiveq = [Queen, Queen, Queen, Queen, Queen];

        assert_eq!(five.cmp(&four), Ordering::Greater);
        assert_eq!(five.cmp(&five), Ordering::Equal);
        assert_eq!(four.cmp(&five), Ordering::Less);

        assert_eq!(five.cmp(&fiveq), Ordering::Greater);
    }

    #[test]
    fn test_hand_type() {
        let cards = [Ace, Ten, Jack, King, Queen];
        let high_cards = Hand { cards };

        assert_eq!(high_cards.hand_type(), HighCard(cards));

        let cards = [Ace, Ace, Jack, King, Queen];
        let one_pair = Hand { cards };

        assert_eq!(
            one_pair.hand_type(),
            OnePair {
                pair: Ace,
                rest: [King, Queen, Jack]
            }
        );

        let cards = [Ace, Ace, King, King, Queen];
        let two_pair = Hand { cards };

        assert_eq!(
            two_pair.hand_type(),
            TwoPair {
                pair_cards: [Ace, King],
                single: Queen
            }
        );

        let cards = [Ace, Ace, Ace, King, Queen];
        let three_of_a_kind = Hand { cards };

        assert_eq!(
            three_of_a_kind.hand_type(),
            ThreeOfAKind {
                three: Ace,
                rest: [King, Queen],
            },
        );

        let cards = [Ace, Ace, Ace, Queen, Queen];
        let full_house = Hand { cards };

        assert_eq!(
            full_house.hand_type(),
            FullHouse {
                three: Ace,
                pair: Queen
            }
        );

        let cards = [Ace, Ace, Ace, Ace, Queen];
        let four_of_a_kind = Hand { cards };

        assert_eq!(
            four_of_a_kind.hand_type(),
            FourOfAKind {
                four: Ace,
                single: Queen
            }
        );

        let cards = [Ace, Ace, Ace, Ace, Ace];
        let five_of_a_kind = Hand { cards };

        assert_eq!(five_of_a_kind.hand_type(), FiveOfAKind(Ace));
    }
}
