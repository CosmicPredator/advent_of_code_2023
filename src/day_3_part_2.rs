use std::fs::read_to_string;

fn day_3_part_2() {
    let data_str: String = read_to_string(r"<Your input file in .txt format>").unwrap();

    // create a 2d array
    let data_vec: Vec<Vec<char>> = data_str.lines().map(|f| {
        f.chars().collect::<Vec<char>>()
    }).collect();

    let pos_offsets = vec![
        (-1, 0), // up
        (0, -1), // left
        (0, 1), // right
        (1, 0), // bottom
        (-1, 1), // top right
        (-1, -1), // top left
        (1, -1), //bottom left
        (1, 1), // bottom right
    ];

    let mut sum = 0;


    for (line_pos, line) in data_vec.iter().enumerate() {
        for (char_pos, c) in line.iter().enumerate() {
            if c.is_symbol() {
                let mut result_vec: Vec<i32> = Vec::new();
                for offset in pos_offsets.iter() {
                    // applying offsets
                    if data_vec[(line_pos as i32 + offset.0) as usize][(char_pos as i32 + offset.1) as usize].is_numeric() {
                        let x = (line_pos as i32 + offset.0) as usize;
                        let y = (char_pos as i32 + offset.1) as usize;

                        // left traverse
                        if !data_vec[x][y + 1].is_numeric() {
                            let x = x;
                            let mut y = y;
                            let mut res: String = String::new();
                            while data_vec[x][y].is_numeric() {
                                res.push(data_vec[x][y]);
                                if y == 0 {break;}
                                y -= 1;
                            }
                            result_vec.push(res.parse::<i32>().unwrap());
                        }

                        // right traverse
                        if !data_vec[x][y - 1].is_numeric() {
                            let x = x;
                            let mut y = y;
                            let mut res: String = String::new();
                            while data_vec[x][y].is_numeric() {
                                res.push(data_vec[x][y]);
                                if y < data_vec[x].len() - 1 {break;}
                                y += 1;
                            }
                            result_vec.push(res.parse::<i32>().unwrap());
                        }
                    }
                    if result_vec.len() > 1 {
                        sum += result_vec.iter().sum::<i32>();
                    }
                }
            }
        }
    }

    println!("{:?}", sum);
}

trait IsSymbol {
    fn is_symbol(&self) -> bool;
}

impl IsSymbol for char {
    fn is_symbol(&self) -> bool {
        *self != '.' && !(*self).is_numeric()
    }
}