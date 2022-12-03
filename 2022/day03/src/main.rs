use std::fs;

fn main() {
    let input: String = fs::read_to_string("input.txt")
        .expect("File input.txt doesn't exist");

    let result = run(input);

    fs::write("output.txt", &result)
        .expect("Couln't write to file output.txt");

    println!("{result}");
}

fn run(input: String) -> String {
    let mut out: u32 = 0;

    for line in input.lines() {
        let (fist_half, second_half) = line.split_at(line.len() / 2);

        assert!(fist_half.len() == second_half.len());

        for c in fist_half.chars() {
            if second_half.contains(c) {
                out += get_priority(c);
                break;
            }
        }
    }

    out.to_string()
}

fn get_priority(c: char) -> u32 {
    let alphabet: String = String::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ");

    let out: usize = alphabet.find(c)
        .unwrap_or_else(|| panic!("Couln not find {c} in {alphabet}"));

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
