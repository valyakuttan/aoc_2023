use std::collections::HashMap;
use std::fmt;

use super::card::Card::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Card {
    Two = 2,
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

impl Card {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '2' => Some(Two),
            '3' => Some(Three),
            '4' => Some(Four),
            '5' => Some(Five),
            '6' => Some(Six),
            '7' => Some(Seven),
            '8' => Some(Eight),
            '9' => Some(Nine),
            'T' => Some(Ten),
            'J' => Some(Jack),
            'Q' => Some(Queen),
            'K' => Some(King),
            'A' => Some(Ace),
            _ => None,
        }
    }

    pub fn to_char(self) -> char {
        let card_to_char_map: HashMap<Card, char> = HashMap::from([
            (Two, '2'),
            (Three, '3'),
            (Four, '4'),
            (Five, '5'),
            (Six, '6'),
            (Seven, '7'),
            (Eight, '8'),
            (Nine, '9'),
            (Ten, 'T'),
            (Jack, 'J'),
            (Queen, 'Q'),
            (King, 'K'),
            (Ace, 'A'),
        ]);
        *card_to_char_map.get(&self).unwrap()
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_card_eq() {
        let c1 = Ace;
        let c2 = Ace;
        assert_eq!(c1, c2);
    }

    #[test]
    fn test_ord() {
        let cards = [
            Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace,
        ];

        for i in 0..13 {
            let c = &cards[i];
            assert!(cards[i..].iter().all(|x| x >= c));
        }
    }

    #[test]
    fn test_from_char() {
        let cs = "23456789TJQKA";
        let ms: Option<Vec<_>> = cs.chars().map(Card::from_char).collect();
        assert!(ms.is_some());

        let cards = [
            Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace,
        ];

        let ms = ms.unwrap();

        let f = |(a, b)| a == b;
        assert!(cards.iter().zip(ms.iter()).all(f));
    }

    #[test]
    fn test_card_value() {
        let c1 = Card::Two;
        assert_eq!(c1 as u32, 2);

        let c2 = Card::Ten;
        assert_eq!(c2 as u32, 10);

        let c3 = Card::Jack;
        assert_eq!(c3 as u32, 11);

        let c3 = Card::Queen;
        assert_eq!(c3 as u32, 12);

        let c4 = Card::King;
        assert_eq!(c4 as u32, 13);

        let c3 = Card::Ace;
        assert_eq!(c3 as u32, 14);
    }
}
