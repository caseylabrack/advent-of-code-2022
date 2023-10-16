use std::fs;
// use std::io::Write;
// use std::io::stdout;

fn main() {

    let data = fs::read_to_string("data/input.txt").expect("couldn't read file"); 

    let part1 = solve_part1(data.as_str());
    println!("part 1: {part1}");

    let part2 = solve_part2(data.as_str());
    println!("part 2: \n{part2}");
}

fn solve_part1 (input: &str) -> i32 {
    let mut cycle: i32 = 1;
    let mut x = 1;
    let mut sum_of_signal_strengths = 0;

    input
        .lines()
        .map(Instruction::from_string)
        .for_each(|op| {

            let ticks = match op {
                Instruction::Addx(_z) => 2,
                Instruction::Noop => 1
            };

            for _n in 0..ticks {
                match cycle {
                    20 | 60 | 100 | 140 | 180 | 220 => {
                        println!("During the {cycle}th cycle, register X has the value {x}, so the signal strength is {cycle} times {x} = {}", cycle * x);
                        sum_of_signal_strengths += cycle * x;
                    },
                    _ => ()
                }
                cycle += 1;
            }

            let delta_x = match op {
                Instruction::Addx(z) => z,
                Instruction::Noop => 0
            };

            x += delta_x;

        });

        sum_of_signal_strengths
}

fn solve_part2 (input: &str) -> String {

    let mut message = String::new();
    let mut cycle: i32 = 1;
    let mut x = 2;

    for instruction in input
        .lines()
        .map(Instruction::from_string) 
    {
        let ticks = match instruction {
            Instruction::Addx(_z) => 2,
            Instruction::Noop => 1
        };

        for _n in 0..ticks {

            if cycle == 41 {
                cycle = 1;
                print!("\n");
                message.push_str("\n");
            }
            
            match i32::abs(x - cycle) <= 1 {
                true => message.push_str("#"),
                false => message.push_str(".")
            };

            cycle += 1;
        }

        x += match instruction {
            Instruction::Addx(z) => z,
            Instruction::Noop => 0
        }
    }
    message
}

#[derive(Debug)]
enum Instruction {
    Addx(i32),
    Noop
}

impl Instruction {
    fn from_string(s: &str) -> Instruction {

        let mut m = s.split(" ");
        let op = match m.next() {
            Some(o) => o,
            None => panic!("missing op")
        };

        let num = match m.next() {
            Some(o) => o,
            None => ""
        };

        match op {
            "addx" => Instruction::Addx(num.parse().expect("should be parseable to number, dummy")),
            "noop" => Instruction::Noop,
            _ => panic!("weird op parsed: {}", op)
        }
    }
}