use std::str::FromStr;

use aoc_2015::io::read_input;

#[derive(Debug, PartialEq, Default)]
struct Present {
    l: u32,
    w: u32,
    h: u32,
}

#[derive(Debug, PartialEq)]
struct ParsePresentError;

impl Present {
    fn smallest_sides(self: &Self) -> (u32, u32) {
        let mut sides = [self.l, self.w, self.h];
        sides.sort();
        (sides[0], sides[1])
    }

    fn surface(self: &Self) -> u32 {
        let (small_1, small_2) = self.smallest_sides();

        2 * self.l * self.w + 2 * self.w * self.h + 2 * self.l * self.h + small_1 * small_2
    }

    fn ribbon(self: &Self) -> u32 {
        let (small_1, small_2) = self.smallest_sides();

        2 * small_1 + 2 * small_2 + self.l * self.w * self.h
    }
}

impl FromStr for Present {
    type Err = ParsePresentError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r: Vec<u32> = s
            .splitn(3, 'x')
            .map(|s| s.parse::<u32>())
            .map(|r| r.unwrap_or(0))
            .collect();

        if r.len() == 3 {
            Ok(Present {
                l: r[0],
                w: r[1],
                h: r[2],
            })
        } else {
            Err(ParsePresentError)
        }
    }
}

fn solve_a(data: &Vec<String>) -> u32 {
    data.into_iter()
        .map(|s| s.parse::<Present>())
        .map(|r| r.unwrap_or_default())
        .map(|present| present.surface())
        .sum()
}

fn solve_b(data: &Vec<String>) -> u32 {
    data.into_iter()
        .map(|s| s.parse::<Present>())
        .map(|r| r.unwrap_or_default())
        .map(|present| present.ribbon())
        .sum()
}

fn main() {
    let lines = read_input();

    let solution_a = solve_a(&lines);
    let solution_b = solve_b(&lines);

    println!("{}", solution_a);
    println!("{}", solution_b);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser() {
        assert_eq!("3x4x5".parse(), Ok(Present { l: 3, w: 4, h: 5 }))
    }

    #[test]
    fn surface() {
        assert_eq!(Present { l: 3, w: 4, h: 2 }.surface(), 58);
        assert_eq!(Present { l: 1, w: 1, h: 10 }.surface(), 43);
    }

    #[test]
    fn ribbon() {
        assert_eq!(Present { l: 3, w: 4, h: 2 }.ribbon(), 34);
        assert_eq!(Present { l: 1, w: 1, h: 10 }.ribbon(), 14);
    }
}
