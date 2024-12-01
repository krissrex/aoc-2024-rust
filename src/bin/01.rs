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

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<u32> {
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

        let answer = calculate_similarity(left_items, right_items);

        Ok(answer)
    }

    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
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

fn calculate_similarity(left_items: BinaryHeap<u32>, right_items: BinaryHeap<u32>) -> u32 {
    let left = left_items.into_sorted_vec().into_iter();
    let mut right = right_items.into_sorted_vec().into_iter();

    let mut last_left_num = 0u32;
    let mut right_num = right.next().unwrap();
    let mut last_right_count = 1;
    let mut right_count = 1;

    let mut result = 0;

    'outer: for next_left in left {
        // Catch up left and right
        while next_left > right_num {
            right_num = match right.next() {
                None => {break 'outer }
                Some(x) => x
            };
        }

        // Add up duplicates in left
        if last_left_num == next_left {
            result += last_left_num * last_right_count;
            continue 'outer;
        }
        last_left_num = next_left;

        // Catch up left
        if right_num > next_left {
            continue 'outer;
        }

        // Count up right
        while let Some(next_right) = right.next() {
            if next_right != right_num {
                result += last_left_num * right_count;

                right_num = next_right;
                last_right_count = right_count;
                right_count = 1;
                break;
            }
            right_count += 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_to_pair_works() {
        let actual = split_to_pair(String::from("1   2"));
        assert_eq!(actual, (1, 2))
    }
}
