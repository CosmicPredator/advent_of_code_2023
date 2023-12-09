fn day_6_part_1() {

    // Add input file
    let input = include_str!("cases_day6.txt");

    // Split function to convert the input string to
    // Vec<Vec<i32>>, for example, the sample input given is
    // converted into [[7, 15, 30], [9, 40, 200]]
    let split = input
        .split("\r\n")
        .collect::<Vec<_>>()
        .iter()
        .map(|f| {
            let mut pred = f.split(":")
             .map(|f| f.trim())
             .collect::<Vec<_>>();
            pred.remove(0);
            pred
        }).collect::<Vec<_>>()
        .iter().map(|f| {
            f[0].split(" ")
                .filter_map(|f| f.parse::<i32>().ok())
                .collect::<Vec<_>>()
        }).collect::<Vec<_>>();

    // getting time and distance vec
    let time = split[0].clone();
    let dist = split[1].clone();

    // implement loop logic to calculate ways
    let mut result = 1;
    for (time, distance) in time.iter().zip(dist) {
        let mut chances = 0;
        for i in 0..*time {
            if (time - i) * i > distance {
                chances += 1;
            }
        }
        result *= chances;
    }
    println!("{:?}", result);
}
