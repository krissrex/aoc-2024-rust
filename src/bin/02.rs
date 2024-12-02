use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let answer = reader
            .lines()
            .map_while(Result::ok)
            .map(split_line)
            .filter(is_safe)
            .count();
        Ok(answer)
    }

    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    //
    // fn part2<R: BufRead>(reader: R) -> Result<usize> {
    //     Ok(0)
    // }
    //
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion

    Ok(())
}

fn split_line(line: String) -> Vec<i32> {
    line.split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect_vec()
}

fn is_safe(levels: &Vec<i32>) -> bool {
    let diffs = levels
        .windows(2)
        .map(|pair| pair[0] - pair[1])
        .collect_vec();

    let first_sign = diffs[0].signum();
    if first_sign == 0 {
        return false;
    }

    diffs
        .iter()
        .all(|x| x.abs() <= 3 && x.signum() == first_sign)
}

#[cfg(test)]
mod tests {
    use crate::{is_safe, split_line};

    #[test]
    fn split_line_should_work() {
        let actual = split_line(String::from("1 2 3"));
        assert_eq!(actual, [1, 2, 3]);
    }

    #[test]
    fn is_safe_works() {
        assert_eq!(is_safe(&Vec::from([1, 2, 3])), true);
        assert_eq!(is_safe(&Vec::from([1, 2, 7])), false);
        assert_eq!(is_safe(&Vec::from([1, 2, 1])), false);
    }
}
