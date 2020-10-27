use aoc_2015::io::read_input;
use std::collections::HashSet;
use std::iter::once;

#[derive(Default, Debug, Hash, PartialEq, Eq, Copy, Clone)]
struct Position(i32, i32);

impl Position {
    fn parse_instruction(self: &Self, instruction: char) -> Self {
        match instruction {
            '^' => Self(self.0, self.1 + 1),
            '>' => Self(self.0 + 1, self.1),
            'v' => Self(self.0, self.1 - 1),
            '<' => Self(self.0 - 1, self.1),
            _ => *self,
        }
    }
}

fn get_visited_houses(data: &String) -> HashSet<Position> {
    data.chars()
        .fold(
            (
                once(Position::default()).collect::<HashSet<Position>>(),
                Position::default(),
            ),
            |(mut visited, current_position), c| {
                let new_position = current_position.parse_instruction(c);
                visited.insert(new_position);
                (visited, new_position)
            },
        )
        .0
}

fn solve_a(data: &String) -> usize {
    get_visited_houses(data).len()
}

fn solve_b(data: &String) -> usize {
    get_visited_houses(
        &data
            .chars()
            .enumerate()
            .filter_map(|(idx, c)| if idx % 2 == 0 { Some(c) } else { None })
            .collect(),
    )
    .union(&get_visited_houses(
        &data
            .chars()
            .enumerate()
            .filter_map(|(idx, c)| if idx % 2 == 1 { Some(c) } else { None })
            .collect(),
    ))
    .count()
}

fn main() {
    let lines = read_input();
    let data = lines.first().unwrap();

    let solution_a = solve_a(data);
    let solution_b = solve_b(data);

    println!("{}", solution_a);
    println!("{}", solution_b);
}
