/*** Day 6
 * 
 */


 // define the hand types
 #[derive(Debug, PartialEq)]
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

fn get_hand_type(hand: &str) -> HandType {
    // iterate hand and count the number of each card
    let mut card_counts: Vec<u32> = vec![0; 13];
    for card in hand.chars() {
        match card as u8 {
            b'2' => card_counts[0] += 1,
            b'3' => card_counts[1] += 1,
            b'4' => card_counts[2] += 1,
            b'5' => card_counts[3] += 1,
            b'6' => card_counts[4] += 1,
            b'7' => card_counts[5] += 1,
            b'8' => card_counts[6] += 1,
            b'9' => card_counts[7] += 1,
            b'T' => card_counts[8] += 1,
            b'J' => card_counts[9] += 1,
            b'Q' => card_counts[10] += 1,
            b'K' => card_counts[11] += 1,
            b'A' => card_counts[12] += 1,
            _ => panic!("Invalid card"),
        }
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

fn main() {
    let input  = include_str!("../input.txt");


}


#[cfg(test)]
mod tests {
    use super::*;


    const INPUT: &str =
            "32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483";

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
