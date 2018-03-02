extern crate rand;
extern crate time;
extern crate clap;

use rand::Rng;
use time::PreciseTime;
use clap::{Arg, App};

use std::fs::File;
use std::io::{BufRead,BufReader};

fn compute(heights: Vec<i32>, len: usize) -> i32 {
    let mut left_max = 0;
    let mut right_max = 0;
    let mut start = 0;
    let mut end = len-1;
    let mut result = 0;

    while start <= end {
        if heights[start] <= heights[end] {
            if heights[start] > left_max {
                left_max = heights[start];
            } else {
                result += left_max - heights[start];
            }
            start += 1;
        } else {
            if heights[end] > right_max {
                right_max = heights[end];
            } else {
                result += right_max - heights[end];
            }
            end -= 1;
        }
    }

    result
}

fn get_random_heights(total_bins: usize, max_bin_height: i32) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut heights: Vec<i32> = Vec::new();

    for _ in 0..total_bins {
        heights.push(rng.gen_range(0, max_bin_height));
    }

    heights
}

fn get_heights_from_input_file(inp_file: String) -> Vec<i32> {
    let mut heights: Vec<i32> = Vec::new();

    /*
    let file = File::open(inp_file);
    for line in BufReader::new(file).lines() {
        let num: i32 = line.parse().unwrap();
        heights.push(num);
    }
    */
    let file = File::open(inp_file).unwrap();
    for line in BufReader::new(file).lines() {
        heights.push(line.unwrap().trim().parse::<i32>().unwrap());
    }

    heights
}

fn main() {
    //let heights: Vec<i32> = vec![6, 4, 2, 4, 1, 0, 5]; // 14
    //let heights: Vec<i32> = vec![26, 40, 11, 2, 34, 24, 30, 47, 15, 42, 36, 35, 42, 26, 24, 47, 42, 4, 31, 10, 0, 3, 22, 35, 22, 41, 24, 28, 15, 34, 20, 36, 14, 17, 42, 3, 26, 6, 39, 15, 12, 36, 38, 27, 22, 17, 20, 9]; // 702
    
    let matches = App::new("Water Trap Calculator")
                    .arg(Arg::with_name("inp")
                         .short("i")
                         .long("input")
                         .help("Input file with bins")
                         .takes_value(true))
                    .get_matches();

    let inp_file = matches.value_of("inp").unwrap_or("");

    let heights: Vec<i32>;
    let mut total_bins: usize = 1000000;
    let max_bin_height = 1000;

    if inp_file == "" {
        println!("No input file passed, using a random heights vector of size = {}", total_bins);
        heights = get_random_heights(total_bins, max_bin_height);
    } else {
        heights = get_heights_from_input_file(inp_file.to_string());
        total_bins = heights.len();
    }
    
    // Generate heights

    let start = PreciseTime::now();
    let result = compute(heights, total_bins);
    let end = PreciseTime::now();

    if result < 0 {
        println!("Most likely integer overflow!");
    }
    println!("Bins = {0}, Result = {1}, Time taken = {2}", total_bins, result, start.to(end));
}
