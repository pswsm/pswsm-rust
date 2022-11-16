use std::vec::Vec;
use std::marker::PhantomData;

#[derive(PartialEq)]
pub struct DateStr<T>
where T: ToString
{
    pub year: u32,
    pub month: u8,
    pub day: u8,
    _og_date: PhantomData<T>
}

impl<T> DateStr<T>
where T: ToString
{
    pub fn from_iso_str(string: T) -> DateStr<T>
    {
        let sep_date: Vec<String> = string.to_string().split('-').into_iter().map(|split| split.to_string() ).collect();
        let year: u32 = sep_date[0].parse::<u32>().unwrap_or_default();
        let month: u8  = sep_date[1].parse::<u8>().unwrap_or_default();
        let day: u8  = sep_date[2].parse::<u8>().unwrap_or_default();
        DateStr { year, month, day, _og_date: PhantomData }
    }
}


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iso_str() {
        let some_date: DateStr<_> = DateStr::from_iso_str("2022-11-16");
        assert_eq!(some_date.year, 2022);
    }
}
