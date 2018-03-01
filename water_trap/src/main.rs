extern crate rand;
extern crate time;

use rand::Rng;
use time::PreciseTime;

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

fn main() {
    //let heights: Vec<i32> = vec![6, 4, 2, 4, 1, 0, 5]; // 14
    //let heights: Vec<i32> = vec![26, 40, 11, 2, 34, 24, 30, 47, 15, 42, 36, 35, 42, 26, 24, 47, 42, 4, 31, 10, 0, 3, 22, 35, 22, 41, 24, 28, 15, 34, 20, 36, 14, 17, 42, 3, 26, 6, 39, 15, 12, 36, 38, 27, 22, 17, 20, 9]; // 702
    
    // Generate heights
    let mut rng = rand::thread_rng();

    let mut heights: Vec<i32> = Vec::new();
    let total_bins = 1000000;
    let max_bin_height = 1000;

    for _ in 0..total_bins {
        heights.push(rng.gen_range(0, max_bin_height));
    }

    let start = PreciseTime::now();
    let result = compute(heights, total_bins);
    let end = PreciseTime::now();

    if result < 0 {
        println!("Most likely integer overflow!");
    }
    println!("Bins = {0}, Result = {1}, Time taken = {2}", total_bins, result, start.to(end));
}
