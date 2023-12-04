use std::{fs, vec};

use regex::Regex;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Could not read input.txt");

    let output = part2(&input);
    println!("{output}")
}

fn part1(input: &str) -> String {
    let mut out = 0;

    for l in input.lines() {
        let regex = Regex::new(r"  ").unwrap();
        let line = regex.replace_all(l, " ");

        let mut points = 0;
        let mut numbers = line.split(": ").last().unwrap().split(" | ");

        let winning: Vec<i32> = numbers
            .next()
            .unwrap()
            .split(' ')
            .map(|n| n.parse::<i32>().unwrap())
            .collect();
        let nums: Vec<i32> = numbers
            .next()
            .unwrap()
            .split(' ')
            .map(|n| n.parse::<i32>().unwrap())
            .collect();

        for num in nums {
            if winning.contains(&num) {
                points = if points == 0 { 1 } else { points + points };
            }
        }

        out += points;
    }

    return out.to_string();
}

fn part2(input: &str) -> String {
    let mut out = 0;
    let mut cards = vec![1; input.lines().count()];

    for (i, l) in input.lines().enumerate() {
        let regex = Regex::new(r"  ").unwrap();
        let line = regex.replace_all(l, " ");

        let mut points = 0;
        let mut numbers = line.split(": ").last().unwrap().split(" | ");

        let winning: Vec<i32> = numbers
            .next()
            .unwrap()
            .split(' ')
            .map(|n| n.parse::<i32>().unwrap())
            .collect();
        let nums: Vec<i32> = numbers
            .next()
            .unwrap()
            .split(' ')
            .map(|n| n.parse::<i32>().unwrap())
            .collect();

        for num in nums {
            if winning.contains(&num) {
                points += 1;
            }
        }

        for j in i..points + i {
            cards[j + 1] += 1 * cards[i];
        }
    }

    cards.iter().for_each(|card| out += card);

    return out.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part1_test() {
        let input = fs::read_to_string("./test.txt").expect("Could not read test.txt");
        let result = fs::read_to_string("./result.txt")
            .expect("Could not read result.txt")
            .replace('\n', "");

        let output = part1(&input);

        assert_eq!(result, output)
    }

    #[test]
    fn part2_test() {
        let input = fs::read_to_string("./test.txt").expect("Could not read test.txt");
        let result = fs::read_to_string("./result2.txt")
            .expect("Could not read result2.txt")
            .replace('\n', "");

        let output = part2(&input);

        assert_eq!(result, output)
    }
}
