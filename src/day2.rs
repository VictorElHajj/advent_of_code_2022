#[derive(Debug)]
pub enum Plays {
    Rock,
    Paper,
    Scissors,
}

impl Plays {
    pub fn succ(&self) -> Self {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }

    pub fn prev(&self) -> Self {
        match self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }

    pub fn from_str(c: &str) -> Self {
        match c {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            e => panic!("Unrecognized character: {}", e),
        }
    }

    fn value(&self) -> usize {
        match self {
            Plays::Rock => 1,
            Plays::Paper => 2,
            Plays::Scissors => 3,
        }
    }

    pub fn play(&self, other: &Self) -> usize {
        let result: i32 = self.value() as i32 - other.value() as i32;
        match result {
            0 => 3 + self.value(),      // Draw
            1 | -2 => 6 + self.value(), // Win
            -1 | 2 => 0 + self.value(), // Loss
            _ => unreachable!(),
        }
    }
}

#[aoc_generator(day2)]
pub fn generator_part1(input: &str) -> Vec<(Plays, Plays)> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split(' ');
            (
                Plays::from_str(split.next().unwrap()),
                Plays::from_str(split.next().unwrap()),
            )
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &Vec<(Plays, Plays)>) -> usize {
    input.iter().map(|(other, you)| you.play(other)).sum()
}

#[aoc(day2, part2)]
pub fn part2(input: &Vec<(Plays, Plays)>) -> usize {
    input
        .iter()
        .map(|(other, you)| {
            match you {
                Plays::Rock => other.prev().play(other),     // Loss
                Plays::Paper => other.play(other),           // Draw
                Plays::Scissors => other.succ().play(other), // Win
            }
        })
        .sum()
}
