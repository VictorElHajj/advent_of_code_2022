#[aoc_generator(day1)]
pub fn generator_part1(input: &str) -> Vec<Vec<usize>> {
    input
        .split("\n\n")
        .map(|chunk| {
            chunk
                .split("\n")
                .map(|line| line.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>()
}

#[aoc(day1, part1)]
pub fn part1(input: &Vec<Vec<usize>>) -> Result<usize, &str> {
    Ok(input.iter().map(|chunk| chunk.iter().sum()).max().unwrap())
}
#[aoc(day1, part2)]
pub fn part2(input: &Vec<Vec<usize>>) -> Result<usize, &str> {
    let mut calories_per_elf: Vec<usize> = input.iter().map(|chunk| chunk.iter().sum()).collect();
    calories_per_elf.sort_by(|a, b| b.cmp(a));
    Ok(calories_per_elf.iter().take(3).sum())
}
