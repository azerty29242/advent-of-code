mod day1;
mod day2;

pub fn solve(day: &str, input: String) {
    let part1 = match day {
        "1" => Ok(day1::part1(input.clone())),
        "2" => Ok(day2::part1(input.clone())),
        _ => Err("Day {day} of year 2024 is not solved yet")
    }.unwrap();

    let part2 = match day {
        "1" => Ok(day1::part2(input)),
        "2" => Ok(day2::part2(input)),
        _ => Err("Day {day} of year 2024 is not solved yet")
    }.unwrap();

    println!("Solution to part 1 : {}", part1);
    println!("Solution to part 2 : {}", part2)
}
