use std::{cmp::max, str::FromStr};

use aoc_2015::io::read_input;

#[derive(PartialEq, Debug)]
enum Action {
    TurnOn,
    TurnOff,
    Toggle,
}
#[derive(PartialEq, Debug, Copy, Clone)]
struct Coordinate(u32, u32);

#[derive(PartialEq, Debug)]
struct Range(Coordinate, Coordinate);

#[derive(PartialEq, Debug)]
struct Instruction {
    range: Range,
    action: Action,
}

#[derive(PartialEq, Debug)]
struct ParseError;

impl Into<(u32, u32)> for Coordinate {
    fn into(self) -> (u32, u32) {
        (self.0, self.1)
    }
}

impl Range {
    fn includes(self: &Self, coordinate: Coordinate) -> bool {
        let (x, y) = coordinate.into();
        let (x_start, y_start) = self.0.into();
        let (x_end, y_end) = self.1.into();

        x_start <= x && x <= x_end && y_start <= y && y <= y_end
    }
}

impl Instruction {
    fn execute(self: &Self, coordinate: Coordinate, status: bool) -> bool {
        if self.range.includes(coordinate) {
            match self.action {
                Action::TurnOn => true,
                Action::TurnOff => false,
                Action::Toggle => !status,
            }
        } else {
            status
        }
    }

    fn update_brightness(self: &Self, coordinate: Coordinate, brightness: i32) -> i32 {
        if self.range.includes(coordinate) {
            match self.action {
                Action::TurnOn => brightness + 1,
                Action::TurnOff => max(brightness - 1, 0),
                Action::Toggle => brightness + 2,
            }
        } else {
            brightness
        }
    }
}

impl FromStr for Coordinate {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.splitn(2, ',').collect();

        if parts.len() == 2 {
            parts[0]
                .parse::<u32>()
                .and_then(|a| parts[1].parse::<u32>().and_then(|b| Ok(Coordinate(a, b))))
                .map_err(|_| ParseError)
        } else {
            Err(ParseError)
        }
    }
}

impl FromStr for Range {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.splitn(2, " through ").collect();
        if parts.len() == 2 {
            Coordinate::from_str(parts[0])
                .and_then(|c1| Coordinate::from_str(parts[1]).map(|c2| Range(c1, c2)))
                .map_err(|_| ParseError)
        } else {
            Err(ParseError)
        }
    }
}

impl FromStr for Action {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "turn on" => Ok(Action::TurnOn),
            "turn off" => Ok(Action::TurnOff),
            "toggle" => Ok(Action::Toggle),
            _ => Err(ParseError),
        }
    }
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (action_str, range_str) =
            s.split_at(s.chars().position(|c| c.is_numeric()).unwrap_or(0));

        action_str
            .trim()
            .parse::<Action>()
            .and_then(|action| {
                range_str
                    .parse::<Range>()
                    .map(|range| Instruction { range, action })
            })
            .map_err(|_| ParseError)
    }
}

fn calc_status(coord: Coordinate, instructions: &Vec<Instruction>) -> bool {
    instructions.into_iter().fold(false, |status, instruction| {
        instruction.execute(coord, status)
    })
}

fn calc_brightness(coord: Coordinate, instructions: &Vec<Instruction>) -> i32 {
    instructions
        .into_iter()
        .fold(0i32, |brightness, instruction| {
            instruction.update_brightness(coord, brightness)
        })
}

fn solve_a(data: &Vec<String>) -> usize {
    let instructions: Vec<Instruction> = data.into_iter().filter_map(|s| s.parse().ok()).collect();

    (0..1000)
        .flat_map(|y| (0..1000).map(move |x| Coordinate(x, y)))
        .filter(|c| calc_status(*c, &instructions))
        .count()
}

fn solve_b(data: &Vec<String>) -> i32 {
    let instructions: Vec<Instruction> = data.into_iter().filter_map(|s| s.parse().ok()).collect();

    (0..1000)
        .flat_map(|y| (0..1000).map(move |x| Coordinate(x, y)))
        .map(|coord| calc_brightness(coord, &instructions))
        .sum()
}

fn main() {
    let data = read_input();

    let solution_a = solve_a(&data);
    let solution_b = solve_b(&data);

    println!("{}", solution_a);
    println!("{}", solution_b);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_coordinate() {
        assert_eq!("324,123".parse(), Ok(Coordinate(324, 123)));
    }

    #[test]
    fn parse_range() {
        assert_eq!(
            "995,774 through 997,784".parse(),
            Ok(Range(Coordinate(995, 774), Coordinate(997, 784)))
        );
    }

    #[test]
    fn parse_instruction() {
        assert_eq!(
            "turn off 854,56 through 965,591".parse(),
            Ok(Instruction {
                action: Action::TurnOff,
                range: Range(Coordinate(854, 56), Coordinate(965, 591))
            })
        );
    }

    #[test]
    fn brightness() {
        let instructions = vec![
            "turn on 0,0 through 0,0".parse().unwrap(),
            "toggle 0,0 through 999,999".parse().unwrap(),
        ];
        assert_eq!(
            (0..1000)
                .flat_map(|y| (0..1000).map(move |x| Coordinate(x, y)))
                .map(|coord| calc_brightness(coord, &instructions))
                .sum::<i32>(),
            2_000_001
        );
    }
}
