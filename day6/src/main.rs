use std::collections::HashSet;
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

fn load_file() -> Vec<Vec<Vec<char>>> {
    let contents: String =
        fs::read_to_string(get_filename()).expect("Should have been able to read the file");

    let data = contents
        .split("\n\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.split("\n").map(|p| p.chars().collect()).collect())
        .collect();

    data
}

fn part_1(data: &Vec<Vec<Vec<char>>>) -> u32 {
    let mut output: u32 = 0;

    for group in data {
        let mut char_set: HashSet<char> = HashSet::new();
        for person in group {
            for question in person {
                char_set.insert(*question);
            }
        }
        output += char_set.len() as u32;
    }

    output
}

fn part_2(data: &Vec<Vec<Vec<char>>>) -> u32 {
    let mut output: u32 = 0;

    for group in data {
        let mut char_set: HashSet<char> = group[0].iter().cloned().collect();

        for person in &group[0..group.len()] {
            let c_set: HashSet<char> = person.iter().cloned().collect();

            char_set = &char_set & &c_set;
        }

        output += char_set.len() as u32;
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
