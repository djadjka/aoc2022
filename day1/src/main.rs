use std::fs::File;
use std::io::Error;
use std::io::BufReader;
use std::io::BufRead;

static FILE_PATH: &str = "./day1/input";

fn main() -> Result<(),Error> {
    let f = File::open(FILE_PATH)?;
    let reader = BufReader::new(f);
    
    let mut cur_elf_calories: usize = 0;
    let mut elfs_calories = Vec::new();

    for line in reader.lines() {
        let l = line?;
        if l.is_empty() {
            elfs_calories.push(cur_elf_calories);
            cur_elf_calories = 0;
        } else {
            cur_elf_calories += l.parse::<usize>().expect("must be a number");
        }
    }
    elfs_calories.sort_by(|a,b| b.cmp(a));
    println!("first answer: {}", elfs_calories[0]);
    println!("second answer: {}", elfs_calories[0..3].iter().sum::<usize>());
    Ok(())
}
