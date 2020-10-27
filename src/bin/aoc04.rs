use md5::{Digest, Md5};

use aoc_2015::io::read_input;

fn check_hash(hash: &Vec<u8>, leading_zeroes: usize) -> bool {
    hash.into_iter()
        .flat_map(|byte| vec![byte >> 4 as u8, byte & 0xf as u8])
        .take(leading_zeroes)
        .all(|n| n == 0)
}

fn find_hash(key: &String, leading_zeroes: usize) -> u32 {
    let mut hasher = Md5::new();
    hasher.update(key);

    (0..)
        .map(|n| {
            let mut hasher = hasher.clone();
            hasher.update(n.to_string());
            let hash: Vec<u8> = hasher.finalize().into_iter().collect();
            (n, hash)
        })
        .find(|(_, hash)| check_hash(hash, leading_zeroes))
        .unwrap()
        .0
}

fn solve_a(key: &String) -> u32 {
    find_hash(key, 5)
}

fn solve_b(key: &String) -> u32 {
    find_hash(key, 6)
}

fn main() {
    let data = read_input();
    let solution_a = solve_a(data.first().unwrap());
    let solution_b = solve_b(data.first().unwrap());

    println!("{}", solution_a);
    println!("{}", solution_b);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_checks() {
        assert_eq!(check_hash(&vec![0, 0, 0, 1], 6), true);
        assert_eq!(check_hash(&vec![0, 0, 0x05, 1], 6), false);
        assert_eq!(check_hash(&vec![0, 0, 0x05, 1], 5), true);
    }
}
