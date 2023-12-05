// This code sippet aims to "not" modify the give value
// and perform different iterations
// which may seem unoptimized.
// The ultimate goal is to not modify the given input value.
// If u wanted a faster solution, the use methods like "replace"

use std::fs::read_to_string;

pub fn day_1_part_2() {
    let mut sum = 0;
    if let Ok(lines) = read_to_string(r"<Your input file in .txt format>") {
        for ln in lines.lines() {
            sum += get_calibration_precise(ln);
        }
    }
    println!("{:?}", sum);
}

pub fn get_calibration_precise(input: &str) -> i32 {

    let mut first_num = 0;


    // This outer loop is for getting all the possible substrings
    // from the given str
    // The the substring is matched with respcted i32 values
    // The the master iteration ('outer) is broke.
    'outer: for i in 0..input.len() {

        // This part checks for parseable i32 as same as like in part 1
        if let Ok(value) = (input.chars().nth(i).unwrap().to_string()).parse::<i32>() {
            first_num = value;
            break 'outer;
        }

        // This snippet matches the substring with respective i32 values
        // refer word_to_i32() fucntion
        for j in i..=input.len() {
            let substring = &input[i..j];
            let num: i32 = word_to_i32(substring);

            if num != -1 {
                first_num = num;
                break 'outer;
            }
        }
    }

    let mut last_num = 0;

    // Same as the above snippet, but with reverse code flow
    // To get last i32 parseable char
    // The last_num keeps on getting updated till the end of the iteration.
    // Ultimately the last updated value is the last number of the string slice.
    for i in 0..input.len() {
        for j in i..=input.len() {
            let substring = &input[i..j];
            let num: i32 = word_to_i32(substring);

            if num != -1 {
                last_num = num
            }
        }

        if let Ok(value) = (input.chars().nth(i).unwrap().to_string()).parse::<i32>() {
            last_num = value;
        }
    }

    format!("{}{}", first_num, last_num).parse::<i32>().unwrap()

}

// pattern matching to convert word to i32
fn word_to_i32(input: &str) -> i32 {
    let num: i32 = match input {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        "zero" => 0,
        _ => -1   
    };
    num
}