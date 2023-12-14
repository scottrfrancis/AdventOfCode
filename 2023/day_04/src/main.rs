/*** Day 4
 * 
 * Questions/Concerns/things to check/assert for / assumptions:
 * 
 * are the 'numbers you have' unique for a card?  do duplicate numbers score points?
 * 
 * 
 * 
 */

 use std::collections::HashSet;

 fn count_winning_numbers(winning_numbers: Vec<&str>, drawn_numbers:Vec<&str>) -> u32 {
        // make winning_numbers a set and drawn_numbers a set
        let winners: HashSet<&str> = winning_numbers.into_iter().collect();
        let drawn: HashSet<&str> = drawn_numbers.into_iter().collect();
        // take intersection of the two sets
        let drawn_winners: HashSet<_> = winners.intersection(&drawn).collect();
        // println!("{} Winners: {:?}", fields[0], drawn_winners);

        let num_winners = drawn_winners.len() as u32;
        num_winners
 }

 fn score_cards(input: &str) -> u32 {
    let mut score = 0;

    for line in input.lines() {
        let card = line.trim();
        let fields: Vec<&str> = card.split([':', '|']).collect();
        // let card_id = fields[0].split_whitespace().collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
        let winning_numbers = fields[1].split_whitespace().collect::<Vec<&str>>();
        let drawn_numbers = fields[2].split_whitespace().collect::<Vec<&str>>();

        // score the card
        let num_drawn_winners = count_winning_numbers(winning_numbers, drawn_numbers);
        let card_score = if num_drawn_winners > 0 { 2u32.pow(num_drawn_winners - 1) } else { 0 };
        score += card_score;
    }

    score
}

fn play_count_cards(input: &str) -> u32 {
    let mut card_counts: Vec<u32> = [0].to_vec();

    for line in input.lines() {
        let card = line.trim();
        let fields: Vec<&str> = card.split([':', '|']).collect();
        let card_id = fields[0].split_whitespace().collect::<Vec<&str>>()[1].parse::<u32>().unwrap() as usize - 1;
        if card_id >= card_counts.len() {
            card_counts.resize(card_id + 1, 0);
        }
        card_counts[card_id as usize] += 1;

        let winning_numbers = fields[1].split_whitespace().collect::<Vec<&str>>();
        let drawn_numbers = fields[2].split_whitespace().collect::<Vec<&str>>();

        // score the card
        let num_drawn_winners = count_winning_numbers(winning_numbers, drawn_numbers);
        // increment the card count for the next `num_drawn_winners` cards
        // scaled by the number of cards with the current id
        let cards_in_play = card_counts[card_id as usize];
        if num_drawn_winners > 0 { 
            let next_card_id = (card_id + 1) as usize;
            let last_card_id = next_card_id + 1 + num_drawn_winners as usize - 1 - 1;
            if last_card_id >= card_counts.len() {
                card_counts.resize(last_card_id + 1, 0);
            }
            card_counts[next_card_id..=last_card_id].iter_mut().for_each(|x| *x += cards_in_play);    
        }
    }

    let sum_cards = card_counts.iter().sum();
    sum_cards
}

fn main() {
    let input  = include_str!("../input.txt");

    let sum = score_cards(input);
    println!("\nSum of all card scores: {}", sum);  

    // part 2
    let sum_cards = play_count_cards(input);
    println!("\nTotal Card Count: {}", sum_cards); 
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_cards() {
        let input =
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(score_cards(input), 13);
    }

    #[test]
    fn test_part1() {
        let input  = include_str!("../input.txt");
        assert_eq!(score_cards(input), 25571);
    }

    #[test]
    fn test_play_count_cards() {
        let input =
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(play_count_cards(input), 30);
    
    }
}
