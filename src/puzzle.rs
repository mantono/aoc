#[derive(Debug, Clone, Copy)]
pub struct Puzzle {
    pub(crate) year: u16,
    pub(crate) day: u8,
    pub(crate) part: Part,
}

impl Puzzle {
    pub fn new(year: u16, day: u8, part: Part) -> Puzzle {
        Puzzle { year, day, part }
    }
}

impl std::fmt::Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let part: &str = match self.part {
            Part::One => "one",
            Part::Two => "two",
        };
        write!(f, "{} day {} part {}", self.year, self.day, part)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Part {
    One,
    Two,
}
