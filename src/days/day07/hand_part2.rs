use std::collections::HashMap;
use std::fmt;

use super::card_part2::{Card, Card::*};
use HandType::*;

#[derive(Debug, Clone, Eq)]
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

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let sht = self.hand_type();
        let oht = other.hand_type();
        if sht.eq(&oht) {
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

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        let sht = self.hand_type();
        let oht = other.hand_type();
        if sht.eq(&oht) {
            self.cards.eq(&other.cards)
        } else {
            false
        }
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hand: String = self.cards.iter().map(|c| c.to_char()).collect();
        write!(f, "{}", hand)
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
            [(_, 5)] => FiveOfAKind,

            [(c1, 4), (c2, 1)] => {
                if *c1 == Card::Jack || *c2 == Card::Jack {
                    FiveOfAKind
                } else {
                    FourOfAKind
                }
            }

            [(c1, 3), (c2, 2)] => {
                if *c1 == Card::Jack || *c2 == Card::Jack {
                    FiveOfAKind
                } else {
                    FullHouse
                }
            }

            [(c1, 3), (c2, 1), (c3, 1)] => {
                if *c1 == Card::Jack || *c2 == Card::Jack || *c3 == Card::Jack {
                    FourOfAKind
                } else {
                    ThreeOfAKind
                }
            }

            [(c1, 2), (c2, 2), (c3, 1)] => {
                if *c1 == Card::Jack || *c2 == Card::Jack {
                    FourOfAKind
                } else if *c3 == Card::Jack {
                    FullHouse
                } else {
                    TwoPair
                }
            }
            [(_, 2), (_, 1), (_, 1), (_, 1)] => {
                if self.cards.contains(&Card::Jack) {
                    ThreeOfAKind
                } else {
                    OnePair
                }
            }

            _ => {
                if self.cards.contains(&Card::Jack) {
                    OnePair
                } else {
                    HighCard
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

pub fn main() {
    // 32T3K
    let hand1 = Hand::new([Three, Two, Ten, Three, King]);

    // KK677 28
    let hand2 = Hand::new([King, King, Six, Seven, Seven]);

    // T55J5 684
    let hand3 = Hand::new([Ten, Five, Five, Jack, Five]);

    // QQQJA 483
    let hand4 = Hand::new([Queen, Queen, Queen, Jack, Ace]);

    // KTJJT 220
    let hand5 = Hand::new([King, Ten, Jack, Jack, Ten]);

    let mut hands = [hand1, hand2, hand3, hand4, hand5];
    hands.sort();

    for h in hands {
        println!("{}", h);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::cmp::Ordering;

    #[test]
    fn test_sample_input() {
        // 2345A 1
        let h1 = Hand::from_str("2345A").unwrap();
        assert_eq!(h1.hand_type(), HighCard);

        // Q2KJJ 13
        let h2 = Hand::from_str("Q2KJJ").unwrap();
        assert_eq!(h2.hand_type(), ThreeOfAKind);

        // Q2Q2Q 19
        let h3 = Hand::from_str("Q2Q2Q").unwrap();
        assert_eq!(h3.hand_type(), FullHouse);

        // T3T3J 17
        let h4 = Hand::from_str("T3T3J").unwrap();
        assert_eq!(h4.hand_type(), FullHouse);

        // T3Q33 11
        let h5 = Hand::from_str("T3Q33").unwrap();
        assert_eq!(h5.hand_type(), ThreeOfAKind);

        // 2345J 3
        let h6 = Hand::from_str("2345J").unwrap();
        assert_eq!(h6.hand_type(), OnePair);

        // J345A 2
        let h7 = Hand::from_str("J345A").unwrap();
        assert_eq!(h7.hand_type(), OnePair);

        // 32T3K 5
        let h8 = Hand::from_str("32T3K").unwrap();
        assert_eq!(h8.hand_type(), OnePair);

        // T55J5 29
        let h9 = Hand::from_str("T55J5").unwrap();
        assert_eq!(h9.hand_type(), FourOfAKind);

        // KK677 7
        let h10 = Hand::from_str("KK677").unwrap();
        assert_eq!(h10.hand_type(), TwoPair);

        // KTJJT 34
        let h11 = Hand::from_str("KTJJT").unwrap();
        assert_eq!(h11.hand_type(), FourOfAKind);

        // QQQJA 31
        let h12 = Hand::from_str("QQQJA").unwrap();
        assert_eq!(h12.hand_type(), FourOfAKind);

        // JJJJJ 37
        let h13 = Hand::from_str("JJJJJ").unwrap();
        assert_eq!(h13.hand_type(), FiveOfAKind);

        // JAAAA 43
        let h14 = Hand::from_str("JAAAA").unwrap();
        assert_eq!(h14.hand_type(), FiveOfAKind);

        // AAAAJ 59
        let h15 = Hand::from_str("AAAAJ").unwrap();
        assert_eq!(h15.hand_type(), FiveOfAKind);

        // AAAAA 61
        let h16 = Hand::from_str("AAAAA").unwrap();
        assert_eq!(h16.hand_type(), FiveOfAKind);

        // 2AAAA 23
        let h17 = Hand::from_str("2AAAA").unwrap();
        assert_eq!(h17.hand_type(), FourOfAKind);

        // 2JJJJ 53
        let h18 = Hand::from_str("2JJJJ").unwrap();
        assert_eq!(h18.hand_type(), FiveOfAKind);

        // JJJJ2 41
        let h19 = Hand::from_str("JJJJ2").unwrap();
        assert_eq!(h18.hand_type(), FiveOfAKind);

        let sorted: [(Hand, u32); 19] = [
            (h1.clone(), 1),
            (h7.clone(), 2),
            (h6.clone(), 3),
            (h8.clone(), 5),
            (h10.clone(), 7),
            (h5.clone(), 11),
            (h2.clone(), 13),
            (h4.clone(), 17),
            (h3.clone(), 19),
            (h17.clone(), 23),
            (h9.clone(), 29),
            (h12.clone(), 31),
            (h11.clone(), 34),
            (h13.clone(), 37),
            (h19.clone(), 41),
            (h14.clone(), 43),
            (h18.clone(), 53),
            (h15.clone(), 59),
            (h16.clone(), 61),
        ];
        let mut hands = sorted.clone();
        hands.sort_by(|a, b| a.0.cmp(&b.0));

        assert_eq!(hands, sorted);
        
        let total_bid: u32 = hands.iter().enumerate().map(|(i, p)| p.1 * (i + 1) as u32).sum();
        assert_eq!(total_bid, 6839);
        
    }

    #[test]
    fn test_more_hand_order() {
        // 32T3K 765
        let h1 = Hand::from_str("2AAAA").unwrap();
        let h2 = Hand::from_str("JJJJ2").unwrap();
        let ht1 = h1.hand_type();
        let ht2 = h2.hand_type();
        assert_ne!(ht1, ht2);
        assert_eq!(ht1, FourOfAKind);
        assert_eq!(ht2, FiveOfAKind);

        assert!(ht1 < ht2);
        assert!(ht1 <= ht2);

        assert!(h1 <= h2);
        assert!(h1 < h2);
    }

    #[test]
    fn test_hand_order() {
        // JTAKQ 220
        let hand0 = Hand::from_str("JTAKQ").unwrap();
        assert_eq!(hand0.hand_type(), OnePair);

        // 32T3K 765
        let hand1 = Hand::from_str("32T3K").unwrap();
        assert_eq!(hand1.hand_type(), OnePair);

        // KK677 28
        let hand2 = Hand::from_str("KK677").unwrap();
        assert_eq!(hand2.hand_type(), TwoPair);

        // T55J5 684
        let hand3 = Hand::from_str("T55J5").unwrap();
        assert_eq!(hand3.hand_type(), FourOfAKind);

        // QQQJA 483
        let hand4 = Hand::from_str("T55J5").unwrap();
        assert_eq!(hand4.hand_type(), FourOfAKind);

        // KTJJT 220
        let hand5 = Hand::from_str("T55J5").unwrap();
        assert_eq!(hand5.hand_type(), FourOfAKind);

        let mut hands = [
            hand0.clone(),
            hand1.clone(),
            hand2.clone(),
            hand3.clone(),
            hand4.clone(),
            hand5.clone(),
        ];
        hands.sort();

        let sorted = [
            hand0.clone(),
            hand1.clone(),
            hand2.clone(),
            hand3.clone(),
            hand4.clone(),
            hand5.clone(),
        ];

        assert_eq!(hands, sorted);

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

        assert_eq!(high_cards.hand_type(), OnePair);

        let cards = [Ace, Ace, Jack, King, Queen];
        let one_pair = Hand { cards };

        assert_eq!(one_pair.hand_type(), ThreeOfAKind);

        let cards = [Ace, Ace, King, King, Queen];
        let two_pair = Hand { cards };

        assert_eq!(two_pair.hand_type(), TwoPair);

        let cards = [Ace, Ace, Ace, King, Queen];
        let three_of_a_kind = Hand { cards };

        assert_eq!(three_of_a_kind.hand_type(), ThreeOfAKind);

        let cards = [Ace, Ace, Ace, Queen, Queen];
        let full_house = Hand { cards };

        assert_eq!(full_house.hand_type(), FullHouse);

        let cards = [Ace, Ace, Ace, Ace, Queen];
        let four_of_a_kind = Hand { cards };

        assert_eq!(four_of_a_kind.hand_type(), FourOfAKind);

        let cards = [Ace, Ace, Ace, Ace, Ace];
        let five_of_a_kind = Hand { cards };

        assert_eq!(five_of_a_kind.hand_type(), FiveOfAKind);
    }
}
