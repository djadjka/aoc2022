use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

static FILE_PATH: &str = "./day2/input";

fn run(is_second: bool) -> Result<usize, io::Error> {
    let f = File::open(FILE_PATH)?;
    let reader = BufReader::new(f);
    let mut second_player_score = 0;
    for line in reader.lines() {
        let line = line?;

        let choices: Vec<&str> = line.split(' ').collect();
        let first_player_choice =
            choices[0].chars().next().expect("must contain symbol") as i8 - 65;
        let outcome = choices[1].chars().next().expect("must contain symbol") as i8 - 88;

        let second_player_choice = if is_second {
            (first_player_choice + outcome - 1).rem_euclid(3)
        } else {
            outcome
        };

        second_player_score += second_player_choice as usize + 1;

        let diff = first_player_choice - second_player_choice;

        if diff.rem_euclid(3) == 2 {
            second_player_score += 6
        } else if diff == 0 {
            second_player_score += 3;
        }
    }
    Ok(second_player_score)
}

fn main() {
    run(false)
        .map(|s| println!("first answer: {}", s))
        .expect("should open file");
    run(true)
        .map(|s| println!("second answer: {}", s))
        .expect("should open file");
}
