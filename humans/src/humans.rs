use dates_str::*;
use rand::{seq::SliceRandom, thread_rng};
use std::str::FromStr;

/// Blood types enum
#[derive(Debug, Clone)]
pub enum BloodType {
    A,
    B,
    O,
    AB,
}

impl std::fmt::Display for BloodType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let btype: &str = match self {
            Self::A => "A",
            Self::B => "B",
            Self::O => "O",
            Self::AB => "AB",
        };
        write!(f, "{}", btype)
    }
}

impl FromStr for BloodType {
    type Err = std::string::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" => Self::A,
            "B" => Self::B,
            "O" => Self::O,
            "AB" => Self::AB,
            _ => unreachable!(),
        })
    }
}

pub struct Human {
    blood_type: BloodType,
    first_name: String,
    last_name: String,
    date_born: DateStr,
}

impl std::fmt::Display for Human {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}", self.first_name, self.last_name)
    }
}

impl Human {
    pub fn new<T: ToString>(blood_type: BloodType, first_name: T, last_name: T, date_born: T) -> Self {
        Human {
            blood_type,
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            date_born: DateStr::from_iso_str(date_born),
        }
    }
}

impl Human {
    pub fn breed(p1: Human, p2: Human) -> Human {
        Human {
            // TODO
            // Actual genetic inheritance, not just random select
            blood_type: [p1.blood_type, p2.blood_type]
                .choose(&mut thread_rng())
                .unwrap_or(&BloodType::O)
                .clone(),
            first_name: p1.first_name,
            last_name: p2.last_name,
            date_born: DateStr::from_iso_str("2003-20-10"),
        }
    }
}
