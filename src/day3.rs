pub struct Rucksack {
    whole: String,
    comp1: String,
    comp2: String,
}

impl Rucksack {
    pub fn find_in_common(&self) -> Option<char> {
        for c in self.comp1.chars() {
            if self.comp2.contains(c) {
                return Some(c);
            }
        }
        None
    }

    pub fn find_in_common_3(&self, other1: &Self, other2: &Self) -> Option<char> {
        for c in self.whole.chars() {
            if other1.whole.contains(c) && other2.whole.contains(c) {
                return Some(c);
            }
        }
        None
    }
}

pub fn char_priority(c: char) -> usize {
    let b = c as u8;
    if b < b'a' {
        return (b - 38) as usize;
    } else {
        return (b - 96) as usize;
    }
}

#[aoc_generator(day3)]
pub fn generator_part1(input: &str) -> Vec<Rucksack> {
    input
        .lines()
        .map(|line| {
            let (c1, c2) = line.split_at(line.len() / 2);
            Rucksack {
                whole: line.to_string(),
                comp1: c1.to_string(),
                comp2: c2.to_string(),
            }
        })
        .collect()
}
#[aoc(day3, part1)]
pub fn part1(input: &Vec<Rucksack>) -> usize {
    input
        .iter()
        .map(|rucksack| char_priority(rucksack.find_in_common().unwrap()))
        .sum()
}

#[aoc(day3, part2)]
pub fn part2(input: &Vec<Rucksack>) -> usize {
    input
        .chunks(3)
        .flat_map(<&[Rucksack; 3]>::try_from)
        .map(|[a, b, c]| char_priority(a.find_in_common_3(b, c).unwrap()))
        .sum()
}
