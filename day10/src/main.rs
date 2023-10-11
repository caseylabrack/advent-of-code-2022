use std::fs;

fn main() {

    let mut cycle: i32 = 1;
    let mut x = 1;
    let mut sum_of_signal_strengths = 0;

    let example = fs::read_to_string("data/input.txt").expect("couldn't read file"); 

    example
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

        println!("final sum of signal strengths: {}", sum_of_signal_strengths);

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