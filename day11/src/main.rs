use std::fs;

fn main() {
    let data = fs::read_to_string("data/example.txt").expect("couldn't read file");
    let lines: Vec<&str> = data.lines().collect();

    // parse monkeys
    let mut monkeys: Vec<Monkey> = vec![];
    let mut idx = 1;
    while idx < lines.len() {

        let mut items: Vec<i32> = vec![];

        let (_, nums) = lines[idx].split_once(": ").unwrap();

        for n in nums.split(", ") {
            items.push(n.parse().unwrap());
        }

        let is_addition = lines[idx+1].contains("+");

        let (_, last_word) = lines[idx+1].rsplit_once(" ").unwrap();

        let (_, testnum) = lines[idx+2].split_once("  Test: divisible by ").unwrap();
        let testnum: i32 = testnum.parse().unwrap();

        let (_, true_monkey) = lines[idx+3].split_once("    If true: throw to monkey ").unwrap();
        let true_monkey:usize = true_monkey.parse().unwrap();

        let (_, false_monkey) = lines[idx+4].split_once("    If false: throw to monkey ").unwrap();
        let false_monkey:usize = false_monkey.parse().unwrap();

        let op = match last_word {
            "old" => match is_addition {
                true => Op::AddSelf,
                false => Op::MultSelf
            },
            _ => match is_addition {
                true => Op::Add(last_word.parse().expect("if param for `Operation` isn't old, it needs to be an int")),
                false => Op::Mult(last_word.parse().expect("if param for `Operation` isn't old, it needs to be an int"))
            }
        };

        monkeys.push(Monkey { items: items, op: op, test: testnum, true_throw: true_monkey, false_throw: false_monkey, inspect_count: 0 });

        idx += 7;
    }

    let next_monkeys_state = &mut monkeys.clone();

    for i in 0..monkeys.len() {

        println!("Monkey {}:", i);

        let m = monkeys.get(i).unwrap();

        for item in next_monkeys_state.get(i).unwrap().items.clone() {

            println!("  Monkey inspects an item with worry level of {}", item);

            let to_throw = match m.op {
                Op::Add(z) => {println!("    Worry level increases by {} to {}", z, item + z); item + z},
                Op::Mult(z) => {println!("    Worry level is multiplied by {} to {}", z, item * z); item * z},
                Op::AddSelf => {println!("    Worry level is increases by itself to {}", item + item); item + item},
                Op::MultSelf => {println!("    Worry level is multiplied by itself to {}", item * item); item * item}
            };

            let to_throw = to_throw / 3;

            println!("    Monkey gets bored with item. Worry level is divided by 3 to {}.", to_throw);

            match to_throw % m.test == 0 {
                true => {
                    println!("    Current worry level is divisible by {}.", m.test);
                    next_monkeys_state.get_mut(m.true_throw).unwrap().items.push(to_throw);
                    println!("    Item with worry level {} is thrown to monkey {}.", to_throw, m.true_throw);
                },
                false => {
                    println!("    Current worry level is not divisible by {}.", m.test);
                    next_monkeys_state.get_mut(m.false_throw).unwrap().items.push(to_throw);
                    println!("    Item with worry level {} is thrown to monkey {}.", to_throw, m.false_throw);
                }
            }
        }

        next_monkeys_state.get_mut(i).unwrap().items.clear();
    }

    for i in 0..next_monkeys_state.len() {
        println!("Monkey {}: {:?}", i, next_monkeys_state.get(i).unwrap());
    }

}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<i32>,
    op: Op,
    test: i32,
    true_throw: usize,
    false_throw: usize,
    inspect_count: i32
}

#[derive(Debug, Copy, Clone)]
enum Op {
    Add(i32),
    Mult(i32),
    AddSelf,
    MultSelf
}