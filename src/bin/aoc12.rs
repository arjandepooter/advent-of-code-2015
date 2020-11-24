use aoc_2015::io::read_input;
use json::ast::Json;
use json::decoder::Decoder;

fn sum(json: &Json) -> f64 {
    match json {
        Json::Number(n) => *n,
        Json::Array(arr) => arr.into_iter().map(sum).sum(),
        Json::Object(obj) => obj.values().into_iter().map(sum).sum(),
        _ => 0f64,
    }
}

fn solve_a(s: &String) -> f64 {
    let mut decoder = Decoder::default(s.chars());
    let json = decoder.decode().unwrap();
    sum(&json)
}

fn sum_ignore_red(json: &Json) -> f64 {
    match json {
        Json::Number(n) => *n,
        Json::Array(arr) => arr.into_iter().map(sum_ignore_red).sum(),
        Json::Object(obj) => {
            if obj.values().into_iter().any(|v| match v {
                Json::String(s) => s == "red",
                _ => false,
            }) {
                0f64
            } else {
                obj.values().into_iter().map(sum_ignore_red).sum()
            }
        }
        _ => 0f64,
    }
}

fn solve_b(s: &String) -> f64 {
    let mut decoder = Decoder::default(s.chars());
    let json = decoder.decode().unwrap();
    sum_ignore_red(&json)
}

fn main() {
    let data = read_input();
    let json = data.first().unwrap();

    let solution_a = solve_a(json);
    let solution_b = solve_b(json);

    println!("{}", solution_a);
    println!("{}", solution_b);
}
