use std::fs;
use text_io::scan;

static INPUT_FILENAME: &str = "input.txt";
static EXAMPLE_FILENAME: &str = "example.txt";

fn get_filename() -> &'static str {
    if cfg!(debug_assertions) {
        EXAMPLE_FILENAME
    } else {
        INPUT_FILENAME
    }
}

struct Pass {
    min: u32,
    max: u32,
    letter: char,
    password: String,
}

fn load_file() -> Vec<Pass> {
    let contents: String =
        fs::read_to_string(get_filename()).expect("Should have been able to read the file");

    let input_split: Vec<&str> = contents.split("\n").collect();

    input_split
        .into_iter()
        .filter(|s| !s.is_empty()) // Filter out empty lines
        .map(|s| {
            let min: u32;
            let max: u32;
            let letter: char;
            let password: String;

            scan!(s.bytes() => "{}-{} {}: {}", min, max, letter, password);

            Pass {
                min,
                max,
                letter,
                password,
            }
        }) // Parse each string
        .collect()
}

fn part_1(data: &Vec<Pass>) -> u32 {
    let mut output: u32 = 0;

    for record in data {
        let c_count = record
            .password
            .chars()
            .filter(|&c| c == record.letter)
            .count();
        let c_count_u32: u32 = c_count.try_into().unwrap();
        if c_count_u32 >= record.min && c_count_u32 <= record.max {
            output += 1;
        }
    }

    output
}

fn part_2(data: &Vec<Pass>) -> u32 {
    let mut output: u32 = 0;

    for record in data {
        let first_pos = record.min as usize - 1;
        let secord_pos = record.max as usize - 1;
        let chars: Vec<char> = record.password.chars().collect();

        if (chars[first_pos] == record.letter) ^ (chars[secord_pos] == record.letter) {
            output += 1;
        }
    }

    output
}

fn main() {
    let data = load_file();
    let part_1_solution = part_1(&data);
    println!("part 1: {:?}", part_1_solution);

    let part_2_solution = part_2(&data);
    println!("part 2: {:?}", part_2_solution);
}
