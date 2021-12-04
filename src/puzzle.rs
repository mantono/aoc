use chrono::{Datelike, Utc};

#[derive(Debug, Clone, Copy)]
pub struct Puzzle {
    pub(crate) year: u16,
    pub(crate) day: u8,
}

impl Puzzle {
    pub fn new(year: u16, day: u8) -> Puzzle {
        Puzzle { year, day }
    }

    pub fn today() -> Puzzle {
        // Puzzles unlock at midnight EST/UTC-5
        let offset = chrono::FixedOffset::west(5 * 3600);
        let date = Utc::now().with_timezone(&offset);
        Puzzle {
            year: date.year() as u16,
            day: date.day() as u8,
        }
    }
}

impl Default for Puzzle {
    fn default() -> Self {
        Self::today()
    }
}

impl std::fmt::Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.year, self.day)
    }
}
