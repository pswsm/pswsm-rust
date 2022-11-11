use std::str::FromStr;

pub const BLOOD_TYPES: [&str; 4] = ["A", "B", "O", "AB"];

#[derive(Debug)]
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
            _ => unreachable!()
        })
    }
}

pub struct Human {
    blood_type: BloodType,
    first_name: String,
    last_name: String,
    natural_age: usize,
    biologic_age: usize,
}

impl std::fmt::Display for Human {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}", self.first_name, self.last_name)
    }
}

impl<T> From<(&str, T, T, usize, usize)> for Human where 
T: ToString
{
    fn from(data: (&str, T, T, usize, usize)) -> Self {
        Human {
            blood_type: BloodType::from_str(data.0).unwrap_or(BloodType::O),
            first_name: data.1.to_string(),
            last_name: data.2.to_string(),
            natural_age: data.3,
            biologic_age: data.4,
        }
    }
}
