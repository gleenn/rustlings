// aoc.rs

const INPUT1: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";

fn run(input: &String) -> i32 {
    let result = input.trim().split("\n\n")
        .map(|elves_work|
            elves_work
                .split("\n")
                .map(|amount| amount.parse::<i32>().unwrap())
                .sum())
        .max().unwrap();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        assert_eq!(24000, run(&INPUT1.to_string()));
    }
}
