use core::fmt;
use std::{
    fs::File,
    io::{self, BufRead},
    path::Path, fmt::Display,
};

enum Outcome {
    Loss,
    Tie,
    Win,
}

enum Choices {
    Paper,
    Rock,
    Scissors,
}

impl Display for Choices {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Choices::Paper => write!(f, "Paper"),
            Choices::Rock => write!(f, "Rock"),
            Choices::Scissors => write!(f, "Scissors"),
        }
    }
}

/*
    A/X = Rock
    B/Y = Paper
    C/Z = Scissors

    Total:
    1p for rock, 2p for paper 3p for scissors,
    +
    0p for loss, 3p for draw, 6p for win
*/

// 2D Array used as a "look-up table" to determine the outcome of the game, in the form of tuples that contains an enum to see if the player won/lost or tied,
// -aswell as the points earned based on the outcome plus the value of either rock, paper or scissors.
const GAME_TABLE: [[(Outcome, i32); 3]; 3] = [
    [(Outcome::Tie, 5), (Outcome::Loss, 1), (Outcome::Win, 9)],
    [(Outcome::Win, 8), (Outcome::Tie, 4), (Outcome::Loss, 3)],
    [(Outcome::Loss, 2), (Outcome::Win, 7), (Outcome::Tie, 6)],
];

fn main() {
    let mut total_score = 0;

    if let Ok(lines) = read_lines("input") {
        for line in lines {
            if let Ok(battle) = line {
                let player_choice = get_choice_by_char(battle.chars().nth(2).unwrap());
                let enemy_choice = get_choice_by_char(battle.chars().nth(0).unwrap());

                println!("Player: '{}' versus Opponent: '{}'", player_choice.to_string(), enemy_choice.to_string());

                let outcome = &GAME_TABLE[enemy_choice as usize][player_choice as usize];

                match outcome.0 {
                    Outcome::Loss => print!("You Lost!"),
                    Outcome::Tie => print!("Tied!"),
                    Outcome::Win => print!("You Win!"),
                }

                total_score += outcome.1;

                println!(" - {} points earned.\n", outcome.1);

            }
        }
    }

    println!("Total scored earned: {}", total_score);
}

fn get_choice_by_char(character: char) -> Choices {
    match character {
        'A' | 'X' => Choices::Rock,
        'B' | 'Y' => Choices::Paper,
        'C' | 'Z' => Choices::Scissors,
        _ => Choices::Paper, // Shouldn't happen, but default to something atleast so the compiler doesn't complain
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
