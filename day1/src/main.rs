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

fn load_file() -> Vec<u32> {
    let contents: String =
        fs::read_to_string(get_filename()).expect("Should have been able to read the file");

    let input_split: Vec<&str> = contents.split("\n").collect();

    input_split
        .into_iter()
        .filter(|s| !s.is_empty()) // Filter out empty lines
        .map(|s| s.parse::<u32>()) // Parse each string
        .collect::<Result<Vec<u32>, _>>() // Collect Results
        .expect("Failed to parse numbers") // Handle parse errors
}

fn part_1(data: &Vec<u32>) -> u32 {
    for i in 0..data.len() {
        for j in (i + 1)..data.len() {
            let sum = data[i] + data[j];
            if sum == 2020 {
                return data[i] * data[j];
            }
        }
    }
    panic!("can't find pair of numbers!");
}

fn part_2(data: &Vec<u32>) -> u32 {
    for i in 0..data.len() {
        for j in (i + 1)..data.len() {
            for k in (j + 1)..data.len() {
                let sum = data[i] + data[j] + data[k];
                if sum == 2020 {
                    return data[i] * data[j] * data[k];
                }
            }
        }
    }
    panic!("can't find pair of numbers!");
}

fn main() {
    let data = load_file();
    let part_1_solution = part_1(&data);
    println!("part 1: {:?}", part_1_solution);

    let part_2_solution = part_2(&data);
    println!("part 2: {:?}", part_2_solution);
}
