use std::collections::HashMap;

use num_integer::lcm;
use regex::Regex;

fn main() -> Result<(), Box<dyn std::error::Error>>  {

    let lines = aoc2023::load_input_data(2023,8)?;
    println!("Answer part 1 - {}", part1(&lines));
    println!("Answer part 1 - {}", part2(&lines));
    Ok(())

}

fn part1(lines: &Vec<String>) -> usize {


    let instructions = lines.get(0).unwrap().chars();

    let path_pattern = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();

    let mut path_map: HashMap<String, (String, String)> = HashMap::new();

    for i in 2..lines.len() {
        for groups in path_pattern.captures_iter(lines.get(i).unwrap()) {
            let path: (String, String) = (groups[2].to_string(), groups[3].to_string());
            path_map.insert(groups[1].to_string(), path);
        }
    }

    let mut current_pos = "AAA";
    let mut looping_instructions = instructions.cycle();
    let mut steps: usize = 0;
    while !current_pos.eq("ZZZ") {
        steps += 1;
        let path = path_map.get(current_pos).unwrap();
        if let Some(direction) = looping_instructions.next() {
            if direction == 'R' {
                current_pos = path.1.as_str();
            } else {
                current_pos = path.0.as_str();
            }
        }
    }

    steps

}

fn part2(lines: &Vec<String>) -> usize {


    let instructions = lines.get(0).unwrap();

    let path_pattern = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();

    let mut path_map: HashMap<String, (String, String)> = HashMap::new();

    for i in 2..lines.len() {
        for groups in path_pattern.captures_iter(lines.get(i).unwrap()) {
            let path: (String, String) = (groups[2].to_string(), groups[3].to_string());
            path_map.insert(groups[1].to_string(), path);
        }
    }

    let mut step_counts: Vec<usize> = Vec::new();
    for start in path_map.keys().filter(|k| k.ends_with("A")) {
        let mut current_pos = start.as_str();
        let mut looping_instructions = instructions.chars().cycle();
        let mut steps: usize = 0;
        while !current_pos.ends_with("Z") {
            steps += 1;
            let path = path_map.get(current_pos).unwrap();
            if let Some(direction) = looping_instructions.next() {
                if direction == 'R' {
                    current_pos = path.1.as_str();
                } else {
                    current_pos = path.0.as_str();
                }
            }
        }
        step_counts.push(steps);
    }

    step_counts.iter().fold(1, |x,&y| lcm(x,y))

}