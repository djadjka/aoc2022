use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

static FILE_PATH: &str = "./day5/input";

fn parse_comand(c: &str) -> [usize; 3] {
    let abc: Vec<&str> = c.split_whitespace().collect();
    let amount = abc[1].parse::<usize>().expect("must be a number");
    let from = abc[3].parse::<usize>().expect("must be a number") - 1;
    let to = abc[5].parse::<usize>().expect("must be a number") - 1;
    [amount, from, to]
}

fn proces_comand(commands: &[String], platforms: &[Vec<u8>], second: bool) -> String {
    let mut platforms = platforms.to_owned();
    for sc in commands {
        let comand = parse_comand(sc);
        let mut boxes: Vec<u8> = Vec::new();
        for _ in 0..comand[0] {
            let e = platforms[comand[1]]
                .pop()
                .expect("platform must be not empty");
            boxes.push(e);
        }

        if second {
            boxes.reverse();
        }

        for b in boxes {
            platforms[comand[2]].push(b);
        }
    }

    let mut answer: Vec<String> = Vec::new();
    for mut p in platforms {
        answer.push((p.pop().expect("must exist") as char).to_string());
    }
    answer.join("")
}

fn main() -> Result<(), io::Error> {
    let f = File::open(FILE_PATH)?;
    let reader = BufReader::new(f);

    let collected_lines: Result<Vec<String>, _> = reader.lines().collect();
    let collected_lines = collected_lines?;
    let split_position = collected_lines
        .iter()
        .position(|e| e.is_empty())
        .expect("must exist");

    let mut levels = collected_lines[0..split_position - 1].to_owned();
    levels.reverse();

    let commands = &collected_lines[split_position + 1..];

    let mut platforms: Vec<Vec<u8>> = vec![Vec::new(); levels[0].len()];

    for level in levels {
        let c_level: Vec<char> = level.chars().collect();
        for i in 0..c_level.len() {
            platforms[i].push(c_level[i] as u8)
        }
    }
    let platforms: Vec<Vec<u8>> = platforms
        .into_iter()
        .filter(|p| {
            p.iter()
                .any(|c| *c != b' ' && *c != b'[' && *c != b']')
        })
        .map(|p| p.into_iter().filter(|&c| c != b' ').collect())
        .collect();

    println!(
        "first answer: {}",
        proces_comand(commands, &platforms, false)
    );
    println!(
        "second answer: {}",
        proces_comand(commands, &platforms, true)
    );
    Ok(())
}
