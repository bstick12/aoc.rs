use std::collections::BTreeSet;
use aoc2023::Point;

fn main() -> Result<(), Box<dyn std::error::Error>>  {

    let lines = aoc2023::load_input_data(2023,11)?;
    //let lines: Vec<String> = aoc2023::load_sample_data("/Users/bnolan/workspace/bstick12/aoc.rs/aoc2023/src/resources/day11_sample.txt")?;

    println!("Answer 1 - {}", solve(&lines, 2));
    println!("Answer 2 - {}", solve(&lines, 1000000));
    Ok(())

}

fn solve(lines: &Vec<String>, expansion: usize) -> isize {

    let mut empty_rows: BTreeSet<usize> = (0..lines.len()).collect();
    let mut empty_cols: BTreeSet<usize> = (0..lines.get(0).unwrap().len()).collect();
    let mut galaxies: Vec<Point> = Vec::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, space) in line.chars().into_iter().enumerate() {
            if space == '#'  {
                galaxies.push(Point::new(x as isize,y as isize));
                empty_rows.remove(&y);
                empty_cols.remove(&x);
            }
        }
    }

    let expanded_galaxies: Vec<Point> =
        galaxies.iter()
        .map(|g| g.add(&Point::new(
            (empty_cols.range(..=g.x as usize).count() as isize) * (expansion - 1) as isize,
            (empty_rows.range(..=g.y as usize).count() as isize) * (expansion - 1) as isize
        )))
        .collect();

    let mut answer: isize = 0;
    for (i, start) in expanded_galaxies.iter().enumerate() {
        for destination in expanded_galaxies.iter().skip(i) {
            answer = answer + start.distance(destination);
        }
    }
    answer

}