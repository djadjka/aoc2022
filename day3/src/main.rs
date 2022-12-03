use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

static FILE_PATH: &str = "./day3/input";

fn set_priorities(l: &str, v: &mut Vec<u8>) {
    for c in l.chars() {
        let ascii_code = c as usize;
        if ascii_code >= 97 {
            v[ascii_code - 97] += 1;
        } else {
            v[ascii_code - 39] += 1;
        }
    }
}

fn first(collected_lines: &Vec<String>) {
    let mut sum = 0;
    for l in collected_lines {
        let first_compartments = &l[..l.len() / 2];
        let second_compartments = &l[l.len() / 2..];
        let mut first_priorities: Vec<u8> = vec![0; 52];
        let mut second_priorities: Vec<u8> = vec![0; 52];
        set_priorities(first_compartments, &mut first_priorities);
        set_priorities(second_compartments, &mut second_priorities);
        
        let priority = first_priorities
            .into_iter()
            .zip(second_priorities)
            .position(|a| a.0 >= 1 && a.1 >= 1)
            .expect("must contain the same item");

        sum += priority + 1;
    }
    println!("first answer: {}", sum);
}

fn second(collected_lines: &Vec<String>) {
    let mut sum = 0;
    for backpacks in collected_lines.chunks(3) {
        let mut priorities: Vec<_> = vec![vec![0; 52]; 3];

        for (backpack, mut prioritie) in backpacks.iter().zip(priorities.iter_mut()) {
            set_priorities(backpack, &mut prioritie);
        }

        for i in 0..52 {
            if priorities[0][i] >= 1 && priorities[1][i] >= 1 && priorities[2][i] >= 1 {
                sum += i + 1;
                break;
            }
        }
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
