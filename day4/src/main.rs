use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

static FILE_PATH: &str = "./day4/input";

fn main() -> Result<(), io::Error> {
    let f = File::open(FILE_PATH)?;
    let reader = BufReader::new(f);

    let mut contain = 0;
    let mut overlap = 0;
    
    for l in reader.lines() {
        let line = l?;
        let ranges: Vec<Vec<usize>> = line
            .split(",")
            .map(|s| {
                s.split("-")
                    .map(|s| s.parse().expect("must be a number"))
                    .collect()
            })
            .collect();
        let feb: &Vec<usize> = &ranges[0];
        let seb: &Vec<usize> = &ranges[1];
        if feb[0] <= seb[0] && feb[1] >= seb[1] || feb[0] >= seb[0] && feb[1] <= seb[1] {
            contain += 1;
        }
        if feb[0] <= seb[0] && seb[0] <= feb[1] || seb[0] <= feb[0] && feb[0] <= seb[1] {
            overlap += 1;
        }
    }
    println!("first answer: {}", contain);
    println!("second answer: {}", overlap);

    Ok(())
}
