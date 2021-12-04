#[derive(Debug, Clone, Copy)]
pub struct Puzzle {
    pub(crate) year: u16,
    pub(crate) day: u8,
}

impl Puzzle {
    pub fn new(year: u16, day: u8) -> Puzzle {
        Puzzle { year, day }
    }
}

impl std::fmt::Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.year, self.day)
    }
}
