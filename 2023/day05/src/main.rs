use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Could not read input.txt");

    let output = part1(&input);
    println!("{output}")
}

fn part1(input: &str) -> String {
    let mut maps: Vec<Vec<[isize; 3]>> = Vec::new();
    let mut lines = input.lines();
    let seeds: Vec<isize> = lines
        .next()
        .unwrap()
        .split(": ")
        .last()
        .unwrap()
        .split(' ')
        .map(|m| m.parse::<isize>().unwrap())
        .collect();

    lines.next();

    let mut map: Vec<[isize; 3]> = Vec::new();
    for line in lines {
        if line.is_empty() {
            maps.push(map.clone());
            map.clear();
            continue;
        }

        if line.ends_with(':') {
            continue;
        }

        let nums: Vec<isize> = line
            .split(' ')
            .map(|m| m.parse::<isize>().unwrap())
            .collect();
        map.push(nums.try_into().unwrap());
    }

    maps.push(map.clone());

    let mut min = -1;
    seeds.iter().for_each(|seed| {
        let mut s = seed.clone();

        for map in maps.clone() {
            for range in map {
                if !(s > range[1] && s < range[1] + range[2]) {
                    continue;
                }

                s += range[0] - range[1];
                break;
            }
        }

        if min == -1 || s < min {
            min = s;
        }
    });

    return min.to_string();
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
