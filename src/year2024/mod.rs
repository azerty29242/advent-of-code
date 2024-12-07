mod day1;

pub fn solve(day: &str, input: String) {
    println!("Solving part 1");

    let part1 = match day {
        "1" => Ok(day1::part1(input.clone())),
        _ => Err("Day {day} of year 2024 is not solved yet")
    }.unwrap();

    println!("Solving part 2");

    let part2 = match day {
        "1" => Ok(day1::part2(input.clone())),
        _ => Err("Day {day} of year 2024 is not solved yet")
    }.unwrap();

    println!("{part1}");
}
