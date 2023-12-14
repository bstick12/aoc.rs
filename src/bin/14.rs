use aoc2023::Point;
use colored::*;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>>  {

    let lines = aoc2023::load_input_data(2023,14)?;
    //let lines: Vec<String> = aoc2023::load_sample_data("/Users/bnolan/workspace/bstick12/aoc.rs/aoc2023/src/resources/day14_sample.txt")?;

    println!("Answer part 1 - {}", part1(&lines));
    println!("Answer part 2 - {}", part2(&lines));
    Ok(())

}

fn part1(lines: &Vec<String>) -> usize {

    let mut grid: Vec<Vec<char>> = vec![vec!['.'; lines.get(0).unwrap().len()]; lines.len()];

    for (y, line) in lines.iter().enumerate() {
        for (x, elem) in line.chars().enumerate() {
            grid[x][y] = elem;
        }
    }

    spin_grid_right_left(&mut grid, Point::NORTH);
    print_2d_array(&grid);

    get_load(&grid) as usize
}

fn part2(lines: &Vec<String>) -> usize {

    let mut grid: Vec<Vec<char>> = vec![vec!['.'; lines.get(0).unwrap().len()]; lines.len()];

    for (y, line) in lines.iter().enumerate() {
        for (x, elem) in line.chars().enumerate() {
            grid[x][y] = elem;
        }
    }

    let mut cache: HashMap<String, (u64,i32)> = HashMap::new();

    let mut i = 0;
    while i < 1_000_000_000  {
        let key = get_key(&grid);
        if cache.contains_key(&key) {
            let entry = cache.get(&key).unwrap();
            let cycle = i - entry.0;
            let idx = entry.0 + (1_000_000_000 - entry.0) % cycle;
            for value in cache.values() {
                if value.0 == idx {
                    return value.1 as usize
                }
            }
            return 0
        } else {
            cache.insert(key, (i, get_load(&grid)));
        }
        spin_grid_right_left(&mut grid, Point::NORTH);
        spin_grid_right_left(&mut grid, Point::WEST);
        spin_grid_left_right(&mut grid, Point::SOUTH);
        spin_grid_left_right(&mut grid, Point::EAST);
        i = i + 1;
    }
    0
}


fn spin_grid_right_left(grid: &mut Vec<Vec<char>>, direction: Point) {

    for y in 0..grid[0].len() {
        for x in 0..grid.len() {
            let mut current = Point::new(x as isize, y as isize);
            if grid[current.x as usize][current.y as usize] == 'O' {
                let mut target: Point = current.add(&direction);
                while get_grid_element(&grid, target, 'O') == '.' {
                    grid[target.x as usize][target.y as usize] = 'O';
                    grid[current.x as usize][current.y as usize] = '.';
                    current = target;
                    target = current.add(&direction);
                }
            }
        }
    }
}

fn spin_grid_left_right(grid: &mut Vec<Vec<char>>, direction: Point) {

    for y in (0..grid[0].len()).rev() {
        for x in (0..grid.len()).rev() {
            let mut current = Point::new(x as isize, y as isize);
            if grid[current.x as usize][current.y as usize] == 'O' {
                let mut target: Point = current.add(&direction);
                while get_grid_element(&grid, target, 'O') == '.' {
                    grid[target.x as usize][target.y as usize] = 'O';
                    grid[current.x as usize][current.y as usize] = '.';
                    current = target;
                    target = current.add(&direction);
                }
            }
        }
    }
}

fn get_grid_element(grid: &Vec<Vec<char>>, target: Point, default:char) -> char {
    if target.x >= 0 && target.x < grid.len() as isize && target.y >= 0 && target.y < grid[0].len() as isize {
        return grid[target.x as usize][target.y as usize]
    }
    default
}

fn get_load(grid: &Vec<Vec<char>>) -> i32 {

    let mut load: i32 = 0;
    for (y, line) in grid.iter().enumerate() {
        for (x, _) in line.iter().enumerate() {
            if grid[x][y] == 'O' {
                load = load + (line.len() - y) as i32;
            }
        }
    }
    load

}

fn print_2d_array(grid: &Vec<Vec<char>>) {

    println!();
    for (y, line) in grid.iter().enumerate() {
        for (x, _) in line.iter().enumerate() {
            match grid[x][y].to_string().as_str() {
                "O" => {
                    print!("{}", "O".to_string().red().bold())
                },
                "#" => {
                    print!("{}", "#".to_string().green())
                },
                _ => print!("{}", grid[x][y]),
            }
        }
        println!();
    }

}

fn get_key(grid: &Vec<Vec<char>>) -> String {

    let mut key: String = String::from("");
    for (y, line) in grid.iter().enumerate() {
        for (x, _) in line.iter().enumerate() {
            key.push_str(grid[x][y].to_string().as_str())
        }
    }
    key
}
