use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

static FILE_PATH: &str = "./day11/input";

#[derive(Clone)]
struct Monkey {
    items: Vec<usize>,
    test_value: usize,
    true_monkey: usize,
    false_monkey: usize,
    operation: Vec<String>,
    counter: usize,
}

impl Monkey {
    fn new(strings: &[String]) -> Monkey {
        let items = strings[0].split(':').collect::<Vec<&str>>()[1]
            .split(',')
            .map(|n| n.trim().parse::<usize>().expect("must be a number "))
            .collect();
        let operation: Vec<&str> = strings[1].split_whitespace().collect();
        let operation = operation[4..].iter().map(|s| s.to_string()).collect();
        let last_to_usize = |s: &String| -> usize {
            s.split_whitespace()
                .last()
                .expect("exist")
                .parse::<usize>()
                .expect("must be a number")
        };

        let test_value = last_to_usize(&strings[2]);
        let true_monkey = last_to_usize(&strings[3]);
        let false_monkey = last_to_usize(&strings[4]);
        Monkey {
            counter: 0,
            items,
            operation,
            test_value,
            true_monkey,
            false_monkey,
        }
    }

    fn take_item(&mut self, item: usize) {
        self.items.push(item);
    }

    fn run_round(monkeys: &mut [Monkey], index: usize, m: usize) {
        while !monkeys[index].items.is_empty() {
            let item = monkeys[index].items.pop().expect("must exist");
            let func = match monkeys[index].operation[0].as_str() {
                "*" => |a: &usize, b: &usize| -> usize { a * b },
                "+" => |a: &usize, b: &usize| -> usize { a + b },
                _ => panic!("t"),
            };
            let mut new = match monkeys[index].operation[1].parse::<usize>() {
                Ok(m) => func(&item, &m),
                Err(_) => func(&item, &item),
            };
            new = if m == 3 { new / 3 } else { new % m };
            monkeys[index].counter += 1;
            if new % monkeys[index].test_value == 0 {
                let a = monkeys[index].true_monkey;
                monkeys[a].take_item(new);
            } else {
                let a = monkeys[index].false_monkey;
                monkeys[a].take_item(new);
            }
        }
    }
}

fn first(monkeys: Vec<Monkey>) -> usize {
    let mut monkeys = monkeys;
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            Monkey::run_round(&mut monkeys, i, 3);
        }
    }
    monkeys.sort_by(|a, b| b.counter.cmp(&a.counter));
    monkeys[..2].iter().map(|m| m.counter).product()
}

fn second(monkeys: Vec<Monkey>) -> usize {
    let mut monkeys = monkeys;
    let m: usize = monkeys.iter().map(|m| m.test_value).product();
    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            Monkey::run_round(&mut monkeys, i, m);
        }
    }
    monkeys.sort_by(|a, b| b.counter.cmp(&a.counter));
    monkeys[..2].iter().map(|m| m.counter).product()
}

fn main() -> Result<(), io::Error> {
    let f = File::open(FILE_PATH)?;
    let reader = BufReader::new(f);

    let collected_lines: Result<Vec<String>, _> = reader.lines().collect();
    let collected_lines = collected_lines?;
    let mut monkeys = Vec::new();

    for strings in collected_lines.chunks(7) {
        monkeys.push(Monkey::new(&strings[1..6]));
    }
    println!("first: {}", first(monkeys.clone()));
    println!("second: {}", second(monkeys));
    Ok(())
}
