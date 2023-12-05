use std::collections::HashMap;
use std::fs::read_to_string;

// define the cube struct
#[derive(Debug)]
struct Cube {
    blue: i32,
    green: i32,
    red: i32,
}

// type alias for simplicity
type Game = Vec<Cube>;

pub fn day_2_part_2() {
    // populating data from cases_day2.rs file to HashMap<u32, Game>
    // Box<T> used if in case the stack gets overflowed
    let mut game_coll: Box<HashMap<u32, Game>> = Box::new(HashMap::new());

    if let Ok(lines) = read_to_string(r"<Your input file in .txt format>") {
        for (e, l) in lines.lines().enumerate() {
            game_coll.insert((e as u32) + 1, str_to_game(l));
        }
    }

    // Initializing empty i32 for resulting sum
    let mut sum: u32 = 0;

    // use Iterator::max_by_key() function to ge max value and multiply it.
    for (_, value) in game_coll.iter() {

        sum += (value.iter().max_by_key(|p|p.blue).unwrap().blue *
        value.iter().max_by_key(|p|p.green).unwrap().green *
        value.iter().max_by_key(|p|p.red).unwrap().red ) as u32;

    }

    println!("{:?}", sum);
}

fn str_to_game(input: &str) -> Game {

    // &str fucntions to process a line from cases_day2.txt to a valid Game(Vec<Cube>) instance.
    // For example, the str "Game 1: 7 blue, 9 red, 1 green; 8 green; 10 green, 5 blue, 3 red; 11 blue, 5 red, 1 green"
    // gets converted to 
    // [Cube { blue: 7, green: 1, red: 9 }, Cube { blue: 0, green: 8, red: 0 }, Cube { blue: 5, green: 10, red: 3 }, Cube { blue: 11, green: 1, red: 5 }]
    let mut game: Game = Vec::new();
    let spilt = input.split(": ").collect::<Vec<_>>();
    let split_again = spilt[1].split("; ");
    let vec_coll = split_again.collect::<Vec<_>>();
    for i in vec_coll.into_iter() {
        let data: Vec<&str> = i.split(", ").collect();
        let mut cube: Cube = Cube {
            blue: 0,
            green: 0,
            red: 0,
        };
        for j in data {
            let actual_data: Vec<&str> = j.split(" ").collect();
            match actual_data[1] {
                "red" => cube.red = actual_data[0].parse::<i32>().unwrap(),
                "blue" => cube.blue = actual_data[0].parse::<i32>().unwrap(),
                "green" => cube.green = actual_data[0].parse::<i32>().unwrap(),
                _ => (),
            }
        }
        game.push(cube);
    }
    game
}
