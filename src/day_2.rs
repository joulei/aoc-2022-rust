const MY_ROCK: &str = "X";
const MY_PAPER: &str = "Y";
const MY_SCISSORS: &str = "Z";

const THEIR_ROCK: &str = "A";
const THEIR_PAPER: &str = "B";
const THEIR_SCISSORS: &str = "C";

const ROCK_POINTS: i32 = 1;
const PAPER_POINTS: i32 = 2;
const SCISSORS_POINTS: i32 = 3;

const WIN_SCORE: i32 = 6;
const DRAW_SCORE: i32 = 3;
const LOSE_SCORE: i32 = 0;

pub fn solution() {
    let score = std::fs::read_to_string("input/day_2.txt")
        .expect("Input file not found")
        .split("\n")
        .fold(0, |acc, item| acc + get_round_score(item));

    // Part one:
    println!("Total score: {}", score);
}

fn get_round_score(round: &str) -> i32 {
    if let [opponent_play, my_play] = round.split_whitespace().collect::<Vec<&str>>()[..] {
        match my_play {
            MY_ROCK => {
                return calculate_rock(opponent_play);
            }
            MY_PAPER => {
                return calculate_paper(opponent_play);
            }
            MY_SCISSORS => {
                return calculate_scissors(opponent_play);
            }
            _ => todo!(),
        }
    }
    todo!()
}

fn calculate_rock(opponent_play: &str) -> i32 {
    let score = match opponent_play {
        THEIR_ROCK => DRAW_SCORE,
        THEIR_PAPER => LOSE_SCORE,
        THEIR_SCISSORS => WIN_SCORE,
        _ => todo!(),
    };
    ROCK_POINTS + score
}

fn calculate_paper(opponent_play: &str) -> i32 {
    let score = match opponent_play {
        THEIR_ROCK => WIN_SCORE,
        THEIR_PAPER => DRAW_SCORE,
        THEIR_SCISSORS => LOSE_SCORE,
        _ => todo!(),
    };
    PAPER_POINTS + score
}

fn calculate_scissors(opponent_play: &str) -> i32 {
    let score = match opponent_play {
        THEIR_ROCK => LOSE_SCORE,
        THEIR_PAPER => WIN_SCORE,
        THEIR_SCISSORS => DRAW_SCORE,
        _ => todo!(),
    };
    SCISSORS_POINTS + score
}
