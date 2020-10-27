use aoc_2015::io::read_input;

fn solve_a(data: &String) -> i32 {
    data.chars().fold(0, |lvl, c| match c {
        '(' => lvl + 1,
        ')' => lvl - 1,
        _ => lvl,
    })
}

fn solve_b(data: &String) -> i32 {
    data.chars()
        .enumerate()
        .try_fold(0i32, |lvl, (idx, c)| {
            Ok(match c {
                '(' => lvl + 1,
                ')' => lvl - 1,
                _ => lvl,
            })
            .and_then(|lvl| {
                if lvl == -1 {
                    Err(idx as i32 + 1)
                } else {
                    Ok(lvl)
                }
            })
        })
        .unwrap_err()
}

fn main() {
    let lines = read_input();
    let data = lines.first().unwrap();

    let solution_a = solve_a(data);
    let solution_b = solve_b(data);

    println!("{}", solution_a);
    println!("{}", solution_b);
}
