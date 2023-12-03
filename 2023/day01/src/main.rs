use regex::Regex;
use std::fs;

fn main() {
    let input = fs::read_to_string("./input2.txt").expect("Could not read input2.txt");

    let output = part2(&input);
    println!("{output}")
}

fn part1(input: &str) -> String {
    let mut out = 0;

    for line in input.lines() {
        let reg = Regex::new(r"\D").unwrap();
        let res = reg.replace_all(line, "");

        let first = res.chars().next().unwrap();
        let last = res.chars().last().unwrap();

        let out_str = format!("{first}{last}");

        out += out_str.parse::<i32>().unwrap();
    }

    return out.to_string();
}

fn part2(input: &str) -> String {
    let mut out = 0;

    for l in input.lines() {
        let line = l
            .replace("one", "o1e")
            .replace("two", "t2o")
            .replace("three", "th3ee")
            .replace("four", "f4ur")
            .replace("five", "f5ve")
            .replace("six", "s6x")
            .replace("seven", "se7en")
            .replace("eight", "ei8ht")
            .replace("nine", "n9ne");

        let reg = Regex::new(r"\D").unwrap();
        let res = reg.replace_all(line.as_str(), "");

        let first = res.chars().next().unwrap();
        let last = res.chars().last().unwrap();

        let out_str = format!("{first}{last}");

        out += out_str.parse::<i32>().unwrap();
    }

    return out.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part2_test() {
        let input = fs::read_to_string("./test2.txt").expect("Could not read test2.txt");
        let result = fs::read_to_string("./result2.txt")
            .expect("Could not read result2.txt")
            .replace('\n', "");

        let output = part2(&input);

        assert_eq!(&result, &output)
    }
}
