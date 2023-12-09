fn day_5_part_1() {

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
    let mut result_vec: Vec<i64> = Vec::new();

    // there are 7 maps of which the conditions are same
    // the result of each maps is passed to the next map
    // meanwhile the value of location gets updated
    for seed in seeds {
        let mut location = seed;
        for rotation in 1..=7 {
            location = map_entities(location, input_str[rotation]);
        }
        result_vec.push(location);
    }

    // find minimum value in the vec
    let min_location = result_vec.iter().min().unwrap();
    println!("{:?}", min_location);
}

// processed the seed with the map
// using traditional loops and ".contains()" approach
// took me like 30mins to finish the execution
// This is the optimized way and the execution done in 3.5ms
// This uses the concept of scaling. it finds the difference between
// the source_start_range and dest_start_range and add the diff to the seed
// if the seed value exists within the range.
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
