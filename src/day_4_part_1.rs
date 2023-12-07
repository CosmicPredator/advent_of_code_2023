fn day_4_part_1() {
    let file = 
        std::fs::read_to_string(r"<Your input file in .txt format>").unwrap();

    // the below iterator pattern does the following
    // line 13 - 23 processes the string and checking for valid u32
    // and then populate to a vec
    // line 24 - 30 checks for winning number from my cards
    // then uses the fold function that appends the accumulator
    // every time a match is found.
    // then the iterator is summed.
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