use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

static FILE_PATH: &str = "./day9/input";

fn move_tail(head: (i32, i32), tail: &mut (i32, i32)) {
    let dif_x: i32 = head.0 - tail.0;
    let dif_y: i32 = head.1 - tail.1;

    if dif_x.abs() <= 1 && dif_y.abs() <= 1 {
        return;
    }

    tail.0 += dif_x.signum();
    tail.1 += dif_y.signum();
}

fn execute_command(
    times: i32,
    dir: (i32, i32),
    knots: &mut Vec<(i32, i32)>,
    answer_first: &mut HashSet<(i32, i32)>,
    answer_second: &mut HashSet<(i32, i32)>,
) {
    for _ in 0..times {
        (knots[0]).0 += dir.0;
        (knots[0]).1 += dir.1;

        for i in 1..knots.len() {
            move_tail(knots[i - 1], &mut knots[i]);
        }

        answer_first.insert(knots[1]);
        answer_second.insert(knots[9]);
    }
}

fn main() -> Result<(), io::Error> {
    let f = File::open(FILE_PATH)?;
    let reader = BufReader::new(f);
    let mut knots = vec![(0, 0); 10];
    let mut answer_first: HashSet<(i32, i32)> = HashSet::new();
    let mut answer_second: HashSet<(i32, i32)> = HashSet::new();
    answer_first.insert((0, 0));
    answer_second.insert((0, 0));
    for l in reader.lines() {
        let line = l?;
        let command: Vec<&str> = line.split_whitespace().collect();
        let times: i32 = command[1].parse().expect("must be a number");
        match command[0] {
            "U" => {
                execute_command(times, (0, 1), &mut knots, &mut answer_first, &mut answer_second);
            }
            "D" => {
                execute_command(times, (0, -1), &mut knots, &mut answer_first,&mut answer_second);
            }
            "L" => {
                execute_command(times, (-1, 0), &mut knots, &mut answer_first, &mut answer_second);
            }
            "R" => {
                execute_command(times, (1, 0), &mut knots, &mut answer_first, &mut answer_second);
            }
            _ => panic!("incorect direction"),
        }
    }
    println!("first: {:?}", answer_first.len());
    println!("second: {:?}", answer_second.len());
    Ok(())
}
