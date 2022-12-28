use std::str::FromStr;

use rand::{seq::SliceRandom, thread_rng};

pub const BLOOD_TYPES: [&str; 4] = ["A", "B", "O", "AB"];

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
    #[allow(dead_code)]
    natural_age: isize,
    #[allow(dead_code)]
    biologic_age: isize,
}

impl std::fmt::Display for Human {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}", self.first_name, self.last_name)
    }
}

impl<T> From<(&str, T, T, isize, isize)> for Human
where
    T: ToString,
{
    fn from(data: (&str, T, T, isize, isize)) -> Self {
        Human {
            blood_type: BloodType::from_str(data.0).unwrap_or(BloodType::O),
            first_name: data.1.to_string(),
            last_name: data.2.to_string(),
            natural_age: data.3,
            biologic_age: data.4,
        }
    }
}

impl Human {
    pub fn breed(p1: Human, p2: Human) -> Human {
        Human {
            // TODO
            // Actual genetic inheritance, not just random select
            blood_type: [p1.blood_type, p2.blood_type].choose(&mut thread_rng()).unwrap_or(&BloodType::O).clone(),
            first_name: p1.first_name,
            last_name: p2.last_name,
            natural_age: 0,
            biologic_age: 0,
        }
    }
}
