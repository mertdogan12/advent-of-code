use std::fs;

struct Section(u32, u32);

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
        let sections: Vec<&str> = line.split(",").collect();

        let sec1: Section = string_to_section(sections[0]);
        let sec2: Section = string_to_section(sections[1]);

        if overlaps(&sec1, &sec2) || overlaps(&sec2, &sec1) {
            out += 1;
        }
    }

    out.to_string()
}

fn overlaps(sec1: &Section, sec2: &Section) -> bool {
    (sec2.1 >= sec1.0 && sec2.1 <= sec1.1) ||
        (sec2.0 >= sec1.0 && sec2.0 <= sec1.1)
}

fn string_to_section(inp: &str) -> Section {
    let section: Vec<&str> = inp.split("-").collect();

    let i1: u32 = section[0].parse::<u32>().unwrap_or_else(|_| {
        panic!("Could not parse {:?}", &section[0]);
    });
    let i2: u32 = section[1].parse::<u32>().unwrap_or_else(|_| {
        panic!("Could not parse {:?}", &section[1]);
    });

    Section(i1, i2)
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
