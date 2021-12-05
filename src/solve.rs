use crate::Puzzle;

pub trait Solver {
    type Item;

    fn puzzle() -> Puzzle {
        Puzzle::today()
    }

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
