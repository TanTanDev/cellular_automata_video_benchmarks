use std::ops::RangeInclusive;

#[allow(dead_code)]
#[derive(Clone)]
pub enum Rule {
    Single(u8),
    Range(RangeInclusive<u8>),
    Singles(Vec<u8>),
}

impl Rule {
    pub fn in_range(&self, value: u8) -> bool {
        match self {
            Rule::Single(single) => value == *single,
            Rule::Range(range) => value < *range.end() && value > *range.start(),
            Rule::Singles(singles) => singles.iter().any(|v| *v == value),
        }
    }

    pub fn optimized(self) -> [bool; 27] {
        let mut array = [false; 27];
        match self {
            Rule::Single(v) => array[v as usize] = true,
            Rule::Range(range) => array[*range.start() as usize..=*range.end() as usize]
                .iter_mut()
                .for_each(|v| *v = true),
            Rule::Singles(values) => {
                values.iter().for_each(|i| array[*i as usize] = true);
            }
        }
        array
    }
}
