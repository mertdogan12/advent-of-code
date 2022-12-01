use std::fs;

fn main() {
    let input: String = fs::read_to_string("input.txt").expect("File input.txt doesn't exist");

    let result = run(input);

    fs::write("output.txt", result).expect("Couln't write to file output.txt");

    println!("Output in file output.txt");
}

fn run(input: String) -> String {
    let mut vec: Vec<u32> = Vec::new();
    let mut tmp: u32 = 0;

    for line in input.lines() {
        if line.replace(" ", "").is_empty() {
            vec.push(tmp);
            tmp = 0;

            continue;
        }

        let inp: u32 = line
            .parse::<u32>()
            .unwrap_or_else(|_| panic!("Couldn't parse input string: {line}"));

        tmp += inp;
    }

    vec.push(tmp);

    vec.sort_by(|a, b| b.cmp(a));

    (vec[0] + vec[1] + vec[2]).to_string()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::run;

    #[test]
    fn run_test() {
        let input: String =
            fs::read_to_string("test-input.txt").expect("File test-input.txt doesn't exist");

        let expected_output: String = fs::read_to_string("test-output.txt")
            .expect("File test-output.txt doesn't exist")
            .replace("\n", "");

        let result = run(input);

        assert_eq!(result, expected_output);
    }
}
