use aoc_2015::io::read_input;

fn contains_three_vowels(str: &String) -> bool {
    str.chars().filter(|c| "aeiou".contains(*c)).count() >= 3
}

fn contains_letter_twice_in_a_row(str: &String) -> bool {
    str.chars().zip(str.chars().skip(1)).any(|(a, b)| a == b)
}

fn does_not_contain_evil_sets(str: &String) -> bool {
    ["ab", "cd", "pq", "xy"]
        .iter()
        .all(|set| !str.contains(set))
}

fn contains_double_pair(str: &String) -> bool {
    (0..str.len() - 2)
        .map(|n| (&str[n..n + 2], &str[n + 2..]))
        .any(|(needle, haystack)| haystack.contains(needle))
}

fn contains_gapped_pair(str: &String) -> bool {
    str.chars().zip(str.chars().skip(2)).any(|(a, b)| a == b)
}

fn solve_a(data: &Vec<String>) -> usize {
    data.clone()
        .into_iter()
        .filter(contains_three_vowels)
        .filter(contains_letter_twice_in_a_row)
        .filter(does_not_contain_evil_sets)
        .count()
}

fn solve_b(data: &Vec<String>) -> usize {
    data.clone()
        .into_iter()
        .filter(contains_double_pair)
        .filter(contains_gapped_pair)
        .count()
}

fn main() {
    let data = read_input();

    let solution_a = solve_a(&data);
    let solution_b = solve_b(&data);

    println!("{}", solution_a);
    println!("{}", solution_b);
}
