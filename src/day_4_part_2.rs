
fn day_4_part_2() {
    // read the input file here
    let ans: String = std::fs::read_to_string(r"<Your input file in .txt format>").unwrap();
    let ans_vec = ans.lines().collect::<Vec<&str>>();
    
    // initialize two vecs, one for saving original winning card counts
    // other for saving copied winning card counts
    let mut winning: Vec<u32> = vec![0; ans_vec.len()];
    let mut copies: Vec<u32> = vec![1; ans_vec.len()];

    // populate the winning vec
    for (count, line) in ans_vec.iter().enumerate() {
        let (win, mine) = get_winning_games(line);
        for i in mine {
            if win.contains(&i) {
                winning[count] += 1;
            }
        }
    }

    // populate the copied winning cards counts vec
    for i in 0..winning.len() {
        for j in i + 1..=i + winning[i] as usize {
            copies[j] += copies[i] as u32;
        }
    }

    // summ of the counts of copied cards using iterator
    println!("{:?}", copies.iter().sum::<u32>());
}

// helper fucntion to convert &str to (Vec<&str>, Vec<&str>) tuple
// for example the string slice "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"
// will get converted to (vec![41, 48, 83, 86, 17], vec![83, 86, 6, 31, 17, 9, 48, 53])
fn get_winning_games(i: &str) -> (Vec<&str>, Vec<&str>) {
    let splitted_vec = i.split(": ")
        .collect::<Vec<_>>()[1]
        .split(" | ")
        .collect::<Vec<_>>()
        .iter()
        .map(|m| {
            m.split(" ")
                .filter(|f|f.parse::<u32>().is_ok())
                .collect::<Vec<_>>()
        }).collect::<Vec<Vec<_>>>();
    (splitted_vec[0].clone(), splitted_vec[1].clone())
}