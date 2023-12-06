
fn main() -> Result<(), Box<dyn std::error::Error>>  {

    let lines = aoc2023::load_input_data(2023,6)?;
    println!("Answer part 1 - {}", part1(&lines));
    println!("Answer part 2 - {}", part2(&lines));
    Ok(())

}

fn part1(lines: &Vec<String>) -> usize {
    let times: Vec<usize> = lines.get(0).unwrap().split(":")
        .collect::<Vec<&str>>().get(1).unwrap().split_whitespace().map(|v| v.parse().unwrap()).collect();
    let distances: Vec<usize> = lines.get(1).unwrap().split(":")
        .collect::<Vec<&str>>().get(1).unwrap().split_whitespace().map(|v| v.parse().unwrap()).collect();
    times.iter().zip(distances).map(|(time, distance)| wins(*time, distance)).product()
}

fn part2(lines: &Vec<String>) -> usize {
    let time: usize = lines.get(0).unwrap().split(":")
        .collect::<Vec<&str>>().get(1).unwrap().split_whitespace().collect::<String>().parse().unwrap();
    let distance: usize = lines.get(1).unwrap().split(":")
        .collect::<Vec<&str>>().get(1).unwrap().split_whitespace().collect::<String>().parse().unwrap();
    wins(time, distance)
}


fn wins(time: usize, distance: usize) -> usize {
    let delta = ((time * time - 4 * distance) as f64).sqrt();
    let lo = (time as f64 - delta) / 2.0;
    let hi = (time as f64 + delta) / 2.0;
    (hi.floor() - lo.ceil()) as usize + 1
}
