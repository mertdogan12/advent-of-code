use std::{fs, collections::LinkedList};

fn main() {
    let input: String = fs::read_to_string("input.txt")
        .expect("File input.txt doesn't exist");

    let result = run(input);

    fs::write("output.txt", &result)
        .expect("Couln't write to file output.txt");

    println!("{result}");
}

fn run(input: String) -> String {
    let bytes: &[u8] = input.as_bytes();
    let mut last_four: LinkedList<char> = LinkedList::new();

    for i in 0..bytes.len() {
        let b: char = bytes[i] as char;

        last_four.push_back(b);

        if last_four.len() < 4 {
            continue;
        }

        if last_four.len() > 4 {
           last_four.pop_front(); 
        }

        assert!(last_four.len() == 4);

        if check(&last_four) {
            return (i + 1).to_string();
        }
    }

    panic!("NO OUTPUT FOUND");
}

fn check(inp: &LinkedList<char>) -> bool {
    let mut list: LinkedList<char> = inp.clone();

    while !list.is_empty() {
        let c: char = list.pop_back().unwrap();

        if list.contains(&c) {
            return false;
        }
    }

    true
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
