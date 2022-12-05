use std::fs;

struct Action {
    count: usize,
    from: usize,
    to: usize
}

fn main() {
    let input: String = fs::read_to_string("input.txt")
        .expect("File input.txt doesn't exist");

    let result = run(input);

    fs::write("output.txt", &result)
        .expect("Couln't write to file output.txt");

    println!("{result}");
}

fn run(input: String) -> String {
    let mut stacks: Vec<Vec<char>> = Vec::new();
    let mut actions: Vec<Action> = Vec::new();
    let mut crates: Vec<&str> = Vec::new();
    let mut parse_actions: bool = false;

    for line in input.lines() {
        if line.is_empty() {
            parse_actions = true;
            continue;
        } 

        if !parse_actions {
            crates.push(line);
            continue;
        }

        let mut words: Vec<&str> = line.split(' ').collect();
        let mut action: Action = Action { count: 0, from: 0, to: 0 };

        while !words.is_empty() {
            let value: usize = words.pop()
                .unwrap()
                .parse::<usize>()
                .unwrap_or_else(|_| {
                    panic!("Error while parsing the value");
                });

            match words.pop().unwrap() {
                "to" => action.to = value - 1,
                "from" => action.from = value - 1,
                "move" => action.count = value,
                _ => panic!("Word doesn't match any given word"),
            }
        }

        actions.push(action);
    }

    let bottom: &str = crates.pop()
        .expect("Couln't pop the bottom from the crates");
    let bottom_bytes: &[u8] = bottom.as_bytes();

    for i in 0..bottom_bytes.len() {
        let c: char = bottom_bytes[i] as char;

        if c == ' ' {
            continue;
        }

        stacks.push(Vec::<char>::new());
   
        for line in crates.iter().rev() {
            let cr: char = line.as_bytes()[i] as char;

            if cr == ' ' {
                continue;
            }

            let position: usize = c.to_string().parse::<usize>().unwrap_or_else(|_| {
                panic!("Couln't parse position: {c}");
            });

            stacks[position - 1].push(cr);
        }
    }

    for action in actions.iter() {
        let mut tmp: Vec<char> = Vec::new();

        for _ in 0..action.count {
            let f: char = stacks[action.from].pop().unwrap();
            tmp.push(f);
        }

        for t in tmp.iter().rev() {
            stacks[action.to].push(*t);
        }
    }

    let mut out: String = String::from("");

    for stack in stacks {
        out.push(*stack.last().unwrap());
    }

    out
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::run;

    #[test]
    fn run_test() {
        let input: String = fs::read_to_string("test-input.txt")
            .expect("File test-input.txt doesn't exist");

        let expected_output: String = fs::read_to_string("test-output.txt")
            .expect("File test-output.txt doesn't exist")
            .replace("\n", "");

        let result = run(input);

        assert_eq!(result, expected_output);
    }
}
