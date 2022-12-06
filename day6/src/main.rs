use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

static FILE_PATH: &str = "./day6/input";

// this work beter for size less than 256, for bigger cases will be better to use HashSet
fn proces_string(line: &str, size: usize) -> Option<usize> {
    for i in 0..(line.len() - size - 1) {
        let subs = line[i..i + size].as_bytes();
        let mut same = false;
        for x in 0..subs.len() {
            for y in x + 1..subs.len() {
                if subs[x] == subs[y] {
                    same = true;
                    break;
                }
            }
        }
        if !same {
            return Some(i + size);
        }
    }
    None
}

// if you rich guy and have extra 255 bytes 🤑💰
fn proces_string_line(line: &str, size: usize) -> Option<usize> {
    let mut flags = vec![0; 255];
    let mut counter = 0;
    let line = line.as_bytes();
    for i in &line[0..size - 1] {
        flags[*i as usize] += 1;
        if flags[*i as usize] == 1 {
            counter += 1;
        }
    }
    for (index, i) in line[size - 1..].iter().enumerate() {
        flags[*i as usize] += 1;
        if flags[*i as usize] == 1 {
            counter += 1;
        }
        if counter == size {
            return Some(size + index);
        }
        flags[line[index] as usize] -= 1;
        if flags[line[index] as usize] == 0 {
            counter -= 1;
        }
    }
    None
}

fn read_line() -> Result<String, io::Error> {
    let f = File::open(FILE_PATH)?;
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    reader.read_line(&mut line)?;
    Ok(line)
}

fn main() -> Result<(), io::Error> {
    let line = read_line()?;
    println!(
        "first answer {}",
        proces_string_line(&line, 4).expect("must contain no repeats")
    );
    println!(
        "second answer {}",
        proces_string_line(&line, 14).expect("must contain no repeats")
    );
    Ok(())
}

#[test]
fn example() {
    use std::time::Instant;

    let line = read_line().unwrap();
    let start = Instant::now();
    for _ in 0..1 {
        proces_string(&line, 4).expect("must contain no repeats");
        proces_string(&line, 1000).unwrap_or_default();
    }
    println!("{:?}", start.elapsed());
    let start = Instant::now();
    for _ in 0..1 {
        proces_string_line(&line, 4).expect("must contain no repeats");
        proces_string_line(&line, 1000).unwrap_or_default();
    }
    println!("{:?}", start.elapsed());
}
