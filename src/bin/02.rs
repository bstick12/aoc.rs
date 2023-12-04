use regex::Regex;

use std::collections::HashMap;


fn main() -> Result<(), Box<dyn std::error::Error>>  {

    let lines = aoc2023::load_input_data(2023,2)?;

    println!("Answer part 1 - {}", part1(&lines));
    println!("Answer part 2 - {}", part2(&lines));
    Ok(())

}

fn part1(lines: &Vec<String>) -> u32 {

    let mut answer: u32 = 0;

    let mut cubes: HashMap<&str, u32> = HashMap::<&str, u32>::new();
    cubes.insert("red", 12);
    cubes.insert("green", 13);
    cubes.insert("blue", 14);

    let game_pattern = Regex::new(r"Game (\d+): (.*)").unwrap();
    let roll_pattern = Regex::new(r"(\d+) (blue|red|green)").unwrap();

    for line in lines {
        for cap in game_pattern.captures_iter(line) {
            let game_id = &cap[1].parse::<u32>().unwrap();
            answer += game_id;
            for x in roll_pattern.captures_iter( &cap[2]) {
                let dice = &x[1].parse::<u32>().unwrap();
                let colour = &x[2];
                if cubes.get(colour).unwrap() < dice {
                    answer -= game_id;
                    break;
                }
            }
        }
    }

    answer
}

fn part2(lines: &Vec<String>) -> u32 {
    let mut answer: u32 = 0;

    let game_pattern = Regex::new(r"Game (\d+): (.*)").unwrap();
    let roll_pattern = Regex::new(r"(\d+) (blue|red|green)").unwrap();

    for line in lines {
        let mut cubes: HashMap<String, u32> = HashMap::<String, u32>::new();
        for cap in game_pattern.captures_iter(line) {
            for x in roll_pattern.captures_iter( &cap[2]) {
                let dice = x[1].parse::<u32>().unwrap();
                let colour = x[2].to_string();
                cubes.entry(colour).and_modify(|v| *v = dice.max(*v)).or_insert(dice);
            }
        }
        let product: u32 = cubes.iter().fold(1, |acc, (_, value)| acc * value);
        answer += product;
    }
    answer
}
