#[derive(Debug)]
pub enum BloodType {
    A,
    B,
    O
}

impl std::fmt::Display for BloodType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let btype: char = match self {
            Self::A => 'A',
            Self::B => 'B',
            Self::O => 'O'
        };
        write!(f, "{}", btype)
    }
}

pub struct Blood {
  genotype: BloodType,
  fenotype: BloodType,
}

impl std::fmt::Display for Blood {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.genotype)
    }
}

impl From<(char, [char; 2])> for Blood {
    fn from(iterable: (char, [char; 2])) -> Self {
        Blood { genotype: iterable.0.to_string(), fenotype: iterable.1.iter().map(|c| c.to_string()).collect() }
    }
}

pub struct Human {
    blood_type: Blood,
    first_name: String,
    last_name: String,
    natural_age: usize,
    biologic_age: usize,
} 

impl std::fmt::Display for Human {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.first_name)
    }
}

impl<T: ToString> From<(Blood, T, T, usize, usize)> for Human {
    fn from(data: (Blood, T, T, usize, usize)) -> Self {
        Human { blood_type: data.0, first_name: data.1.to_string(), last_name: data.2.to_string(), natural_age: data.3, biologic_age: data.4 }
    }
}
