use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Could not read input.txt");

    let output = part2(&input);
    println!("{output}")
}

fn part1(input: &str) -> String {
    let mut out = 0;
    let lines = input.lines();

    for (i, l) in lines.clone().enumerate() {
        let line = format!("{l}.");
        let mut symbol = false;
        let mut value = String::new();

        let chars = line.chars();
        let next_chars = lines.clone().nth(i + 1).unwrap_or("").chars();
        let prev_chars = if i == 0 {
            "".chars()
        } else {
            lines.clone().nth(i - 1).unwrap_or("").chars()
        };

        for (j, char) in chars.clone().enumerate() {
            if char.is_digit(10) {
                value.push(char);

                let before = if j == 0 {
                    false
                } else {
                    is_symbol(chars.clone().nth(j - 1).unwrap_or('.'))
                        || is_symbol(next_chars.clone().nth(j - 1).unwrap_or('.'))
                        || is_symbol(prev_chars.clone().nth(j - 1).unwrap_or('.'))
                };

                if symbol
                    || before
                    || is_symbol(chars.clone().nth(j + 1).unwrap_or('.'))
                    || is_symbol(next_chars.clone().nth(j).unwrap_or('.'))
                    || is_symbol(next_chars.clone().nth(j + 1).unwrap_or('.'))
                    || is_symbol(prev_chars.clone().nth(j).unwrap_or('.'))
                    || is_symbol(prev_chars.clone().nth(j + 1).unwrap_or('.'))
                {
                    symbol = true;
                }

                continue;
            }

            if symbol {
                out += value.parse::<i32>().unwrap();
            }

            symbol = false;
            value = String::new();
        }
    }

    return out.to_string();
}

fn part2(input: &str) -> String {
    let mut out = 0;
    let mut hashmap: HashMap<(usize, usize), [i32; 2]> = HashMap::new();
    let lines: Vec<String> = input.lines().map(|l| format!(".{l}.")).collect();

    for (i, line) in lines.iter().enumerate() {
        let mut symbol = false;
        let mut value = String::new();
        let mut gearpos = (0, 0); // x, y

        let chars = line.chars();
        
        let next_chars = if i >= lines.len() - 1 {
            "".chars()
        } else {
            lines[i + 1].chars()
        };

        let prev_chars = if i == 0 {
            "".chars()
        } else {
            lines[i - 1].chars()
        };

        for (j, char) in chars.clone().enumerate() {
            if char.is_digit(10) {
                value.push(char);

                if symbol
                    || is_gear(
                        chars.clone().nth(j - 1).unwrap_or('.'),
                        (j - 1, i),
                        &mut gearpos,
                    )
                    || is_gear(
                        chars.clone().nth(j + 1).unwrap_or('.'),
                        (j + 1, i),
                        &mut gearpos,
                    )
                    || is_gear(
                        next_chars.clone().nth(j).unwrap_or('.'),
                        (j, i + 1),
                        &mut gearpos,
                    )
                    || is_gear(
                        next_chars.clone().nth(j - 1).unwrap_or('.'),
                        (j - 1, i + 1),
                        &mut gearpos,
                    )
                    || is_gear(
                        next_chars.clone().nth(j + 1).unwrap_or('.'),
                        (j + 1, i + 1),
                        &mut gearpos,
                    )
                    || is_gear(
                        prev_chars.clone().nth(j).unwrap_or('.'),
                        (j, i - 1),
                        &mut gearpos,
                    )
                    || is_gear(
                        prev_chars.clone().nth(j - 1).unwrap_or('.'),
                        (j - 1, i - 1),
                        &mut gearpos,
                    )
                    || is_gear(
                        prev_chars.clone().nth(j + 1).unwrap_or('.'),
                        (j + 1, i - 1),
                        &mut gearpos,
                    )
                {
                    symbol = true;
                }

                continue;
            }

            if symbol {
                let num = value.parse::<i32>().unwrap();

                if hashmap.contains_key(&gearpos) {
                    hashmap.get_mut(&gearpos).unwrap()[1] = num;
                } else {
                    hashmap.insert(gearpos, [num, 0]);
                }
            }

            symbol = false;
            value = String::new();
        }
    }

    hashmap.iter().for_each(|m| out += m.1[0] * m.1[1]);

    return out.to_string();
}

fn is_symbol(c: char) -> bool {
    return c != '.' && !c.is_digit(10);
}

fn is_gear<'a>(c: char, pos: (usize, usize), gear: &'a mut (usize, usize)) -> bool {
    if c == '*' {
        *gear = pos;
        return true;
    }

    return false;
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
