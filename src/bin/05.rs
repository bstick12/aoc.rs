use std::collections::BTreeMap;

fn main() -> Result<(), Box<dyn std::error::Error>>  {

    let lines = aoc2023::load_input_data(2023,5)?;
    //let lines: Vec<String> = aoc2023::load_sample_data("/Users/bnolan/workspace/bstick12/aoc.rs/aoc2023/src/resources/day05_sample.txt")?;

    println!("Answer part 1 - {}", part1(&lines));
    println!("Answer part 2 - {}", part2(&lines));
    Ok(())

}


#[derive(Debug, Hash, Eq, PartialEq)]
struct Range {
    destination: u64,
    range: u64,
}

fn part1(lines: &Vec<String>) -> u64 {

    let seedline: Vec<&str> = lines.get(0).unwrap().split(":").collect();
    let seeds: Vec<u64> = seedline.get(1).unwrap().trim().split(" ").map(|v| v.parse::<u64>().unwrap()).collect();

    let mut maps: Vec<BTreeMap<u64, Range>> = Vec::new();
    let mut map: BTreeMap<u64, Range> = BTreeMap::new();

    let mut i = 3;
    while i < lines.len() {
        if lines.get(i).unwrap().is_empty() {
            maps.push(map);
            i+=1;
            map = BTreeMap::new();
        } else {
            let dsr:Vec<u64> = lines.get(i).unwrap().trim().split(" ").map(|v| v.parse::<u64>().unwrap()).collect();
            map.insert(*dsr.get(1).unwrap(), Range{destination: *dsr.get(0).unwrap(), range: *dsr.get(2).unwrap()});
        }
        i+=1;
    }
    maps.push(map);

    let mut location: u64 = u64::MAX;
    for seed in seeds {
        let mut lookup = seed;
        for entry in &maps {
            let irn =  entry.range(..lookup).next_back();
            match irn {
                Some(blah) => {
                    if blah.0 + blah.1.range >= lookup {
                        lookup = (lookup - blah.0) + blah.1.destination;
                    }
                }
                None => {}
            }
        }
        location = location.min(lookup);
    }

    location

}

fn part2(lines: &Vec<String>) -> u64 {

    let seedline: Vec<&str> = lines.get(0).unwrap().split(":").collect();
    let seeds: Vec<u64> = seedline.get(1).unwrap().trim().split(" ").map(|v| v.parse::<u64>().unwrap()).collect();

    let mut maps: Vec<BTreeMap<u64, Range>> = Vec::new();
    let mut map: BTreeMap<u64, Range> = BTreeMap::new();

    let mut i = 3;
    while i < lines.len() {
        if lines.get(i).unwrap().is_empty() {
            maps.push(map);
            i+=1;
            map = BTreeMap::new();
        } else {
            let dsr:Vec<u64> = lines.get(i).unwrap().trim().split(" ").map(|v| v.parse::<u64>().unwrap()).collect();
            map.insert(*dsr.get(1).unwrap(), Range{destination: *dsr.get(0).unwrap(), range: *dsr.get(2).unwrap()});
        }
        i+=1;
    }
    maps.push(map);

    let mut location: u64 = u64::MAX;
    for j in (0..seeds.len()).step_by(2) {
        for k in *seeds.get(j).unwrap()..*seeds.get(j).unwrap() + *seeds.get(j+1).unwrap() {
            let mut lookup: u64 = k;
            for entry in &maps {
                let irn =  entry.range(..=lookup).next_back();
                match irn {
                    Some(rn) => {
                        if rn.0 + rn.1.range >= lookup {
                            lookup = (lookup - rn.0) + rn.1.destination;
                        }
                    }
                    None => {}
                }
            }
            location = location.min(lookup);
        }
    }

    location

}

