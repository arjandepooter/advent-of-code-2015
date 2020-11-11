use aoc_2015::io::read_input;

struct RepeatCount<I>
where
    I: Iterator,
{
    iter: I,
    current: Option<I::Item>,
}

impl<I> RepeatCount<I>
where
    I: Iterator,
{
    fn new(mut iter: I) -> Self {
        let current = iter.next();

        RepeatCount { iter, current }
    }
}

impl<I> Iterator for RepeatCount<I>
where
    I: Iterator,
    I::Item: PartialEq,
    I::Item: Copy,
{
    type Item = (I::Item, usize);

    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            Some(cur) => {
                let mut count: usize = 1;
                loop {
                    let next_item = self.iter.next();
                    match next_item {
                        Some(nxt) if nxt == cur => {
                            count += 1;
                        }
                        _ => {
                            self.current = next_item;
                            break;
                        }
                    };
                }
                Some((cur, count))
            }
            None => None,
        }
    }
}

trait RepeatCountTrait: Iterator {
    fn repeat_count(self) -> RepeatCount<Self>
    where
        Self: Sized,
        Self::Item: Clone,
    {
        RepeatCount::new(self)
    }
}

impl<T: ?Sized> RepeatCountTrait for T where T: Iterator {}

fn look_and_say(start: &String, n: usize) -> String {
    let mut result = start.clone();

    for _ in 0..n {
        result = result
            .chars()
            .repeat_count()
            .map(|(c, count)| format!("{}{}", count, c))
            .collect();
    }

    result
}

fn solve_a(start: &String) -> usize {
    look_and_say(start, 40).len()
}

fn solve_b(start: &String) -> usize {
    look_and_say(start, 50).len()
}

fn main() -> Result<(), &'static str> {
    let data = read_input();
    let s = data.first().ok_or("Error reading input")?;

    let solution_a = solve_a(s);
    let solution_b = solve_b(s);

    println!("{}", solution_a);
    println!("{}", solution_b);

    Ok(())
}
