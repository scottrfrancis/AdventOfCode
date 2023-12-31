/*** Day 6
 * 
 */

 use core::{cmp::Ordering};
use std::f32::consts::E;

 // define the hand types
 #[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
 enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    // Straight,
    // Flush,
    FullHouse,
    FourOfAKind,
    // StraightFlush,
    // RoyalFlush,
    FiveOfAKind,
}

fn get_card_rank(card: u8) -> u8 {
    match card {
        b'2' => 0,
        b'3' => 1,
        b'4' => 2,
        b'5' => 3,
        b'6' => 4,
        b'7' => 5,
        b'8' => 6,
        b'9' => 7,
        b'T' => 8,
        b'J' => 9,
        b'Q' => 10,
        b'K' => 11,
        b'A' => 12,
        _ => {
            println!("Invalid card: {}", card as char);
            panic!("Invalid card")
        },
    }
}

fn get_hand_type(hand: &str) -> HandType {
    // iterate hand and count the number of each card
    let mut card_counts: Vec<u32> = vec![0; 13];
    for card in hand.chars() {
        card_counts[get_card_rank(card as u8) as usize] += 1;
    }

    // check the hand type
    let fives: Vec<bool> = card_counts.iter().map(|&x| x == 5).collect();
    let num_fives = fives.iter().filter(|&x| *x).count();
    if num_fives >= 1 {
        return HandType::FiveOfAKind;
    }

    let fours: Vec<bool> = card_counts.iter().map(|&x| x == 4).collect();
    let num_fours = fours.iter().filter(|&x| *x).count();
    if num_fours >= 1 {
        return HandType::FourOfAKind;
    }

    let threes: Vec<bool> = card_counts.iter().map(|&x| x == 3).collect();
    let num_threes = threes.iter().filter(|&x| *x).count();

    let twos: Vec<bool> = card_counts.iter().map(|&x| x == 2).collect();
    let num_twos = twos.iter().filter(|&x| *x).count();

    if num_threes >= 1 {
        if  num_twos >= 1 {
            return HandType::FullHouse;
        }
        return HandType::ThreeOfAKind;
    }

    if num_twos == 2 {
        return HandType::TwoPair;
    }

    if num_twos == 1 {
        return HandType::OnePair;
    }

    HandType::HighCard
}

fn get_hand_type_wild(hand: &str) -> HandType {
    // iterate hand and count the number of each card
    let mut card_counts: Vec<u32> = vec![0; 13];
    for card in hand.chars() {
        card_counts[get_card_rank(card as u8) as usize] += 1;
    }
    // zero out the Wild cards -- but save for later
    let mut num_wild = card_counts[9] as usize;      // 'J' is 9
    card_counts[9] = 0;
    if num_wild >= 5 {
        println!("SURPRISE!! 5 wilds: {}", hand);
        return HandType::FiveOfAKind;
    }

    // check the hand type
    let fives: Vec<bool> = card_counts.iter().map(|&x| x == 5).collect();
    let num_fives = fives.iter().filter(|&x| *x).count();
    if num_fives >= 1 {
        return HandType::FiveOfAKind;
    }

    let fours: Vec<bool> = card_counts.iter().map(|&x| x == 4).collect();
    let num_fours = fours.iter().filter(|&x| *x).count();
    if num_fours >= 1 {
        if num_wild >= 1 {
            return HandType::FiveOfAKind;
        }
        return HandType::FourOfAKind;
    }

    let threes: Vec<bool> = card_counts.iter().map(|&x| x == 3).collect();
    let num_threes = threes.iter().filter(|&x| *x).count();

    let twos: Vec<bool> = card_counts.iter().map(|&x| x == 2).collect();
    let num_twos = twos.iter().filter(|&x| *x).count();

    if num_threes >= 1 {
        let num_threes_and_wild = num_threes + num_wild;
        match num_threes_and_wild {
            3..=5 => return HandType::FiveOfAKind,
            2 => return HandType::FourOfAKind,
            1 => {
                if num_twos >= 1 {
                    return HandType::FullHouse;
                }
            }
            _ => (),
        }

        return HandType::ThreeOfAKind;
    }

    if num_twos == 2 {
        if num_wild >= 1 {
            return HandType::FullHouse;
        }
        return HandType::TwoPair;
    }

    if num_twos == 1 {
        let mut num_twos_and_wild = num_twos + num_wild;
        match num_twos_and_wild {
            4..=5 => return HandType::FiveOfAKind,
            3 => return HandType::FourOfAKind,
            2 => return HandType::ThreeOfAKind,
            1 => return HandType::OnePair,
            _ => (),
        }
    }

    match num_wild {
        4..=5 => return HandType::FiveOfAKind,
        3 => return HandType::FourOfAKind,
        2 => return HandType::ThreeOfAKind,
        1 => return HandType::OnePair,
        0 => return HandType::HighCard,
        _ => return HandType::HighCard,
    }
}

// returns true if hand1 is better (>) than hand2
fn hand_is_better(hand1: &str, hand2: &str) -> bool {
    let type1 = get_hand_type(hand1);
    let type2 = get_hand_type(hand2);

    match type1.cmp(&type2) {
        Ordering::Greater => return true,
        Ordering::Less => return false,
        Ordering::Equal => (),
    }

    assert!(type1 == type2);
    // compare letter by letter
    let mut chars2 = hand2.chars();
    for c1 in hand1.chars() {
        let Some(c2) = chars2.next() else {
            return true;
        };
        
        let rank1 = get_card_rank(c1 as u8);
        let rank2 = get_card_rank(c2 as u8);

        match rank1.cmp(&rank2) {
            Ordering::Greater => return true,
            Ordering::Less => return false,
            Ordering::Equal => continue,
        }
    }

    false
}

fn hand_is_better_wild(hand1: &str, hand2: &str) -> bool {
    let type1 = get_hand_type_wild(hand1);
    let type2 = get_hand_type_wild(hand2);

    match type1.cmp(&type2) {
        Ordering::Greater => return true,
        Ordering::Less => return false,
        Ordering::Equal => (),
    }

    assert!(type1 == type2);
    // compare letter by letter
    let mut chars2 = hand2.chars();
    for c1 in hand1.chars() {
        let Some(c2) = chars2.next() else {
            return true;
        };
        
        let rank1: i8 = if c1 != 'J' { get_card_rank(c1 as u8) as i8 } else { -1 };
        let rank2: i8 = if c2 != 'J' { get_card_rank(c2 as u8) as i8 } else { -1 };

        match rank1.cmp(&rank2) {
            Ordering::Greater => return true,
            Ordering::Less => return false,
            Ordering::Equal => continue,
        }
    }

    false
}

fn total_winnings_wild(input: &str) -> u32 {
    let mut lines: Vec<&str> = input.lines()
        .map(|l| l.trim()).collect();
    lines.sort_by(|&a, &b| {
        let mut a = a.split_whitespace();
        let mut b = b.split_whitespace();
        let hand1 = a.next().unwrap();
        let hand2 = b.next().unwrap();
        match hand_is_better_wild(hand1, hand2) {
            true => Ordering::Greater,
            false => Ordering::Less,
        }
    });

    let mut total = 0;
    for (i, line) in lines.iter().enumerate() {
        let mut line = line.split_whitespace();
        let hand = line.next().unwrap();
        let bet = line.next().unwrap().parse::<u32>().unwrap();
        total += (i as u32 + 1)*bet;
    }

    total
}

fn total_winnings(input: &str) -> u32 {
    let mut lines: Vec<&str> = input.lines()
        .map(|l| l.trim()).collect();
    lines.sort_by(|&a, &b| {
        let mut a = a.split_whitespace();
        let mut b = b.split_whitespace();
        let hand1 = a.next().unwrap();
        let hand2 = b.next().unwrap();
        match hand_is_better(hand1, hand2) {
            true => Ordering::Greater,
            false => Ordering::Less,
        }
    });

    let mut total = 0;
    for (i, line) in lines.iter().enumerate() {
        let mut line = line.split_whitespace();
        let hand = line.next().unwrap();
        let bet = line.next().unwrap().parse::<u32>().unwrap();
        total += (i as u32 + 1)*bet;
    }

    total
}

fn main() {
    println!("Part 1");
    let input  = include_str!("../input.txt");
    let total = total_winnings(input);
    println!("Total winnings: {}", total);

    println!("Part 2");
    let total = total_winnings_wild(input);
    println!("Total winnings: {}", total);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input  = include_str!("../input.txt");
        let total = total_winnings_wild(input);
        assert_eq!(total, 248652697);
    }

    #[test]
    fn test_part1() {
        let input  = include_str!("../input.txt");
        let total = total_winnings(input);
        assert_eq!(total, 250453939);
    }

    const INPUT: &str =
            "32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483";
        
    #[test]
    fn test_total_winnings() {
        let total = total_winnings(INPUT);
        assert_eq!(total, 6440);
    }

    #[test]
    fn test_ordering_wild() {
        let mut lines: Vec<&str> = INPUT.lines()
            .map(|l| l.trim()).collect();
        lines.sort_by(|&a, &b| {
            let mut a = a.split_whitespace();
            let mut b = b.split_whitespace();
            let hand1 = a.next().unwrap();
            let hand2 = b.next().unwrap();
            match hand_is_better_wild(hand1, hand2) {
                true => Ordering::Greater,
                false => Ordering::Less,
            }
        });

        let expected = vec!["32T3K 765", "KK677 28", "T55J5 684", "QQQJA 483", "KTJJT 220", ];
        assert_eq!(lines, expected);        
    }

    #[test]
    fn test_ordering() {
        let mut lines: Vec<&str> = INPUT.lines()
            .map(|l| l.trim()).collect();
        lines.sort_by(|&a, &b| {
            let mut a = a.split_whitespace();
            let mut b = b.split_whitespace();
            let hand1 = a.next().unwrap();
            let hand2 = b.next().unwrap();
            match hand_is_better(hand1, hand2) {
                true => Ordering::Greater,
                false => Ordering::Less,
            }
        });

        let expected = vec!["32T3K 765", "KTJJT 220", "KK677 28", "T55J5 684", "QQQJA 483"];
        assert_eq!(lines, expected);
    }

    #[test]
    fn test_rank_hands_wild() {
        let hand1 = "32T3K";
        let hand2 = "T55J5";
        
        assert!(!hand_is_better_wild(hand1, hand2));
        assert!( hand_is_better_wild(hand2, hand1));
    
        assert!(!hand_is_better_wild("T55J5", "QQQJA"));
    }

    #[test]
    fn test_rank_hands() {
        let hand1 = "KK677";
        let hand2 = "KTJJT";
        
        assert!(hand_is_better(hand1, hand2));
        assert!(!hand_is_better(hand2, hand1));
    }

    #[test]
    fn test_hand_types_wild() {
        assert_eq!(get_hand_type_wild("32T3K"), HandType::OnePair);
        assert_eq!(get_hand_type_wild("T55J5"), HandType::FourOfAKind);
        assert_eq!(get_hand_type_wild("KK677"), HandType::TwoPair);
        assert_eq!(get_hand_type_wild("KTJJT"), HandType::FourOfAKind);
        assert_eq!(get_hand_type_wild("QQQJA"), HandType::FourOfAKind);
    
        assert_eq!(get_hand_type_wild("43T2K"), HandType::HighCard);
        assert_eq!(get_hand_type_wild("J3T2K"), HandType::OnePair);
        assert_eq!(get_hand_type_wild("32J3K"), HandType::ThreeOfAKind);
        assert_eq!(get_hand_type_wild("KKJ77"), HandType::FullHouse);
        assert_eq!(get_hand_type_wild("QQQTA"), HandType::ThreeOfAKind);
        assert_eq!(get_hand_type_wild("33T33"), HandType::FourOfAKind);
        assert_eq!(get_hand_type_wild("33J33"), HandType::FiveOfAKind);

        assert_eq!(get_hand_type_wild("JJJJJ"), HandType::FiveOfAKind);
    }

    #[test]
    fn test_hand_types() {
        let hand = "32T3K";
        let hand_type = get_hand_type(hand);
        assert_eq!(hand_type, HandType::OnePair);

        assert_eq!(get_hand_type("T55J5"), HandType::ThreeOfAKind);
        assert_eq!(get_hand_type("KK677"), HandType::TwoPair);
        assert_eq!(get_hand_type("KTJJT"), HandType::TwoPair);
        assert_eq!(get_hand_type("QQQJA"), HandType::ThreeOfAKind);

        assert_eq!(get_hand_type("33T33"), HandType::FourOfAKind);
        assert_eq!(get_hand_type("33TT3"), HandType::FullHouse);
        assert_eq!(get_hand_type("33333"), HandType::FiveOfAKind);
        assert_eq!(get_hand_type("43T2K"), HandType::HighCard);
    }


}
