use std::{fs, ops::Range};

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Could not read input.txt");

    let output = part2(&input);
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

#[derive(Clone, Debug)]
struct Category {
    source: Range<isize>,
    destination: Range<isize>,
}

fn part2(input: &str) -> String {
    let mut maps_inp: Vec<Vec<Category>> = Vec::new();
    let mut lines = input.lines();
    let seeds_nums: Vec<isize> = lines
        .next()
        .unwrap()
        .split(": ")
        .last()
        .unwrap()
        .split(' ')
        .map(|m| m.parse::<isize>().unwrap())
        .collect();

    let seeds: Vec<Range<isize>> = seeds_nums.chunks(2).map(|m| (m[0]..m[0] + m[1])).collect();

    lines.next();

    let mut map: Vec<Category> = Vec::new();
    for line in lines {
        if line.is_empty() {
            maps_inp.push(map.clone());
            map.clear();
            continue;
        }

        if line.ends_with(':') {
            continue;
        }

        let mut nums = line
            .split(' ')
            .map(|m| m.parse::<isize>().unwrap())
            .collect::<Vec<isize>>()
            .into_iter();

        let dest = nums.next().unwrap();
        let source = nums.next().unwrap();
        let offset = nums.next().unwrap();

        map.push(Category {
            source: (source..source + offset),
            destination: (dest..dest + offset),
        });
    }

    maps_inp.push(map.clone());
    let maps = &maps_inp;

    let mut out: Vec<Range<isize>> = Vec::new();
    seeds.iter().for_each(|s| {
        let mut new_seeds: Vec<Range<isize>> = Vec::new();
        new_seeds.push(s.clone());

        for map in maps.clone() {
            let tmp = new_seeds.clone();
            dbg!(new_seeds.clone());
            new_seeds.clear();
            let mut found = false;

            for s in tmp.clone() {
                let mut leftover_seeds: Vec<Range<isize>> = Vec::new();
                leftover_seeds.clear();
                leftover_seeds.push(s.clone());

                dbg!(map.clone());

                for category in map.clone() {
                    dbg!(leftover_seeds.clone());
                    dbg!(category.clone());

                    for seed in leftover_seeds.clone() {
                        let diff = category.destination.start - category.source.start;
                        let start: isize;
                        let end: isize;

                        dbg!(category.source.clone());
                        dbg!(seed.clone());

                        if category.source.end < seed.start || category.source.start > seed.end {
                            println!("nope");
                            println!("nope");
                            continue;
                        } else {
                            leftover_seeds.clear();
                            found = true;
                        }

                        if category.source.start <= seed.start {
                            start = seed.start;
                        } else {
                            start = category.source.start;
                            leftover_seeds.push(seed.start..category.source.start);
                        }

                        if category.source.end < seed.end {
                            end = category.source.end;
                            leftover_seeds.push(category.source.end..seed.end);
                        } else {
                            end = seed.end;
                        }

                        dbg!(diff);
                        leftover_seeds.push(start + diff..end + diff);
                    }
                    println!("{:?}", leftover_seeds.len());

                    if found {
                        new_seeds.append(&mut leftover_seeds);
                    }
                }

                if !found {
                    new_seeds.push(s.clone());
                }

                dbg!(new_seeds.clone());
                println!();
            }

            drop(tmp);
        }

        out.append(&mut new_seeds);
    });

    let min = out.iter().map(|m| m.start).min().unwrap();

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

    #[test]
    fn part2_test() {
        let input = fs::read_to_string("./test.txt").expect("Could not read test.txt");
        let result = fs::read_to_string("./result2.txt")
            .expect("Could not read result2.txt")
            .replace('\n', "");

        let output = part2(&input);

        assert_eq!(result, output)
    }
}
