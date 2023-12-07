use aoc_helpers::parsing::*;
use aoc_helpers::runner::*;
use std::cmp::Ord;
use std::cmp::Ordering;

fn main() {
    let solution = Solution {};
    run(&solution);
}

struct Solution {}

impl AocSolution for Solution {
    fn year(&self) -> u32 {
        2023
    }
    fn day(&self) -> u32 {
        7
    }

    fn part_one(&self, input: &str) -> String {
        let mut hands: Vec<Hand> = Vec::new();
        for line in input.lines() {
            let hand = parse_hand(line);
            hands.push(hand);
        }
        hands.sort();
        let mut sum = 0u32;
        for i in 0..hands.len() {
            sum += (i as u32 + 1u32) * hands[i].bid;
        }
        sum.to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let mut hands: Vec<Hand> = Vec::new();
        for line in input.lines() {
            let hand = parse_hand_with_jokers(line);
            hands.push(hand);
        }
        hands.sort();
        let mut sum = 0u32;
        for i in 0..hands.len() {
            // println!("{:?}", hands[i]);
            sum += (i as u32 + 1u32) * hands[i].bid;
        }
        sum.to_string()
    }
}

#[derive(Debug)]
struct Hand {
    hand: [u8; 5],
    rank: u8,
    bid: u32,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        // Hands are only equivalent if they're the same cards in the same order.  Otherwise, even
        // if the hand ranks are equal, there's a difference in at least one tiebreaker card.
        self.hand == other.hand
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.rank < other.rank {
            return Ordering::Less;
        }
        if self.rank > other.rank {
            return Ordering::Greater;
        }

        for i in 0..5 {
            if self.hand[i] < other.hand[i] {
                return Ordering::Less;
            }
            if self.hand[i] > other.hand[i] {
                return Ordering::Greater;
            }
        }

        Ordering::Equal
    }
}

fn parse_hand(s: &str) -> Hand {
    let mut hand = [0u8; 5];
    let mut counts = [0u8; 13];
    let mut remainder = s;

    let mut i = 0usize;
    while let Ok((rem, card)) = take_card_char(remainder) {
        remainder = rem;
        match card {
            '2' => {
                counts[0] += 1;
                hand[i] = 1;
            }
            '3' => {
                counts[1] += 1;
                hand[i] = 2;
            }
            '4' => {
                counts[2] += 1;
                hand[i] = 3;
            }
            '5' => {
                counts[3] += 1;
                hand[i] = 4;
            }
            '6' => {
                counts[4] += 1;
                hand[i] = 5;
            }
            '7' => {
                counts[5] += 1;
                hand[i] = 6;
            }
            '8' => {
                counts[6] += 1;
                hand[i] = 7;
            }
            '9' => {
                counts[7] += 1;
                hand[i] = 8;
            }
            'T' => {
                counts[8] += 1;
                hand[i] = 9;
            }
            'J' => {
                counts[9] += 1;
                hand[i] = 10;
            }
            'Q' => {
                counts[10] += 1;
                hand[i] = 11;
            }
            'K' => {
                counts[11] += 1;
                hand[i] = 12;
            }
            'A' => {
                counts[12] += 1;
                hand[i] = 13;
            }
            _ => panic!("Unexpected card {}", card),
        };
        i += 1;
    }

    (remainder, _) = take_spaces(remainder).expect("Missing space after hand");
    let (_, bid) = take_u32(remainder).expect("Missing bid");

    Hand {
        hand,
        rank: rank_hand(&counts),
        bid,
    }
}

fn rank_hand(hand: &[u8; 13]) -> u8 {
    let mut pairs = 0u8;
    let mut triples = 0u8;
    for c in hand.iter() {
        match c {
            2 => pairs += 1,
            3 => triples += 1,
            4 => return 5, // Four of a kind
            5 => return 6, // Five of a kind
            _ => (),
        };
        if pairs == 1 && triples == 1 {
            return 4; // Full house
        }
        if pairs == 2 {
            return 2; // Two pair
        }
    }
    if triples == 1 {
        return 3; // Three of a kind
    }
    if pairs == 1 {
        return 1; // Pair
    }
    return 0; // High card
}

fn parse_hand_with_jokers(s: &str) -> Hand {
    let mut hand = [0u8; 5];
    let mut counts = [0u8; 13];
    let mut remainder = s;

    let mut i = 0usize;
    while let Ok((rem, card)) = take_card_char(remainder) {
        remainder = rem;
        match card {
            'J' => {
                counts[0] += 1;
                hand[i] = 0;
            }
            '2' => {
                counts[1] += 1;
                hand[i] = 1;
            }
            '3' => {
                counts[2] += 1;
                hand[i] = 2;
            }
            '4' => {
                counts[3] += 1;
                hand[i] = 3;
            }
            '5' => {
                counts[4] += 1;
                hand[i] = 4;
            }
            '6' => {
                counts[5] += 1;
                hand[i] = 5;
            }
            '7' => {
                counts[6] += 1;
                hand[i] = 6;
            }
            '8' => {
                counts[7] += 1;
                hand[i] = 7;
            }
            '9' => {
                counts[8] += 1;
                hand[i] = 8;
            }
            'T' => {
                counts[9] += 1;
                hand[i] = 9;
            }
            'Q' => {
                counts[10] += 1;
                hand[i] = 10;
            }
            'K' => {
                counts[11] += 1;
                hand[i] = 11;
            }
            'A' => {
                counts[12] += 1;
                hand[i] = 12;
            }
            _ => panic!("Unexpected card {}", card),
        };
        i += 1;
    }

    (remainder, _) = take_spaces(remainder).expect("Missing space after hand");
    let (_, bid) = take_u32(remainder).expect("Missing bid");

    Hand {
        hand,
        rank: rank_hand_with_jokers(&counts),
        bid,
    }
}

fn rank_hand_with_jokers(hand: &[u8; 13]) -> u8 {
    let five_of_a_kind = 6u8;
    let four_of_a_kind = 5u8;
    let full_house = 4u8;
    let three_of_a_kind = 3u8;
    let two_pair = 2u8;
    let pair = 1u8;
    let high_card = 0u8;

    let mut pairs = 0u8;
    let mut triples = 0u8;
    let jokers = hand[0];
    if jokers == 5 || jokers == 4 {
        return five_of_a_kind;
    }
    for c in hand.iter().skip(1) {
        if c + jokers >= 5 {
            return five_of_a_kind;
        }
        if c + jokers == 4 {
            return four_of_a_kind;
        }
        if *c == 2 {
            pairs += 1;
        }
        if *c == 3 {
            triples += 1;
        }
    }

    // Full house uses all cards.  Can't make one with two jokers (since we'd need a three of a kind
    // with them, which ends up upgrading to a five of a kind).  Similarly can't make one with 2
    // jokers.  So only possible way to do this with jokers is with two pairs, and 1 joker to upgrade
    // one of those pairs to a triple.
    if (pairs == 1 && triples == 1) || (jokers == 1 && pairs == 2) {
        return full_house;
    }

    // If we have a triple, we can't have any jokers since that would've triggered the four of a kind
    // branch.  So we guarantee at this point that having a triple is indeed a three of a kind.
    //
    // With one joker, we need one pair (two pairs was a full house) to make a three of a kind.  With
    // two jokers, we need no pairs (so it upgrades one of the other three cards to a three of a kind).
    if triples == 1 || (jokers == 1 && pairs == 1) || (jokers == 2 && pairs == 0) {
        return three_of_a_kind;
    }

    // If we have two pairs, the fifth card can't be a joker (since that would've been a full house).
    // So this is guaranteed to be a two pair.
    if pairs == 2 {
        return two_pair;
    }

    // At this point, only two ways to make a pair.  Having a pair with no jokers, or having 1 joker with no pairs.
    // Any other combination would result in a higher rank above.
    if pairs == 1 || jokers == 1 {
        return pair;
    }

    // No jokers, no pairs, no triples, no four of a kind, no five of a kind.  That can only be high card.
    return high_card;
}

#[cfg(test)]
mod day1_tests {
    use super::*;

    #[test]
    fn samples_part1() {
        let solution = Solution {};
        assert_eq!(
            solution.part_one(
                "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
            ),
            "6440"
        );
    }

    #[test]
    fn samples_part2() {
        let solution = Solution {};
        assert_eq!(
            solution.part_two(
                "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
            ),
            "5905a"
        );
    }
}
