use crate::Puzzle;

/// Solver trait with some default implementations of functions to reduce the amount
/// of boiler plate needed to parse and process any puzzle input.
pub trait Solver {
    type Item;

    /// Returns which puzzle (year and day) the Solver is for. Will default to the
    /// current day if no explicit implementation is done.
    fn puzzle() -> Puzzle {
        Puzzle::today()
    }

    /// Take the whole input file and return an answer. Will try to solve for part
    /// two first if a solution has been implemented for the second part, else it
    /// will use the solution for part one.
    fn solve(input: String) -> String {
        let inputs: Vec<Self::Item> = Self::parse_input(input);
        let solution2: String = Self::solve_two(&inputs);
        if solution2.is_empty() {
            Self::solve_one(&inputs)
        } else {
            solution2
        }
    }

    fn parse_input(input: String) -> Vec<Self::Item> {
        input.lines().map(Self::parse).collect()
    }

    fn parse(line: &str) -> Self::Item;
    fn solve_one(inputs: &[Self::Item]) -> String;
    fn solve_two(_inputs: &[Self::Item]) -> String {
        String::new()
    }
}
