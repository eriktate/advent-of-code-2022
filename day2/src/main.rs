enum Outcome {
    Win,
    Draw,
    Lose,
}

impl Outcome {
    fn score(&self) -> u32 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
        }
    }

    fn parse(ch: char) -> Result<Self, String> {
        match ch {
            'X' => Ok(Outcome::Lose),
            'Y' => Ok(Outcome::Draw),
            'Z' => Ok(Outcome::Win),
            _ => Err(String::from("invalid game input")),
        }
    }
}

enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn parse(ch: char) -> Result<Self, String> {
        match ch {
            'A' | 'X' => Ok(Move::Rock),
            'B' | 'Y' => Ok(Move::Paper),
            'C' | 'Z' => Ok(Move::Scissors),
            _ => Err(String::from("invalid game input")),
        }
    }

    fn score(&self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    fn force_outcome(&self, desired_result: &Outcome) -> Move {
        match desired_result {
            Outcome::Win => match self {
                Move::Rock => Move::Paper,
                Move::Paper => Move::Scissors,
                Move::Scissors => Move::Rock,
            },
            Outcome::Draw => match self {
                Move::Rock => Move::Rock,
                Move::Paper => Move::Paper,
                Move::Scissors => Move::Scissors,
            },
            Outcome::Lose => match self {
                Move::Rock => Move::Scissors,
                Move::Paper => Move::Rock,
                Move::Scissors => Move::Paper,
            },
        }
    }
}

fn get_outcome(player_move: &Move, opponent_move: &Move) -> Outcome {
    match player_move {
        Move::Rock => match opponent_move {
            Move::Rock => Outcome::Draw,
            Move::Paper => Outcome::Lose,
            Move::Scissors => Outcome::Win,
        },
        Move::Paper => match opponent_move {
            Move::Rock => Outcome::Win,
            Move::Paper => Outcome::Draw,
            Move::Scissors => Outcome::Lose,
        },
        Move::Scissors => match opponent_move {
            Move::Rock => Outcome::Lose,
            Move::Paper => Outcome::Win,
            Move::Scissors => Outcome::Draw,
        },
    }
}

fn part_one(input: &str) -> u32 {
    let mut total_score = 0;
    for line in input.lines() {
        let mut chars = line.chars();
        let opponent_move = Move::parse(chars.next().unwrap()).expect("invalid game input");
        chars.next();
        let player_move = Move::parse(chars.next().unwrap()).expect("invalid game input");
        total_score += player_move.score() + get_outcome(&player_move, &opponent_move).score();
    }

    total_score
}

fn part_two(input: &str) -> u32 {
    let mut total_score = 0;
    for line in input.lines() {
        let mut chars = line.chars();
        let opponent_move = Move::parse(chars.next().unwrap()).expect("invalid game input");
        chars.next();
        let desired_outcome = Outcome::parse(chars.next().unwrap()).expect("invalid game input");
        total_score +=
            desired_outcome.score() + opponent_move.force_outcome(&desired_outcome).score();
    }

    total_score
}

fn main() {
    let input = include_str!("../input.txt");

    println!("part_one score: {}", part_one(input));
    println!("part_two score: {}", part_two(input));
}
