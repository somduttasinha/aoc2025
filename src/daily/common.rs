use clap::{ValueEnum, builder::PossibleValue};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Part {
    Part1 = 0,
    Part2 = 1,
}

impl ValueEnum for Part {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Part1, Self::Part2]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            Self::Part1 => PossibleValue::new("1"),
            Self::Part2 => PossibleValue::new("2"),
        })
    }
}

pub fn read(path: &str) -> String {
    return std::fs::read_to_string(path).expect("Expected content in file");
}
