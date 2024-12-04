use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use grid::Grid;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "04";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<u32> {
        let letters = reader
            .lines()
            .map_while(Result::ok)
            .map(|line| line.chars().collect_vec())
            .collect_vec();
        let grid = Grid::from(letters);
        let mut answer = 0;

        for ((x, y), letter) in grid.indexed_iter() {
            if *letter == 'X' {
                //println!("Exploring {x}, {y}");
                answer += explore(&grid, x, y);
            } else {
                continue;
            }
        }

        Ok(answer)
    }

    assert_eq!(18, part1(BufReader::new(TEST.as_bytes()))?);

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

fn explore(grid: &Grid<char>, x: usize, y: usize) -> u32 {
    let mut positions = vec![];
    for xdir in -1..=1 {
        for ydir in -1..=1 {
            if xdir == 0 && ydir == 0 {
                continue;
            }
            let mut search = vec![];
            for i in 1..=3 {
                search.push((x as i32 + xdir * i, y as i32 + ydir * i));
            }
            positions.push(search);
        }
    }

    let (rows, cols) = grid.size();
    let positions = positions
        .into_iter()
        .filter(|pos| {
            let (last_x, last_y) = pos.last().unwrap();
            if *last_x >= cols as i32 {
                return false;
            }
            if *last_x < 0 {
                return false;
            }
            if *last_y >= rows as i32 {
                return false;
            }
            if *last_y < 0 {
                return false;
            }
            true
        })
        .map(|search| {
            search
                .into_iter()
                .map(|(search_x, search_y)| (search_x as usize, search_y as usize))
                .collect_vec()
        })
        .collect_vec();

    //println!("{x}, {y} has valid positions at {positions:?}");

    let mut result = 0;
    for search in positions {
        if grid[search[0]] == 'M' && grid[search[1]] == 'A' && grid[search[2]] == 'S' {
            result += 1;
            //println!("XMAS at {search:?}")
        }
    }

    result
}
