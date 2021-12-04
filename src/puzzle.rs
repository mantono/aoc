#[derive(Debug, Clone, Copy)]
pub struct Puzzle {
    pub year: u16,
    pub day: u8,
    pub part: Part,
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
