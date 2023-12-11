use std::collections::{HashMap, HashSet};
use aoc2023::Point;
use colored::*;

fn main() -> Result<(), Box<dyn std::error::Error>>  {

    let lines = aoc2023::load_input_data(2023,10)?;
    //let lines: Vec<String> = aoc2023::load_sample_data("/Users/bnolan/workspace/bstick12/aoc.rs/aoc2023/src/resources/day10_sample.txt")?;

    solve(&lines);
    Ok(())

}

fn solve(lines: &Vec<String>) {

    let mut grid: Vec<Vec<char>> = Vec::new();

    let mut start: Point = Point::new(0, 0);

    for (y, line) in lines.iter().enumerate() {
        let mut grid_line:  Vec<char> = Vec::new();
        for (x, tile) in line.chars().into_iter().enumerate() {
            if tile == 'S' {
                start = Point{x: x as isize,y: y as isize};
                grid_line.push('|');
//                print!("{}", 'F');
            } else {
                grid_line.push(tile);
//                print!("{}", tile);
            }
        }
//        println!("");
        grid.push(grid_line);
    }

    let mut route: HashMap<(char, char), (char, Point)> = HashMap::new();
    route.insert(('|', 'N'), ('N', Point::NORTH));
    route.insert(('|', 'S'), ('S', Point::SOUTH));
    route.insert(('-', 'W'), ('W', Point::WEST));
    route.insert(('-', 'E'), ('E', Point::EAST));
    route.insert(('F', 'N'), ('E', Point::EAST));
    route.insert(('F', 'W'), ('S', Point::SOUTH));
    route.insert(('7', 'N'), ('W', Point::WEST));
    route.insert(('7', 'E'), ('S', Point::SOUTH));
    route.insert(('J', 'S'), ('W', Point::WEST));
    route.insert(('J', 'E'), ('N', Point::NORTH));
    route.insert(('L', 'S'), ('E', Point::EAST));
    route.insert(('L', 'W'), ('N', Point::NORTH));

    let mut direction: char = 'N';
    let mut steps: usize = 0;
    let mut current = start;
    let mut found = false;
    
    let mut track:HashSet<Point> = HashSet::new();
    track.insert(current);
    while !found {
        steps += 1;
        let tile = grid.get(current.y as usize).unwrap().get(current.x as usize).unwrap();
        let next: &(char, Point) = route.get(&(*tile, direction)).unwrap();
        direction = next.0;
        current = current.add(&next.1);
        track.insert(current);
        if steps != 0 && current.eq(&start) {
            found = true;
        } 
    }

    println!("Answer 1 - {}", steps as isize / 2);

    let mut empties: HashSet<Point> = HashSet::new();
    for (y, line) in grid.iter().enumerate() {
        for(x, _) in line.iter().enumerate() {
            let p = Point::new(x as isize, y as isize);
            if !track.contains(&Point::new(x as isize, y as isize)) {
                empties.insert(p);
            }
        }
    }

    for empty in empties {
        grid[empty.y as usize][empty.x as usize] = '.';
    }

    //visualize_grid(&grid);

    // Didn't want to implement the expansion I used in Java Version
    // So looked at reddit and came across https://en.wikipedia.org/wiki/Point_in_polygon
    // and this solution https://gist.github.com/icub3d/0814b8ab7c87d581f02c3bc7248370b8

    // Find the just the dots in the grid
    let dots: Vec<Point> = grid.iter().enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate().filter(|(_, &tile)| tile == '.')
                .map(move |(x, _)| Point::new(x as isize,y as isize))
        }).collect();

    let rights = HashSet::from(['-', 'L', 'F']);
    let lefts = HashSet::from(['-', 'J', '7']);

    let mut inside = HashSet::new();

    for dot in dots {
        let left = track
            .iter()
            .filter(|p| p.x == dot.x && p.y < dot.y && lefts.contains(grid.get(p.y as usize).unwrap().get(p.x as usize).unwrap()))
            .count();
        let right = track
            .iter()
            .filter(|p| p.x == dot.x && p.y < dot.y && rights.contains(grid.get(p.y as usize).unwrap().get(p.x as usize).unwrap()))
            .count();
        // If the number of crossing is odd then we are inside
        if left.min(right) % 2 == 1 {
            inside.insert(dot);
        }
    }

    println!("Answer 2 - {}", inside.len());

    visualize_grid(&grid);

}

pub fn visualize_grid(grid: &Vec<Vec<char>>) {

    for line in grid {
        for tile in line {
            if *tile == '.' {
                print!("{}", tile.to_string().red().bold());
            } else {
                print!("{}", tile.to_string().blue().bold());
            }
        }
        println!("");
    }

}