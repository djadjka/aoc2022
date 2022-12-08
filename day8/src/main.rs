use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

static FILE_PATH: &str = "./day8/input";

fn main() -> Result<(), io::Error> {
    let f = File::open(FILE_PATH)?;
    let reader = BufReader::new(f);
    let map: Vec<Vec<u8>> = reader
        .lines()
        .map(|l| l.unwrap().chars().map(|c| c as u8).collect())
        .collect();

    let mut counter = map[0].len() * 2 + (map.len() - 2) * 2;

    let mut is_visable: Vec<Vec<bool>> = vec![vec![false; map[0].len()]; map.len()];

    let mut set_visable = |y: usize, x: usize, current_max: u8| -> u8 {
        if current_max < map[y][x] {
            if !is_visable[y][x] {
                counter += 1;
            }
            is_visable[y][x] = true;
            return map[y][x];
        }
        current_max
    };

    for i in 1..map[0].len() - 1 {
        let mut current_max_top = map[0][i];
        let mut current_max_down = map[map[0].len() - 1][i];
        let mut current_max_left = map[i][0];
        let mut current_max_right = map[i][map[0].len() - 1];

        for j in 1..map.len() - 1 {
            let reverse_index = map.len() - 1 - j;
            current_max_top = set_visable(j,i,current_max_top);
            current_max_down = set_visable(reverse_index,i,current_max_down);
            current_max_left = set_visable(i,j,current_max_left);
            current_max_right = set_visable(i,reverse_index, current_max_right);
        }
    }

    println!("{}", counter);
    Ok(())
}
