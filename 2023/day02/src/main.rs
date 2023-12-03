use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Could not read input.txt");

    let output = part2(&input);
    println!("{output}")
}

const MAX_RED: i32 = 12;
const MAX_GREEN: i32 = 13;
const MAX_BLUE: i32 = 14;

fn part1(input: &str) -> String {
    let mut out = 0;

    for line in input.lines() {
        let inp = line.split(": ");
        let id = inp
            .clone()
            .nth(0)
            .unwrap()
            .split(' ')
            .nth(1)
            .unwrap()
            .replace(':', "")
            .parse::<i32>()
            .unwrap();

        let mut value = id;
        for data in inp.clone().nth(1).unwrap().split("; ") {
            let mut cubes = [0, 0, 0]; // red, green, blue

            for cube in data.split(", ") {
                let amount = cube.split(' ').next().unwrap().parse::<i32>().unwrap();
                let color = cube.split(' ').last().unwrap();

                let index = match color {
                    "red" => 0,
                    "green" => 1,
                    "blue" => 2,
                    _ => panic!("{color} is not a valid color"),
                };

                if amount > cubes[index] {
                    cubes[index] = amount;
                }
            }

            if cubes[0] > MAX_RED || cubes[1] > MAX_GREEN || cubes[2] > MAX_BLUE {
                value = 0;
                break;
            }
        }

        out += value;
    }

    return out.to_string();
}

fn part2(input: &str) -> String {
    let mut out = 0;

    for line in input.lines() {
        let inp = line.split(": ");
        let mut cubes = [-1, -1, -1]; // red, green, blue

        for data in inp.clone().nth(1).unwrap().split("; ") {
            for cube in data.split(", ") {
                let amount = cube.split(' ').next().unwrap().parse::<i32>().unwrap();
                let color = cube.split(' ').last().unwrap();

                let index = match color {
                    "red" => 0,
                    "green" => 1,
                    "blue" => 2,
                    _ => panic!("{color} is not a valid color"),
                };

                if amount > cubes[index] {
                    cubes[index] = amount;
                }
            }

        }

        out += cubes[0] * cubes[1] * cubes[2];
    }

    return out.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part1_test() {
        let input = fs::read_to_string("./test.txt").expect("Could not read input.txt");
        let result = fs::read_to_string("./result.txt")
            .expect("Could not read result.txt")
            .replace('\n', "");

        let output = part1(&input);

        assert_eq!(result, output)
    }

    #[test]
    fn part2_test() {
        let input = fs::read_to_string("./test2.txt").expect("Could not read input2.txt");
        let result = fs::read_to_string("./result2.txt")
            .expect("Could not read result2.txt")
            .replace('\n', "");

        let output = part2(&input);

        assert_eq!(result, output)
    }
}
