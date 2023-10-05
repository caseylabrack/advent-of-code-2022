use std::collections::HashSet;
use std::{fs, str::FromStr};
use std::ops::Add;

fn main() {
    
    let mut head = Point {x:0, y: 0};
    let mut tail = Point{x: 0, y:0};
    let mut points_tail_visits: HashSet<Point> = HashSet::new();

    let example = fs::read_to_string("data/input.txt")
        .expect("missing test input file");

    example
        .lines()
        .map(Command::from_str)
        .for_each(|x| {
            
            let c = x.unwrap();

            // println!("MOVE: {c:?}");

            // where does head move per step
            let delta = match c.dir {
                Direction::Left => Point { x: -1, y: 0},
                Direction::Right => Point { x: 1, y: 0 },
                Direction::Up => Point {x: 0, y: 1},
                Direction::Down => Point {x: 0, y: -1}
            };

            // move head and tail
            for _n in 0..c.dist {

                head = head + delta;

                let diffx = head.x - tail.x;
                let diffy = head.y - tail.y;
                if i32::abs(diffx) == 2 || i32::abs(diffy) == 2 {
                    tail.x += sign(head.x - tail.x);
                    tail.y += sign(head.y - tail.y);    
                }  

                points_tail_visits.insert(tail);

                // println!("head is at {:?}", head);
                // println!("tail is at {:?}", tail);
            }
            
        });

    // println!("head, {:?}", head);
    // println!("the example input: {}", example);

    println!("num points: {}", points_tail_visits.len());
    // assert_eq!(Command::from_str("R 4").unwrap(), Command::Right(4));
}

fn sign (p:i32) -> i32 {
    if p > 0 {
        return 1;
    } else if p < 0 {
        return -1;
    } else {
        return 0;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        // todo!()
        Point{
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Direction {
    Right,
    Left,
    Up,
    Down
}

#[derive(Debug, Clone, PartialEq)]
struct Command {
    dir: Direction,
    dist: i32
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // todo!()
        let (dir, d) = s.split_at(2);
        let dir = match dir.chars().next() {
            Some('R') => Direction::Right,
            Some('L') => Direction::Left,
            Some('U') => Direction::Up,
            Some('D') => Direction::Down,
            _ => panic!("direction needs to be one of these: RLUD"),
        };
        let d: i32 = d.parse().expect("couldn't parse steps");
        Ok(Command { dir: dir, dist: d })
    }
}