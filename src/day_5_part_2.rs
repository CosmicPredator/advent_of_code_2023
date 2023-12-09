// CAUTION!!! BRUTEFORCE AHEAD

fn day_5_part_2() {

    // read input from file cases_day5.txt
    // split the string into Vec of seeds and maps
    let input_str: Vec<&str> = include_str!("cases_day5.txt")
        .split("\r\n\r\n").collect();

    // split seeds into Vec<i64>
    let seeds: Vec<i64> = input_str[0].split(": ").collect::<Vec<&str>>()[1]
        .split(" ")
        .map(|f| f.parse::<i64>().unwrap())
        .collect();

    // initialize empty vec to store results to find minimum value at the end
    let mut result = 0;

    // there are 7 maps of which the conditions are same
    // the result of each maps is passed to the next map
    // meanwhile the value of location gets updated
    // for part 2, i just added another for loop for chunked ranges
    for i in (0..3).step_by(2) {
        for j in seeds[i]..seeds[i] + seeds[i + 1] {
            let mut location = j;
            for rotation in 1..=7 {
                location = map_entities(location, input_str[rotation]);
            }
            if location < result {
                result = location;
            } else if result == 0 {
                result = location
            }
        }  
    }

    // find minimum value in the vec
    println!("Result: {:?}", result);
}

// processed the seed with the map
fn map_entities(seed: i64, map_str: &str) -> i64 {
    let mut range = map_str.split("\r\n").collect::<Vec<&str>>();
    range.remove(0);
    for entity in range.iter() {
        let processed = entity
            .split(" ")
            .map(|f| f.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        if seed >= processed[1] && seed <= processed[1] + processed[2] {
            let diff = processed[0] - processed[1];
            return diff + seed;
        }
    }
    seed
}
