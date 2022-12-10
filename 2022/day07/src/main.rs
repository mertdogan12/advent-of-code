use std::fs;

use crate::filesystem::FileSystem;

mod filesystem;

fn main() {
    let input: String = fs::read_to_string("input.txt")
        .expect("File input.txt doesn't exist");

    let result = run(input);

    fs::write("output.txt", &result)
        .expect("Couln't write to file output.txt");

    println!("{result}");
}

fn run(input: String) -> String {
    let mut filesystem: FileSystem = FileSystem::new();

    for line in input.lines() {
        let mut args = line.split(' ');
        let first_arg = args.next().unwrap();

        match first_arg {
            "$" => run_command(args.next().unwrap(), args.next().unwrap_or(""), &mut filesystem),
            "dir" => filesystem.create_dir(args.next().unwrap()),
            _ => {
                filesystem.create_file(
                    first_arg.parse().unwrap_or_else(|_| panic!("Couln not parse int: {first_arg}")), 
                    args.next().unwrap()
                );
            },
        }

    }

    fn run_command(cmd: &str, args: &str, filesystem: &mut FileSystem) {
        match cmd {
            "cd" => filesystem.change_dir(args),
            "ls" => return,
            _ => panic!("command not found: {cmd}"),
        }
    }

    filesystem.get_size().to_string()
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
