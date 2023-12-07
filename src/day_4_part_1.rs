fn day_4_part_1() {
    let file = 
        std::fs::read_to_string(r"<Your input file in .txt format>").unwrap();
    println!("{:?}", file.lines().map(|line| {
        line.split(": ")
            .collect::<Vec<_>>()[1]
            .split(" | ")
            .map(|part| {
                part.split(" ")
                    .filter(|f| {
                        f.parse::<u32>().is_ok()
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
        }).map(|split| {
            split[0].iter()
                .filter(|&elem| split[1].contains(elem))
                .fold(0, |acc, _| {
                    if acc != 0 { acc + acc } else { acc + 1 }
                })
        }).sum::<i32>()
    );
}