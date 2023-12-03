use std::fs::read_to_string;

fn day_1_part_2() {

    let mut sum = 0;
    if let Ok(lines) = read_to_string(r"E:\rustApps\advent_of_code\src\cases.txt") {
        for ln in lines.lines() {
            sum += get_calibration_precise(ln);
        }
    }
    print!("{:?}", sum);
}

fn get_calibration_precise(input: &str) -> i32 {

    let mut first_num = 0;

    'outer: for i in 0..input.len() {

        if let Ok(value) = (input.chars().nth(i).unwrap().to_string()).parse::<i32>() {
            first_num = value;
            break 'outer;
        }

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