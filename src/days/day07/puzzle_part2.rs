/// -- Part Two ---
// To make things a little more interesting, the Elf introduces one additional rule. Now, J cards are jokers - wildcards
// that can act like whatever card would make the hand the strongest type possible.

// To balance this, J cards are now the weakest individual cards, weaker even than 2. The other cards stay in the same
// order: A, K, Q, T, 9, 8, 7, 6, 5, 4, 3, 2, J.

// J cards can pretend to be whatever card is best for the purpose of determining hand type; for example, QJJQ2 is now
// considered four of a kind. However, for the purpose of breaking ties between two hands of the same type, J is
// always treated as J, not the card it's pretending to be: JKKK2 is weaker than QQQQ2 because J is weaker than Q.

// Now, the above example goes very differently:

// 32T3K 765
// T55J5 684
// KK677 28
// KTJJT 220
// QQQJA 483

//     32T3K is still the only one pair; it doesn't contain any jokers, so its strength doesn't increase.
//     KK677 is now the only two pair, making it the second-weakest hand.
//     T55J5, KTJJT, and QQQJA are now all four of a kind! T55J5 gets rank 3, QQQJA gets rank 4, and KTJJT gets rank 5.

// With the new joker rule, the total winnings in this example are 5905.

// Using the new joker rule, find the rank of every hand in your set. What are the new total winnings?
//
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

use super::card_part2::Card;
use super::hand_part2::Hand;

//const INPUT: &str = "puzzle_inputs/day07/sample.input";
const INPUT: &str = "puzzle_inputs/day07/puzzle.input";

pub fn main() -> Result<(), Error> {
    let input = File::open(INPUT)?;
    let buffered = BufReader::new(input);

    let mut v = Vec::new();
    for line in buffered.lines() {
        let s = line?;
        v.push(s);
    }

    let mut bids: Vec<Bid> = Vec::new();

    for l in v.iter() {
        let xs: Vec<_> = l.split(' ').filter(|s| !s.is_empty()).collect();
        let cards: Option<Vec<_>> = xs[0].chars().map(Card::from_char).collect();
        let cards = cards.unwrap();
        let cards = [cards[0], cards[1], cards[2], cards[3], cards[4]];
        let hand = Hand::new(cards);
        let amount: i32 = xs[1].parse().unwrap();
        let bid = Bid((hand, amount));
        bids.push(bid);
    }

    bids.sort_by(|a, b| a.0 .0.cmp(&b.0 .0));

    let total_bid: i32 = bids
        .iter()
        .enumerate()
        .map(|(i, b)| (i + 1) as i32 * b.0 .1)
        .sum();

    println!("{}", total_bid);
    assert_eq!(total_bid, 251824095); //part2

    Ok(())
}

#[derive(Debug)]
struct Bid((Hand, i32));

impl fmt::Display for Bid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Bid((hand, amout)) = self;
        write!(f, "({}, {})", hand, amout)
    }
}
