use std::fs;

static INPUT_FILENAME: &str = "input.txt";
static EXAMPLE_FILENAME: &str = "example.txt";

struct Slope {
    down: usize,
    right: usize,
}
static SLOPES: [Slope; 5] = [
    Slope { down: 1, right: 1 },
    Slope { down: 1, right: 3 },
    Slope { down: 1, right: 5 },
    Slope { down: 1, right: 7 },
    Slope { down: 2, right: 1 },
];

fn get_filename() -> &'static str {
    if cfg!(debug_assertions) {
        EXAMPLE_FILENAME
    } else {
        INPUT_FILENAME
    }
}

fn load_file() -> Vec<Vec<bool>> {
    let contents: String =
        fs::read_to_string(get_filename()).expect("Should have been able to read the file");

    let data: Vec<Vec<bool>> = contents
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.chars()
                .map(|c| match c {
                    '#' => true,
                    '.' => false,
                    _ => panic!("Unexpected char! {}", c),
                })
                .collect()
        })
        .collect();

    data
}

fn part_1(data: &Vec<Vec<bool>>) -> u32 {
    let mut output: u32 = 0;

    let mut j = 3;
    for i in 1..data.len() {
        if data[i][j % data[0].len()] {
            output += 1;
        }
        j = j + 3;
    }

    output
}

fn part_2(data: &Vec<Vec<bool>>) -> u32 {
    let mut output: u32 = 1;

    for slope in &SLOPES {
        let mut i = slope.down;
        let mut j = slope.right;
        let mut slope_output: u32 = 0;

        while i < data.len() {
            if data[i][j % data[0].len()] {
                slope_output += 1;
            }
            i += slope.down;
            j += slope.right;
        }
        output *= slope_output;
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
