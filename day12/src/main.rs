use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

use std::collections::VecDeque;

static FILE_PATH: &str = "./day12/input";

static DIR: [(i8, i8); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn run(
    point: (usize, usize),
    queue: &mut VecDeque<(u8, u8)>,
    map: &[Vec<u8>],
    distance: &mut HashMap<(u8, u8), usize>,
    start_point: (u8, u8),
    first_answer: &mut Option<usize>,
    second_answer: &mut Option<usize>,
) {
    for d in DIR {
        let new_x = point.0 as i8 + d.0 as i8;
        let new_y = point.1 as i8 + d.1 as i8;
        if new_x < 0 || new_y < 0 || new_x >= map.len() as i8 || new_y >= map[0].len() as i8 {
            continue;
        }

        let h = distance
            .get(&(point.0 as u8, point.1 as u8))
            .expect("must exist");

        match distance.get(&(new_x as u8, new_y as u8)) {
            Some(_) => continue,
            None => {
                let new_x = new_x as usize;
                let new_y = new_y as usize;
                if map[point.0][point.1] as i8 - map[new_x][new_y] as i8 <= 1 {
                    if map[new_x][new_y] == b'a' && second_answer.is_none() {
                        *second_answer = Some(h + 1);
                    }

                    let new_x = new_x as u8;
                    let new_y = new_y as u8;

                    if (new_x, new_y) == start_point {
                        *first_answer = Some(h + 1);
                    }

                    distance.insert((new_x, new_y), h + 1);
                    queue.push_back((new_x, new_y));
                }
            }
        }
    }
}

fn drow_map(map: &mut Vec<Vec<u8>>) -> Result<((u8, u8), (u8, u8)), io::Error> {
    let f = File::open(FILE_PATH)?;
    let reader = BufReader::new(f);

    let mut start: (u8, u8) = (0, 0);
    let mut end: (u8, u8) = (0, 0);
    for (x, l) in reader.lines().enumerate() {
        let line = l?;
        let processed_line: Vec<u8> = line
            .chars()
            .enumerate()
            .map(|(y, c)| {
                if c == 'S' {
                    start = (x as u8, y as u8);
                    return b'a';
                }
                if c == 'E' {
                    end = (x as u8, y as u8);
                    return b'z';
                }
                c as u8
            })
            .collect();
        map.push(processed_line);
    }
    Ok((start, end))
}

fn main() -> Result<(), io::Error> {
    let mut map: Vec<Vec<u8>> = Vec::new();

    let (start, end) = drow_map(&mut map)?;

    let mut queue: VecDeque<(u8, u8)> = VecDeque::new();

    let mut distance: HashMap<(u8, u8), usize> = HashMap::new();

    let mut first_answer = None;
    let mut second_answer = None;

    queue.push_back(end);

    distance.insert((end.0, end.1), 0);

    while let Some(point) = queue.pop_front() {
        let usize_point = (point.0 as usize, point.1 as usize);

        run(
            usize_point,
            &mut queue,
            &map,
            &mut distance,
            start,
            &mut first_answer,
            &mut second_answer,
        );

        if first_answer.is_some() && second_answer.is_some() {
            break;
        }
    }

    println!("first answer: {}", first_answer.expect("must exist"));
    println!("second answer: {}", second_answer.expect("must exist"));

    Ok(())
}
