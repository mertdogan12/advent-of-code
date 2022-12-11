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
    let mut inp: Vec<Vec<usize>> = Vec::new();
    let mut out: usize = 0;

    for line in input.lines() {
        let mut tmp: Vec<usize> = Vec::new();

        for c in line.chars() {
            tmp.push(c.to_string().parse().unwrap());
        }

        inp.push(tmp);
    }

    for i in 1..inp.len()-1 {
        let v = inp.get(i).unwrap();

        for j in 1..v.len()-1 {
            let val = v.get(j).unwrap();

            let mut top = i;

            for k in 1..=i {
                let k_val = inp
                    .get(i - k).unwrap()
                    .get(j).unwrap();

                if val <= k_val {
                    top = k;
                    break;
                }
            }

            let mut bottom = inp.len() - i - 1;

            for k in 1..(inp.len() - i) {
                let k_val = inp
                    .get(i + k).unwrap()
                    .get(j).unwrap();
                
                if val <= k_val {
                    bottom = k;
                    break;
                }
            }

            let mut left = j;

            for k in 1..=j {
                let k_val = v.get(j - k).unwrap();

                if val <= k_val {
                    left = k;
                    break;
                }
            }

            let mut right = v.len() - j - 1;

            for k in 1..v.len() - j {
                let k_val = v.get(j + k).unwrap();

                if val <= k_val {
                    right = k;
                    break;
                }
            }

            let sum = top * bottom * right * left;
            out = if sum > out {
                sum
            } else {
                out
            };
        }
    }

    out.to_string()
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
