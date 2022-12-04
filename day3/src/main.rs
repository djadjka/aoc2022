use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

static FILE_PATH: &str = "./day3/input";

fn set_priorities(l: &str) -> u64 {
    let mut p: u64 = 0;
    for c in l.chars() {
        let ascii_code = c as usize;
        if ascii_code >= 97 {
            p |= 1<<(ascii_code - 97);
        } else {
            p |= 1<<(ascii_code - 39);
        }
    }
    p
}

fn first(collected_lines: &Vec<String>) {
    let mut sum = 0;
    for l in collected_lines {
        let first_compartments = &l[..l.len() / 2];
        let second_compartments = &l[l.len() / 2..];

        let first_priorities: u64 = set_priorities(first_compartments);
        let second_priorities: u64 = set_priorities(second_compartments);

        let priority = (first_priorities & second_priorities).trailing_zeros();
        sum += priority;
    }
    println!("first answer: {}", sum);
}

fn second(collected_lines: &Vec<String>) {
    let mut sum = 0;
    for backpacks in collected_lines.chunks(3) {
        let priorities: Vec<u64> = backpacks.iter().map(|s| set_priorities(&s)).collect();
        let priority = (priorities[0] & priorities[1] & priorities[2]).trailing_zeros();
        sum += priority;
    }
    println!("second answer: {}", sum);
}

fn main() -> Result<(), io::Error> {
    let f = File::open(FILE_PATH)?;
    let reader = BufReader::new(f);
    let collected_lines: Result<Vec<String>, _> = reader.lines().collect();

    let collected_lines = collected_lines?;

    first(&collected_lines);
    second(&collected_lines);
    Ok(())
}
