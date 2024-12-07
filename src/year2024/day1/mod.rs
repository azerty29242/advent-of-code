pub fn part1(input: String) -> String {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();
    for line in input.lines() {
        let mut numbers = line.split_whitespace();
        list1.push(numbers.next().unwrap().parse::<i32>().unwrap());
        list2.push(numbers.next().unwrap().parse::<i32>().unwrap());
    }
    list1.sort();
    list2.sort();
    let mut distances = Vec::new();
    for (first_number, second_number) in list1.iter().zip(list2.iter()) {
        distances.push((first_number - second_number).abs());
    }
    format!("{}", distances.iter().sum::<i32>())
}

pub fn part2(input: String) -> String {
    input
}
