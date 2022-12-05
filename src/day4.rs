use std::ops::RangeInclusive;

pub struct Ranges {
    pub max: u8,
    pub min: u8,
    pub range: RangeInclusive<u8>,
}

#[aoc_generator(day4)]
pub fn generator(input: &str) -> Vec<(Ranges, Ranges)> {
    input
        .lines()
        .map(|line| {
            let mut range_iter = line.split(',');
            let r1: Vec<u8> = range_iter
                .next()
                .unwrap()
                .split('-')
                .take(2)
                .map(|r| r.parse::<u8>().unwrap())
                .collect();
            let r2: Vec<u8> = range_iter
                .next()
                .unwrap()
                .split('-')
                .take(2)
                .map(|r| r.parse::<u8>().unwrap())
                .collect();
            (
                Ranges {
                    min: r1[0],
                    max: r1[1],
                    range: r1[0]..=r1[1],
                },
                Ranges {
                    min: r2[0],
                    max: r2[1],
                    range: r2[0]..=r2[1],
                },
            )
        })
        .collect()
}

#[aoc(day4, part1)]
pub fn part1(input: &Vec<(Ranges, Ranges)>) -> usize {
    input
        .iter()
        .filter(|&(r1, r2)| {
            (r1.range.contains(&r2.max) && r1.range.contains(&r2.min))
                || (r2.range.contains(&r1.max) && r2.range.contains(&r1.min))
        })
        .count()
}

#[aoc(day4, part2)]
pub fn part2(input: &Vec<(Ranges, Ranges)>) -> usize {
    input
        .iter()
        .filter(|&(r1, r2)| {
            (r1.range.contains(&r2.max) || r1.range.contains(&r2.min))
                || (r2.range.contains(&r1.max) || r2.range.contains(&r1.min))
        })
        .count()
}
