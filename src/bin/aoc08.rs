use aoc_2015::io::read_input;

enum State {
    Init,
    String,
    Escape,
    Ascii(bool),
    End,
}

fn calculate_length(s: &String) -> u32 {
    s.chars()
        .fold((State::Init, 0u32), |(state, len), c| match c {
            '\\' => match state {
                State::Escape => (State::String, len + 1),
                State::String => (State::Escape, len),
                _ => panic!("invalid state"),
            },
            '"' => match state {
                State::Init => (State::String, len),
                State::Escape => (State::String, len + 1),
                State::String => (State::End, len),
                _ => panic!("invalid state"),
            },
            'x' => match state {
                State::Escape => (State::Ascii(false), len),
                State::String => (State::String, len + 1),
                _ => panic!("invalid state"),
            },
            c => match (c.is_digit(16), state) {
                (true, State::Ascii(false)) => (State::Ascii(true), len),
                (true, State::Ascii(true)) => (State::String, len + 1),
                (_, State::String) => (State::String, len + 1),
                _ => panic!("invalid state"),
            },
        })
        .1
}

fn encode_string(s: &String) -> String {
    let encoded: String = s
        .chars()
        .flat_map(|c| match c {
            '\\' | '\"' => vec!['\\', c],
            c => vec![c],
        })
        .collect();

    format!("\"{}\"", encoded)
}

fn solve_a(data: &Vec<String>) -> u32 {
    data.into_iter()
        .map(|s| s.len() as u32 - calculate_length(s))
        .sum::<u32>()
}

fn solve_b(data: &Vec<String>) -> u32 {
    data.into_iter()
        .map(|s| (encode_string(s).len() - s.len()) as u32)
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
    fn calc_length() {
        assert_eq!(calculate_length(&"\"\"".to_string()), 0);
        assert_eq!(calculate_length(&"\"abc\"".to_string()), 3);
        assert_eq!(calculate_length(&"\"aaa\\\"aaa\"".to_string()), 7);
        assert_eq!(calculate_length(&"\"\\x27\"".to_string()), 1);
    }
}
