use itertools::Itertools;
use std::collections::HashMap;

use aoc_2015::io::read_input;

type Lookup = HashMap<String, HashMap<String, usize>>;

fn get_distances(data: &Vec<String>) -> Lookup {
    let mut distances = HashMap::new();

    for line in data {
        let (city_str, distance_str) = line.split(" = ").collect_tuple().expect("Invalid line");

        let distance: usize = distance_str.parse().expect("Invalid distance");
        let (city1, city2) = city_str
            .split(" to ")
            .collect_tuple()
            .expect("Invalid cities");

        distances
            .entry(city1.into())
            .or_insert(HashMap::new())
            .insert(city2.into(), distance);

        distances
            .entry(city2.into())
            .or_insert(HashMap::new())
            .insert(city1.into(), distance);
    }

    distances
}

fn iter_distances<'a>(lookup: &'a Lookup) -> impl 'a + Iterator<Item = usize> {
    let cities = lookup.keys().clone();

    cities.permutations(lookup.len()).map(move |permutation| {
        let mut cities = permutation.clone().into_iter();
        let start = cities.next().unwrap();

        let (_, result) = cities.fold((start, 0), |(from, distance), to| {
            (
                to,
                distance + *lookup.get(from).and_then(|m| m.get(to)).unwrap(),
            )
        });

        result
    })
}

fn solve_a(lookup: &Lookup) -> usize {
    iter_distances(lookup).min().unwrap()
}

fn solve_b(lookup: &Lookup) -> usize {
    iter_distances(lookup).max().unwrap()
}

fn main() {
    let data = read_input();
    let distances = get_distances(&data);

    let solution_a = solve_a(&distances);
    let solution_b = solve_b(&distances);

    println!("{}", solution_a);
    println!("{}", solution_b);
}
