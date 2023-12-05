use std::fs::read_to_string;

fn day_1_part_1() {

    // Initialize the result sum to 0
    let mut sum = 0;
    if let Ok(lines) = read_to_string(r"<Your input file in .txt format>") {
        for ln in lines.lines() {
            sum += get_calibration(ln);
        }
    }
    print!("{:?}", sum);
}

fn get_calibration(input: &str) -> i32 {

    let mut first_num: i32 = 0;
    let mut second_num: i32 = 0;

    // This code catches the first parseable i32 (can be checked with char.is_numeric() too.)
    // Then it stores that i32 to first_num
    // Then breaks the loop.
    for c in input.chars() {
        if let Ok(value) = (c.to_string()).parse::<i32>() {
            first_num = value;
            break;
        }
    }
    
    // Same iteration check as before, but with reversed list.
    // So that we can get the last i32 parseable char
    for c in input.chars().rev() {
        if let Ok(value) = (c.to_string()).parse::<i32>() {
            second_num = value;
            break;
        }
    }

    // Interoplating both first and last num and converting it to i32
    format!("{:?}{:?}", first_num, second_num).parse::<i32>().unwrap()
}
