use std::collections::BinaryHeap;
use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<u32> {
        // TODO: Solve Part 1 of the puzzle

        let mut left_items = BinaryHeap::new();
        let mut right_items = BinaryHeap::new();

        reader
            .lines()
            .map_while(Result::ok)
            .map(split_to_pair)
            //.inspect(|x| println!("{x:?}"))
            .for_each(|(left, right)| {
                left_items.push(left);
                right_items.push(right);
            });

        let answer = calculate_diff(left_items, right_items);

        Ok(answer)
    }

    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    println!("=== Part 1 - reading file ===");
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

fn split_to_pair(line: String) -> (u32, u32) {
    let items = line.split_ascii_whitespace().collect::<Vec<_>>();
    (
        items[0].parse::<u32>().unwrap(),
        items[1].parse::<u32>().unwrap(),
    )
}

fn calculate_diff(left_items: BinaryHeap<u32>, right_items: BinaryHeap<u32>) -> u32 {
    left_items.into_sorted_vec()
        .iter().zip(right_items.into_sorted_vec())
        .map(|(left, right)| {
            left.abs_diff(right)
        })
        .fold(0u32, |acc, add| {
            acc + add
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_to_pair_works() {
        let actual = split_to_pair(String::from("1   2"));
        assert_eq!(actual, (1, 2))
    }

    #[test]
    fn calculate_diff_works() {
        let left = BinaryHeap::from([3, 2, 1]);
        let right = BinaryHeap::from([4, 5, 3]);

        let actual = calculate_diff(left, right);
        assert_eq!(actual, 6)
    }
}
