fn main() -> Result<(), Box<dyn std::error::Error>>  {

    let lines = aoc2023::load_input_data(2023,9)?;
    println!("Answer part 1 - {}", part1(&lines, true));
    println!("Answer part 2 - {}", part1(&lines, false));
    Ok(())

}

fn part1(lines: &Vec<String>, forward: bool) -> isize {

    let mut answer: isize = 0;
    for line in lines {
        let mut numbers: Vec<isize> = line.split(" ").map(|v| v.parse::<isize>().unwrap()).collect();
        let mut zeros: bool = false;
        let mut stack: Vec<Vec<isize>> = Vec::new();
        let zero: isize = 0;
        while !zeros  {
            zeros = true;
            stack.push(numbers.clone());
            let new_numbers: Vec<isize> = numbers.windows(2).map(|w| w[1] - w[0]).collect();
            zeros &= new_numbers.iter().filter(|x| **x != zero).count() == 0;
            numbers = new_numbers;
        }
        let mut direction_value: isize;
        if forward {
            direction_value = *numbers.last().unwrap();
            while !stack.is_empty() {
                numbers = stack.pop().unwrap();
                direction_value += numbers.last().unwrap();
            }
        } else {
            direction_value = *numbers.first().unwrap();
            while !stack.is_empty() {
                numbers = stack.pop().unwrap();
                direction_value = numbers.first().unwrap() - direction_value;
            }
        }
        answer += direction_value;
    }
    answer
}