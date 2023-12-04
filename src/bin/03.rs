use regex::Match;
use regex::Regex;

use std::collections::HashMap;
use std::collections::HashSet;

fn main() -> Result<(), Box<dyn std::error::Error>>  {

    let lines = aoc2023::load_input_data(2023,3)?;
    //let lines: Vec<String> = aoc2023::load_sample_data("/Users/bnolan/workspace/bstick12/aoc.rs/aoc2023/src/resources/day03_sample1.txt")?;

    println!("Answer part 1 - {}", part1(&lines));
    println!("Answer part 2 - {}", part2(&lines));
    Ok(())

}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

fn part1(lines: &Vec<String>) -> u32 {

    let mut answer: u32 = 0;

    let symbol_pattern = Regex::new(r"([^0-9.])").unwrap();

    let mut symbols = HashSet::new();

    for (index, line) in lines.iter().enumerate() {
        for symbol in symbol_pattern.captures_iter(line) {
            symbols.insert(Point{x: index as isize, y: symbol.get(0).unwrap().start() as isize});
        }
    }

    let number_pattern = Regex::new(r"(\d+)").unwrap();

    for (index, line) in lines.iter().enumerate() {
        for number in number_pattern.captures_iter(line) {
            if is_adjacent(&symbols, index as isize, number.get(0).unwrap()) {
                answer += number[0].parse::<u32>().unwrap();
            }
        }
    }

    answer

}

fn is_adjacent(symbols: &HashSet<Point>, x: isize, m: Match) -> bool {
    for j in (x-1)..=(x+1) {
        for k in (m.start() as isize - 1)..=(m.start() + m.len()) as isize {
            if symbols.contains(&Point{x: j, y: k}) {
                return true
            }
        }
    }
    false
}

fn part2(lines: &Vec<String>) -> u32 {
    let mut answer: u32 = 0;

    let symbol_pattern = Regex::new(r"(\*)").unwrap();

    let mut symbols = HashSet::new();

    for (index, line) in lines.iter().enumerate() {
        for symbol in symbol_pattern.captures_iter(line) {
            symbols.insert(Point{x: index as isize, y: symbol.get(0).unwrap().start() as isize});
        }
    }

    let number_pattern = Regex::new(r"(\d+)").unwrap();

    let mut gears: HashMap<Point, Vec<u32>> = HashMap::<Point, Vec<u32>>::new();

    for (index, line) in lines.iter().enumerate() {
        for number in number_pattern.captures_iter(line) {
            match get_adjacent(&symbols, index as isize, number.get(0).unwrap()) {
                Some((p,value)) => {
                    gears.entry(p).and_modify(|v| {v.push(value)}).or_insert(vec![value]);
                }
                None => {}
            }
        }
    }

    for (_, v) in gears {
        if v.len() > 1  {
            let product: u32 = v.iter().fold(1, |acc, value| acc * value);
            answer += product;
        }
    }

    answer
}

fn get_adjacent(symbols: &HashSet<Point>, x: isize, m: Match) -> Option<(Point, u32)> {
    for j in (x-1)..=(x+1) {
        for k in (m.start() as isize - 1)..=(m.start() + m.len()) as isize {
            if symbols.contains(&Point{x: j, y: k}) {
                return Some((Point{x: j, y: k}, m.as_str().parse::<u32>().unwrap()))
            }
        }
    }
    None
}