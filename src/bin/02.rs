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

    // region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let answer = reader
            .lines()
            .map_while(Result::ok)
            .map(split_line)
            .filter(|x| is_safe_dampened(x, false))
            .count();
        Ok(answer)
    }

    assert_eq!(4, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
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

fn is_safe_dampened(levels: &Vec<i32>, recursed: bool) -> bool {
    let diffs = levels
        .windows(2)
        .map(|pair| pair[1] - pair[0])
        .collect_vec();

    //println!("levels {:?}", levels);
    //println!("diffs {:?}", diffs);
    let most_common_sign = match diffs.iter().map(|x| x.signum()).sum::<i32>().signum() {
        0 => 1,
        x => x,
    };

    let is_invalid = |x: &i32| !(x.abs() <= 3 && x.signum() == most_common_sign);
    let invalids = diffs.iter().positions(is_invalid).collect_vec();

    if invalids.is_empty() {
        return true;
    }

    if recursed {
        return false;
    }

    for invalid in invalids {
        for i in 0..=1 {
            let mut levels_dampened = levels.to_owned();
            let invalid_index = invalid + i; // The diff is a midpoint between 2 indexes
            levels_dampened.remove(invalid_index);
            //println!("Removed index {}", invalid_index);
            if is_safe_dampened(&levels_dampened, true) {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn is_safe_dampened_works() {
        println!("1");
        assert_eq!(is_safe_dampened(&Vec::from([1, 2, 3]), false), true);
        println!("2");
        assert_eq!(is_safe_dampened(&Vec::from([1, 2, 7]), false), true);
        println!("3");
        assert_eq!(is_safe_dampened(&Vec::from([1, 6, 7]), false), true);
        println!("4");
        assert_eq!(is_safe_dampened(&Vec::from([1, 2, 1]), false), true);
        println!("5");
        assert_eq!(is_safe_dampened(&Vec::from([1, 1, 2]), false), true);
        println!("6");
        assert_eq!(is_safe_dampened(&Vec::from([1, 2, 1, 2]), false), false);
        println!("7");
        assert_eq!(is_safe_dampened(&Vec::from([1, 2, 1, 1]), false), false);
        println!("8");
        assert_eq!(
            is_safe_dampened(&Vec::from([20, 23, 19, 16, 15]), false),
            true
        );
        println!("9");
        assert_eq!(
            is_safe_dampened(&Vec::from([70, 74, 75, 77, 79, 82]), false),
            true
        );
        println!("10");
        assert_eq!(
            is_safe_dampened(&Vec::from([54, 51, 54, 55, 57, 58]), false),
            true
        );
    }
}
