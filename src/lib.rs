mod input;
mod puzzle;
mod solve;

pub use crate::input::get_input;
pub use crate::puzzle::Puzzle;
pub use crate::solve::Solver;

/// Take an implementation of Solver and use it to parse
/// the puzzle input and solve the puzzle
pub fn run<T: Solver>() -> Result<String, String> {
    let puzzle: Puzzle = T::puzzle();
    let input: String = get_input(puzzle)?;
    Ok(T::solve(input))
}
