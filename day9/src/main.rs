use std::collections::HashSet;
use std::process;
use std::{fs, str::FromStr};
use std::ops::Add;

fn main() {
    
    let example = match fs::read_to_string("data/input.txt") {
        Ok(s) => s,
        Err(e) => {
            println!("config error: {:?}", e);
            process::exit(1);
        },
    };  

    let part1 = match solve(example.as_str(), 2) {
        Ok(val) => val,
        Err(e) => {
            println!("application error: {e:?}");
            process::exit(1);
        }
    };
    println!("part 1: {part1:?}");

    let part2 = match solve(example.as_str(), 10) {
        Ok(val) => val,
        Err(e) => {
            println!("application error: {e:?}");
            process::exit(1);
        }
    };
    println!("part 2: {part2:?}");
}

fn solve (input: &str, number_knots: usize) -> Result<usize, &'static str> {
    let mut points_tail_visits: HashSet<Point> = HashSet::new();  
    let mut knots: Vec<Point> = Vec::with_capacity(number_knots);

    for _ in 0..number_knots {
        knots.push(Point {x:0, y: 0});
    }
    for m in input   
        .lines()
        .map(Command::from_str)
    {
        let m = match m {
            Ok(val) => val,
            Err(e) => return Err(e)
        };

        let delta = match m.dir {
            Direction::Left => Point { x: -1, y: 0},
            Direction::Right => Point { x: 1, y: 0 },
            Direction::Up => Point {x: 0, y: 1},
            Direction::Down => Point {x: 0, y: -1}
        };

        // each step
        for _n in 0..m.dist {

            knots[0] = knots[0] + delta;

            // each knot
            for j in 1..number_knots {
            
                let diffx = knots[j-1].x - knots[j].x;
                let diffy = knots[j-1].y - knots[j].y;

                // if the head is ever two units away in either direction, 
                // move tail x and y by its signed difference from head x and y 
                if i32::abs(diffx) == 2 || i32::abs(diffy) == 2 {
                    knots[j].x += diffx.signum();
                    knots[j].y += diffy.signum();    
                }  
            }   
            points_tail_visits.insert(knots[number_knots-1].clone());
        }
    }
    Ok(points_tail_visits.len())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
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

impl TryFrom<char> for Direction {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'R' => Ok(Direction::Right),
            'L' => Ok(Direction::Left),
            'U' => Ok(Direction::Up),
            'D' => Ok(Direction::Down),
            _ => Err("only valid directions are: R, L, U, D")
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Command {
    dir: Direction,
    dist: i32
}

impl FromStr for Command {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" ");
        let (Some(dir), Some(steps), None) = (parts.next(), parts.next(), parts.next()) else {
            println!("got string: {s:?}");
            return Err("needed line format of [direction][space][steps]");
        };

        let dir = match dir.chars().next() {
            Some(char) => char,
            None => return Err("invalid string when parsing Command"),
        };

        let steps = match steps.parse::<i32>()  {
            Ok(s) => s,
            Err(_e) => return Err("couldn't parse number of steps part"),
        };

        Ok(Command { 
            dir: dir.try_into()?, 
            dist: steps,
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_a_direction() {
        let dir: Result<Direction, _> = 'R'.try_into();
        assert_eq!(dir, Ok(Direction::Right));
    }

    #[test]
    fn parse_a_command() {
        let c = Command::from_str("R 4");
        assert_eq!(c, Ok(Command {dir: Direction::Right, dist: 4}));
    }

    #[test]
    fn problem_example_part1 () {

        let example = 
        "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2";

        let s = solve(example, 2);
        assert_eq!(s, Ok(13));
    }

    #[test]
    fn problem_example_part2 () {

        let example = 
        "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20";

        let s = solve(example, 10);
        assert_eq!(s, Ok(36));
    }
}