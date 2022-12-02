use std::fs;

enum Todo {
    Win,
    Draw,
    Lose
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
    let mut score: u32 = 0;

    for line in input.lines() {
        let opponent: u32 = get_value(line.chars().nth(0)
            .expect("Failed to get first char of input string"));
        let todo: Todo = get_todo(line.chars().nth(2)
            .expect("Failed to get first char of input string"));

        score += play(todo, opponent);
    }
        
    score.to_string()
}

fn play(todo: Todo, opponent: u32) -> u32 {
    let mut score: u32 = 0; 
    let me: u32 = match todo {
        Todo::Lose => hunted_from(opponent),
        Todo::Draw => opponent,
        Todo::Win => hunter_from(opponent),
    };

    score += me;

    if hunted_from(me) == opponent {
        score += 6;
    }

    if me == opponent {
        score += 3;
    }

    score
}

fn hunted_from(inp: u32) -> u32 {
    if inp == 1 {
        return 3;
    }
        
    inp - 1
}

fn hunter_from(inp: u32) -> u32 {
    if inp == 3 {
        return 1;
    }
        
    inp + 1
}

fn get_todo(c: char) -> Todo {
    match c {
        'X' => Todo::Lose,
        'Y' => Todo::Draw,
        'Z' => Todo::Win,
        _ => panic!("Can not get value of {c}"),
    }
}

fn get_value(c: char) -> u32 {
    match c {
        'A' => 1,
        'B' => 2,
        'C' => 3,
        _ => panic!("Can not get value of {c}"),
    }
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
