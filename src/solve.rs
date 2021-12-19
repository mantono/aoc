use std::str::FromStr;

use crate::Puzzle;

/// Solver trait with some default implementations of functions to reduce the amount
/// of boiler plate needed to parse and process any puzzle input.
pub trait Solver {
    type Item: FromStr;
    /// Returns which puzzle (year and day) the Solver is for. Will default to the
    /// current day if no explicit implementation is done.
    fn puzzle() -> Puzzle {
        Puzzle::today()
    }

    /// Take the whole input file and return an answer. Will try to solve for part
    /// two first if a solution has been implemented for the second part, else it
    /// will use the solution for part one.
    fn solve<T: Into<String>>(input: T) -> String {
        let inputs: Vec<Self::Item> = Self::parse_input(input);
        let solution2: String = Self::solve_two(&inputs);
        if solution2.is_empty() {
            Self::solve_one(&inputs)
        } else {
            solution2
        }
    }

    /// This is your solution to the first part of the puzzle
    fn solve_one(inputs: &[Self::Item]) -> String;

    /// This is your solution to the second part of the puzzle
    fn solve_two(inputs: &[Self::Item]) -> String {
        drop(inputs);
        String::new()
    }

    /// Takes the puzzle input and
    /// - breaks it down to lines
    /// - trims any leading or trailing whitespace on each line
    /// - removes any lines that are empty
    /// - calls the user implemented parse function
    ///
    /// and return the input as a Vec of Self::Item.
    ///
    /// The trimming of whitespace and filtering of empty lines is
    /// not required for the regular puzzle input, but can be needed
    /// when writing test cases based on example input, if they are
    /// defined as raw literal strings which may have some leading
    /// whitespace.
    ///
    /// So for exmaple, an input like this, which has leading whitespace
    /// and begins and ends with lines only having whitespace, would
    /// still work.
    ///
    /// ```
    ///    let input = r#"
    ///        00100
    ///        11110
    ///        10110
    ///        00010
    ///        01010
    ///    "#;
    /// ```
    ///
    /// Puzzle inputs are rarely, if ever, whitespace dependent or contains
    /// any whitespaces. But if any puzzle input would require whitespaces
    /// and they would significant for solving the puzzle, it would probably
    /// be best to override the default implementation of this function.
    fn parse_input<T: Into<String>>(input: T) -> Vec<Self::Item> {
        input
            .into()
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .map(|line| match Self::parse(line) {
                Ok(val) => val,
                Err(_) => panic!("Unable to parse line: '{}'", line),
            })
            .collect()
    }

    /// By default, the types implementation of `FromStr` will be used to convert
    /// the `&str` into type `Self::Item`. But if for some reason you want to use
    /// override the parsing of the string it can be done by implementing a different
    /// version of it here.
    fn parse(line: &str) -> Result<Self::Item, <Self::Item as FromStr>::Err> {
        line.parse()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solver;

    struct Aoc;

    impl Solver for Aoc {
        type Item = usize;

        fn solve_one(inputs: &[Self::Item]) -> String {
            inputs.iter().sum::<usize>().to_string()
        }
    }

    const INPUT: &str = r#"
        12
        912
        233
        23
    "#;

    #[test]
    fn test_parser() {
        let parsed: Vec<usize> = Aoc::parse_input(INPUT);
        assert_eq!(vec![12, 912, 233, 23], parsed);
    }

    #[test]
    fn test_solver() {
        assert_eq!(String::from("1180"), Aoc::solve(INPUT));
    }
}
