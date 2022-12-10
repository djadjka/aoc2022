use bitvec::prelude::*;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

static FILE_PATH: &str = "./day10/input";

static ANSWER_CYCLES: [i32; 6] = [20, 60, 100, 140, 180, 220];

fn main() -> Result<(), io::Error> {
    let f = File::open(FILE_PATH)?;

    let reader = BufReader::new(f);

    let mut x: i32 = 1;
    let mut cycle = 0;
    let mut answer = 0;

    let mut display = bitvec![0; 240];

    for l in reader.lines() {
        let line = l?;
        let command: Vec<&str> = line.split_whitespace().collect();
        for _ in 0..command.len() {
            cycle += 1;

            if ANSWER_CYCLES.contains(&cycle) {
                answer += cycle * x;
            }

            if (cycle - 1) % 40 >= x - 1 && (cycle - 1) % 40 <= x + 1 {
                display.insert((cycle - 1) as usize, true);
            }
        }

        if command.len() == 2 {
            x += command[1].parse::<i32>().unwrap();
        }
    }

    println!("first answer: {}\n", answer);
    println!("second answer:");
    for row in display.chunks(40) {
        println!(
            "{}",
            String::from_iter(row.iter().map(|b| if *b { '$' } else { ' ' }))
        );
    }
    Ok(())
}
