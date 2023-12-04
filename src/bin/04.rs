use regex::Regex;

use std::collections::HashSet;


fn main() -> Result<(), Box<dyn std::error::Error>>  {

    let lines = aoc2023::load_input_data(2023,4)?;
    //let lines: Vec<String> = aoc2023::load_sample_data("/Users/bnolan/workspace/bstick12/aoc.rs/aoc2023/src/resources/day04_sample.txt")?;

    println!("Answer part 1 - {}", part1(&lines));
    println!("Answer part 2 - {}", part2(&lines));
    Ok(())

}

fn part1(lines: &Vec<String>) -> u32 {

    let mut answer: u32 = 0;

    let card_pattern = Regex::new(r"Card\s+\d+: ([^|]*)\|(.*)").unwrap();
    let number_pattern = Regex::new(r"(\d+)").unwrap();

    for line in lines {
        for groups in card_pattern.captures_iter(line) {
            let numbers = get_numbers(&number_pattern, &groups[1]);
            let winning = get_numbers(&number_pattern, &groups[2]);
            let winners: HashSet<u32> = numbers.intersection(&winning).cloned().collect();
            if winners.len() > 0 {
                answer += u32::pow(2, winners.len() as u32 - 1);
            }
        }
    }

    answer

}

fn get_numbers(pattern: &Regex, numbers: &str) -> HashSet<u32> {
    pattern.captures_iter(numbers)
                .flat_map(|captures| captures.get(1))
                .map(|capture| capture.as_str().parse::<u32>().unwrap())
                .collect()
}

fn part2(lines: &Vec<String>) -> u32 {

    let card_pattern = Regex::new(r"Card\s+\d+: ([^|]*)\|(.*)").unwrap();
    let number_pattern = Regex::new(r"(\d+)").unwrap();

    let mut wins: Box<[usize]> = vec![0; lines.len() + 1].into_boxed_slice();

    for (index, line) in lines.iter().enumerate() {
        for groups in card_pattern.captures_iter(line) {
            let numbers = get_numbers(&number_pattern, &groups[1]);
            let winning = get_numbers(&number_pattern, &groups[2]);
            let winners: HashSet<u32> = numbers.intersection(&winning).cloned().collect();
            wins[index+1] = winners.len();
        }
    }

    let mut cards: Box<[usize]> = vec![0; lines.len() + 1].into_boxed_slice();
    for i in 1..wins.len() {
        cards[i] += 1;
        for j in i+1..=i+wins[i] {
            cards[j] += cards[i];
        }
    }

    let sum: usize = cards.iter().sum();
    sum as u32
}

