use std::fs::read_to_string;

fn day_1_part_1() {
    let mut sum = 0;
    if let Ok(lines) = read_to_string(r"E:\rustApps\advent_of_code\src\cases.txt") {
        for ln in lines.lines() {
            sum += get_calibration(ln);
        }
    }
    print!("{:?}", sum);
}

fn get_calibration(input: &str) -> i32 {
    let mut first_num: i32 = 0;
    let mut second_num: i32 = 0;

    for c in input.chars() {
        if let Ok(value) = (c.to_string()).parse::<i32>() {
            first_num = value;
            break;
        }
    }
    
    for c in input.chars().rev() {
        if let Ok(value) = (c.to_string()).parse::<i32>() {
            second_num = value;
            break;
        }
    }

    format!("{:?}{:?}", first_num, second_num).parse::<i32>().unwrap()
}
