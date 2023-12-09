fn day_6_part_2() {

    // Add str from input file
    let input = include_str!("cases_day6.txt");

    // slightly modified string parser
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
                .collect::<String>()
        }).map(|f| {
            f.parse::<u128>().unwrap()
        }).collect::<Vec<_>>();

    // same way finder as part 1
    let mut result = 0;
    for i in 0..split[0] {
        if (split[0] - i) * i > split[1] {
            result += 1;
        }
    }
    println!("{:?}", result);
}
