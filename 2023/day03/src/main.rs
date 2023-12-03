use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Could not read input.txt");

    let output = part1(&input);
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

fn is_symbol(c: char) -> bool {
    return c != '.' && !c.is_digit(10);
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
}
