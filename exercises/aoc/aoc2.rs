// I AM NOT DONE

const

fn score(line: &str) -> i32 {
    line.split(" ").map(|char|)
}

fn score_all(line: &str) -> i32 {
    0
}

fn run(input: &String) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn score_test() {
        assert_eq!(8, score(&"A Y"));
        assert_eq!(-1, score(&"B X"));
        assert_eq!(6, score(&"C Z"));
    }

    #[test]
    fn score_all_test() {
        assert_eq!(15, score(&"A Y\nB X\nC Z"));
    }
}
