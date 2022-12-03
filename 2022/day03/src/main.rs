use std::fs;

fn main() {
    let input: String = fs::read_to_string("input.txt")
        .expect("File input.txt doesn't exist");

    let result = run(input);

    fs::write("output.txt", &result)
        .expect("Couln't write to file output.txt");

    println!("{result}");
}

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn run(input: String) -> String {
    let mut out: u32 = 0;
    let mut pos: u8 = 0;
    let mut tmp: Vec<char> = Vec::new();

    for line in input.lines() {
        match pos {
            0 => {
                for c in line.chars() {
                    if !tmp.contains(&c) {
                        tmp.push(c);
                    }
                }
            },
            1 | 2 => {
                let mut tmp_tmp: Vec<char> = Vec::new();

                while !tmp.is_empty() {
                    let c: char = tmp.pop()
                        .expect("Error while poping tmp");

                    if line.contains(c) {
                        tmp_tmp.push(c);
                    }
                }

                tmp = tmp_tmp;
            },
            _ => panic!("Position {pos} is not allowed"),
        }

        pos += 1;

        if pos == 3 {
            assert!(tmp.len() == 1);
            out += get_priority(tmp.pop()
                .expect("Error while poping an element out of tmp"));

            tmp.clear();
            pos = 0;
        }
    }

    out.to_string()
}

fn get_priority(c: char) -> u32 {
    let out: usize = ALPHABET.find(c)
        .unwrap_or_else(|| panic!("Couln not find {c} in {ALPHABET}"));

    (out + 1) as u32
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
