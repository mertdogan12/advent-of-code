use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Could not read input.txt");

    let output = part1(&input);
    println!("{output}")
}

fn part1(input: &str) -> String {
    return String::from(input);
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn part1_test() {
        let input = fs::read_to_string("./test.txt").expect("Could not read input.txt");
        let result = fs::read_to_string("./result.txt").expect("Could not read input.txt");

        let output = part1(&input);

        assert_eq!(result, output)
    }
}
