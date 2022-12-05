#[derive(Debug)]
pub struct Day5 {
    stacks: [Vec<char>; 9],
    moves: Vec<Move>,
}

#[derive(Debug)]
pub struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

#[aoc_generator(day5)]
pub fn generator(input: &str) -> Day5 {
    let (stack_str, move_str) = input.split_once("\n\n").unwrap();

    let mut stacks: [Vec<char>; 9] = Default::default();
    // Parse stacks in reverse order, skip line with stack numbers
    for line in stack_str.lines().map(str::to_string).rev().skip(1) {
        let line_chars: Vec<char> = line.chars().collect();
        for i in 0..9 {
            match line_chars[1 + i * 4] {
                ' ' => {}
                c => stacks[i].push(c),
            }
        }
    }
    // Parse instructions
    let moves = move_str
        .lines()
        .map(|line| {
            let words: Vec<&str> = line.split(' ').collect();
            let amount = words[1].parse::<usize>().unwrap();
            let from = words[3].parse::<usize>().unwrap();
            let to = words[5].parse::<usize>().unwrap();
            Move { amount, from, to }
        })
        .collect();

    Day5 { stacks, moves }
}

#[aoc(day5, part1)]
pub fn part1(input: &Day5) -> String {
    let mut stacks = input.stacks.clone();
    for Move { amount, from, to } in input.moves.iter() {
        for _ in 0..*amount {
            let removed = stacks[*from - 1].pop().unwrap();
            stacks[*to - 1].push(removed);
        }
    }
    stacks
        .iter_mut()
        .map(|stack| stack.pop().unwrap())
        .collect::<String>()
}

#[aoc(day5, part2)]
pub fn part2(input: &Day5) -> String {
    let mut stacks = input.stacks.clone();
    for Move { amount, from, to } in input.moves.iter() {
        let len = stacks[*from - 1].len();
        let mut removed: Vec<char> = stacks[*from - 1].drain(len - amount..).collect();
        stacks[*to - 1].append(&mut removed);
    }
    stacks
        .iter_mut()
        .map(|stack| stack.pop().unwrap())
        .collect::<String>()
}
