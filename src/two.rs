use std::{fs::File, io::Read};

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
enum Action {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
enum GameResult {
    Win = 6,
    Tie = 3,
    Loss = 0,
}

// return whether the left hand side wins
fn left_wins(lhs: Action, rhs: Action) -> GameResult {
    use Action::*;
    use GameResult::*;
    match lhs {
        Rock => match rhs {
            Rock => Tie,
            Paper => Loss,
            Scissors => Win,
        },
        Paper => match rhs {
            Rock => Win,
            Paper => Tie,
            Scissors => Loss,
        },
        Scissors => match rhs {
            Rock => Loss,
            Paper => Win,
            Scissors => Tie,
        },
    }
}

pub fn first(mut file: File) -> color_eyre::Result<u32> {
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    let mut my_score = 0;
    let mut en_score = 0;
    for game in buf.split("\n") {
        let chars: Vec<char> = game.chars().collect();
        let my_action = match chars[2] {
            'X' => Action::Rock,
            'Y' => Action::Paper,
            'Z' => Action::Scissors,
            _ => unreachable!(),
        };

        let en_action = match chars[0] {
            'A' => Action::Rock,
            'B' => Action::Paper,
            'C' => Action::Scissors,
            _ => unreachable!(),
        };

        my_score += my_action as u32;
        my_score += left_wins(my_action, en_action) as u32;

        en_score += en_action as u32;
        en_score += left_wins(en_action, my_action) as u32;
    }

    Ok(my_score)
}

pub fn second(mut file: File) -> color_eyre::Result<u32> {
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    let mut my_score = 0;
    let mut en_score = 0;

    for game in buf.split("\n") {
        let chars: Vec<char> = game.chars().collect();
        let my_result = match chars[2] {
            'X' => GameResult::Loss,
            'Y' => GameResult::Tie,
            'Z' => GameResult::Win,
            _ => unreachable!(),
        };

        let en_action = match chars[0] {
            'A' => Action::Rock,
            'B' => Action::Paper,
            'C' => Action::Scissors,
            _ => unreachable!(),
        };

        let my_action = match (en_action, my_result) {
            (Action::Scissors, GameResult::Tie)
            | (Action::Paper, GameResult::Win)
            | (Action::Rock, GameResult::Loss) => Action::Scissors,

            (Action::Rock, GameResult::Win)
            | (Action::Paper, GameResult::Tie)
            | (Action::Scissors, GameResult::Loss) => Action::Paper,

            (Action::Rock, GameResult::Tie)
            | (Action::Paper, GameResult::Loss)
            | (Action::Scissors, GameResult::Win) => Action::Rock,
        };

        my_score += my_action as u32;
        my_score += my_result as u32;
    }

    Ok(my_score)
}
