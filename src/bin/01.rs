fn main() -> Result<(), Box<dyn std::error::Error>>  {

    let lines = aoc2023::load_input_data(2023,1)?;

    println!("Answer part 1 - {}", part1(&lines));
    println!("Answer part 2 - {}", part2(&lines));
    Ok(())

}

fn part1(lines: &Vec<String>) -> u32 {

    let mut answer: u32 = 0;
    lines.iter().for_each(|line| {
        let digits: Vec<u32> = line.chars().filter(|c| c.is_digit(10)).map(|c| c.to_digit(10).unwrap()).collect();
        answer += (digits.get(0).unwrap() * 10) + digits.last().unwrap();
    });
    answer
}

fn part2(lines: &Vec<String>) -> u32 {
    let transformed: Vec<String> = lines.iter().map(|l|
        l.replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e")
    ).collect();
    part1(&transformed)
}
