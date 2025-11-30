use std::cmp;
use std::fs;

static INPUT_FILENAME: &str = "input.txt";
static EXAMPLE_FILENAME: &str = "example.txt";

fn get_filename() -> &'static str {
    if cfg!(debug_assertions) {
        EXAMPLE_FILENAME
    } else {
        INPUT_FILENAME
    }
}

fn load_file() -> Vec<String> {
    let contents: String =
        fs::read_to_string(get_filename()).expect("Should have been able to read the file");

    let data: Vec<String> = contents
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();
    data
}

fn get_id(id_str: &String) -> u32 {
    let (height, width) = id_str.split_at(7);

    let row_str: String = height
        .chars()
        .map(|c| {
            match c {
                'F' => 0,
                'B' => 1,
                _ => panic!("unrecognized char - {}", c),
            }
            .to_string()
        })
        .collect();
    let row = u32::from_str_radix(&row_str, 2).expect("here should be binary value");

    let collumn_str: String = width
        .chars()
        .map(|c| {
            match c {
                'L' => 0,
                'R' => 1,
                _ => panic!("unrecognized char - {}", c),
            }
            .to_string()
        })
        .collect();
    let collumn = u32::from_str_radix(&collumn_str, 2).expect("here should be binary value");

    row * 8 + collumn
}

fn part_1(data: &Vec<String>) -> u32 {
    let mut output: u32 = 0;

    for line in data {
        output = cmp::max(get_id(&line), output);
    }

    output
}

fn part_2(data: &Vec<String>) -> u32 {
    let mut output: Option<u32> = None;

    let max_id = 127 * 8 + 7;
    let mut possible_id: Vec<bool> = vec![true; max_id + 1];

    println!(
        "data_len: {}, possible_id_len: {}",
        data.len(),
        possible_id.len()
    );

    for line in data {
        possible_id[get_id(line) as usize] = false;
    }

    for i in 8..(possible_id.len() - 8) {
        if possible_id[i] && !possible_id[i - 1] && !possible_id[i + 1] {
            match output {
                None => output = Some(i as u32),
                Some(old) => panic!(
                    "found more than one possible id! check input! old: {}, new: {}",
                    old, i as u32
                ),
            };
        }
    }

    output.expect("haven't find any available seat id!");
    output.unwrap()
}

fn main() {
    let data = load_file();
    let part_1_solution = part_1(&data);
    println!("part 1: {:?}", part_1_solution);

    let part_2_solution = part_2(&data);
    println!("part 2: {:?}", part_2_solution);
}
