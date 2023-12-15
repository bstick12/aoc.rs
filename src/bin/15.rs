use regex::Regex;
use std::collections::BTreeMap;

fn main() -> Result<(), Box<dyn std::error::Error>>  {

    let lines = aoc2023::load_input_data(2023,15)?;
    //let lines: Vec<String> = aoc2023::load_sample_data("/Users/bnolan/workspace/bstick12/aoc.rs/aoc2023/src/resources/day14_sample.txt")?;

    println!("Answer part 1 - {}", part1(&lines));
    println!("Answer part 2 - {}", part2(&lines));
    Ok(())

}

fn part1(lines: &Vec<String>) -> usize {

    let line = lines.get(0).unwrap();

    let mut total = 0;
    for command in line.split(",") {
        total = total + get_hash(command);
    }

    total

}

fn get_hash(command: &str) -> usize {
    let mut command_hash = 0;
    for c in command.chars() {
        command_hash = command_hash + c as usize;
        command_hash = command_hash * 17;
        command_hash = command_hash % 256;
    }
    command_hash
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Lens<'a> {
    label: &'a str,
    focal: usize,
}

fn part2(lines: &Vec<String>) -> usize {

    let line = lines.get(0).unwrap();

    let pattern = Regex::new(r"[=-]").unwrap();

    let mut boxes: BTreeMap<usize, Vec<Lens>> = BTreeMap::new();

    
    for command in line.split(",") {
        
        let parts: Vec<&str> = pattern.split(command).collect();

        let list = boxes.entry(get_hash(parts.get(0).unwrap()))
            .and_modify(|_v| {})
            .or_insert(vec![]);

        if parts.get(1).unwrap().is_empty() {
            let lens = Lens{
                label: parts.get(0).unwrap(),
                focal: 0
            };
            for (i, item) in list.iter_mut().enumerate() {
                if item.label == lens.label {
                    list.remove(i);
                    break;
                }
            }
        } else {
            let lens = Lens{
                label: parts.get(0).unwrap(),
                focal: parts.get(1).unwrap().parse::<usize>().unwrap()
            };
            let mut remove: isize = -1;
            for (i, item) in list.iter().enumerate() {
                if item.label == lens.label {
                    remove = i as isize;
                    break;
                }
            }
            if remove != -1 {
                list.remove(remove as usize);
                list.insert(remove as usize, lens);
            } else {
                list.push(lens);
            }
        }
    }

    let mut total = 0;

    for (key, value) in boxes {
        let mut box_size = 0;
        for (i, lens) in value.iter().enumerate() {
            box_size = box_size + ((key + 1) * (i + 1) * lens.focal);
        }
        total = total + box_size;
    }

    total

}



