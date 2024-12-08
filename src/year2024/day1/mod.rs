use std::collections::HashMap;

pub fn part1(input: String) -> String {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();
    for line in input.lines() {
        let mut numbers = line.split_whitespace();
        list1.push(numbers.next().unwrap().parse::<u32>().unwrap());
        list2.push(numbers.next().unwrap().parse::<u32>().unwrap());
    };
    list1.sort();
    list2.sort();
    let mut distance_score: u32 = 0;
    for (first_number, second_number) in list1.iter().zip(list2.iter()) {
        distance_score += std::cmp::max(first_number, second_number) - std::cmp::min(first_number, second_number);
    };
    format!("{}", distance_score)
}

pub fn part2(input: String) -> String {
    let mut list1 = Vec::new();
    let mut list2 = HashMap::new();
    for line in input.lines() {
        let mut numbers = line.split_whitespace();
        list1.push(numbers.next().unwrap().parse::<u32>().unwrap());
        let second_number = numbers.next().unwrap().parse::<u32>().unwrap();
        *list2.entry(second_number).or_insert(0) += 1;
    };
    let mut similarity_score: u32 = 0;
    for number in list1 {
        let duplicates = list2.get(&number).unwrap_or(&0);
        similarity_score += number * duplicates;
    }
    format!("{}", similarity_score)
}
