pub mod io {
    use std::io::{stdin, BufRead};

    pub fn read_input() -> Vec<String> {
        let stdin = stdin();
        let lines = stdin.lock().lines();
        lines.map(|line| line.unwrap()).collect()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
