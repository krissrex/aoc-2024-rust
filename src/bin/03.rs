use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use regex::{Match, Regex};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<i32> {
        let answer = reader
            .lines()
            .map_while(Result::ok)
            .map(|line| {
                find_all_mul(line)
                    .into_iter()
                    .fold(0, |acc, mul| acc + apply_mul(mul))
            })
            .sum();
        Ok(answer)
    }

    // 2*4 + 5*5 + 11*8 + 8*5
    assert_eq!(161, part1(BufReader::new(TEST.as_bytes()))?);

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

fn find_all_mul(line: String) -> Vec<String> {
    let mul_pattern: Regex = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    mul_pattern
        .find_iter(line.as_str())
        .map(|m: Match| m.as_str().to_string())
        .collect_vec()
}

fn apply_mul(mul: String) -> i32 {
    let mul = mul.strip_prefix("mul(").unwrap().strip_suffix(")").unwrap();
    let (left, right) = mul.split_once(",").unwrap();

    left.parse::<i32>().unwrap() * right.parse::<i32>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_all_mul_works() {
        assert_eq!(find_all_mul(String::from("")).is_empty(), true);
        assert_eq!(find_all_mul(String::from("mul (1,2)")).is_empty(), true);
        assert_eq!(find_all_mul(String::from("mul(1,2)")), ["mul(1,2)"]);
        assert_eq!(find_all_mul(String::from("mul(1,2)x")), ["mul(1,2)"]);
        assert_eq!(find_all_mul(String::from("xmul(1,2)")), ["mul(1,2)"]);
        assert_eq!(
            find_all_mul(String::from("mul(1,2)mul(3,4)")),
            ["mul(1,2)", "mul(3,4)"]
        );
    }

    #[test]
    fn apply_mul_works() {
        assert_eq!(apply_mul(String::from("mul(1,3)")), 3);
        assert_eq!(apply_mul(String::from("mul(10,100)")), 1000);
    }
}
