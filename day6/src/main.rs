use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

static FILE_PATH: &str = "./day6/input";

// this work beter for size less than 256, for bigger cases will be better to use HashSet
fn proces_string(line: &str, size: usize) -> Option<usize> {
    for i in 0..(line.len() - size -1) {
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

fn main() -> Result<(), io::Error> {
    let f = File::open(FILE_PATH)?;
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    reader.read_line(&mut line)?;

    println!("first answer {}", proces_string(&line, 4).expect("must contain no repeats"));
    println!("second answer {}", proces_string(&line, 14).expect("must contain no repeats"));

    Ok(())
}
