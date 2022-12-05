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

fn proces_comand(
    commands: &[String],
    number_platforms: usize,
    levels: &Vec<Vec<String>>,
    second: bool,
) -> String {
    let mut platforms: Vec<Vec<String>> = vec![Vec::new(); number_platforms];

    for level in levels {
        for (i, b) in level.iter().enumerate() {
            if !b.is_empty() {
                platforms[i].push(b.clone());
            }
        }
    }

    for sc in commands {
        let comand = parse_comand(sc);
        let mut boxes: Vec<String> = Vec::new();
        for _ in 0..comand[0] {
            let e = platforms[comand[1]].pop().expect("platform must be not empty");
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
        answer.push(p.pop().unwrap());
    }
    let answer: Vec<String> = answer
        .iter()
        .map(|s| s[1..s.len() - 1].to_string())
        .collect();
    answer.join("")
}

fn main() -> Result<(), io::Error> {
    let f = File::open(FILE_PATH)?;
    let reader = BufReader::new(f);

    let collected_lines: Result<Vec<String>, _> = reader.lines().collect();
    let collected_lines = collected_lines?;
    let split_position = collected_lines.iter().position(|e| e.is_empty()).unwrap();

    let platforms = &collected_lines[0..split_position - 1];

    let number_platforms: usize = collected_lines[split_position - 1]
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();

    let commands = &collected_lines[split_position + 1..];

    let mut levels: Vec<Vec<String>> = platforms
        .iter()
        .map(|s| s[0..number_platforms * 4 - 1].chars())
        .map(|s| {
            s.collect::<Vec<char>>()
                .chunks(4)
                .map(|x| String::from_iter(x).trim().to_string())
                .collect()
        })
        .collect();

    levels.reverse();

    println!(
        "first answer: {}",
        proces_comand(commands, number_platforms, &levels, false)
    );
    println!(
        "second answer: {}",
        proces_comand(commands, number_platforms, &levels, true)
    );
    Ok(())
}
