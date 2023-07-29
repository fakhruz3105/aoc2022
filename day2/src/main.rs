use std::fs;

#[derive(Copy, Clone)]
enum Hand {
    Rock,
    Paper,
    Scissor
}

#[derive(Copy, Clone)]
enum MatchResult {
    Win,
    Draw,
    Lost
}

fn main() {
    let input = fs::read_to_string("index.txt").unwrap();
    let total_score: u32 = input.split("\n").map(|round| {
        let hands: Vec<&str> = round.split_whitespace().collect();
        let opponent_hand = match hands[0] {
            "A" => Some(Hand::Rock),
            "B" => Some(Hand::Paper),
            "C" => Some(Hand::Scissor),
            _ => None
        };

        let match_result = match hands[1] {
            "X" => Some(MatchResult::Lost),
            "Y" => Some(MatchResult::Draw),
            "Z" => Some(MatchResult::Win),
            _ => None
        };

        match (opponent_hand.unwrap(), match_result.unwrap()) {
            // Lost match
            (Hand::Paper, MatchResult::Lost) => 1,
            (Hand::Scissor, MatchResult::Lost) => 2,
            (Hand::Rock, MatchResult::Lost) => 3,
            // Draw match
            (Hand::Rock, MatchResult::Draw) => 4,
            (Hand::Paper, MatchResult::Draw) => 5,
            (Hand::Scissor, MatchResult::Draw) => 6,
            // Win match
            (Hand::Scissor, MatchResult::Win) => 7,
            (Hand::Rock, MatchResult::Win) => 8,
            (Hand::Paper, MatchResult::Win) => 9,
        }
    }).sum();

    println!("{}", total_score);
}
