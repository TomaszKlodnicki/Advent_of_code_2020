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

static EYE_COLOR: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

#[derive(Default, Debug)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

fn fill_data_by_key(key: &str, value: &str, passport: &mut Passport) {
    match key {
        "byr" => passport.byr = Some(value.to_string()),
        "iyr" => passport.iyr = Some(value.to_string()),
        "eyr" => passport.eyr = Some(value.to_string()),
        "hgt" => passport.hgt = Some(value.to_string()),
        "hcl" => passport.hcl = Some(value.to_string()),
        "ecl" => passport.ecl = Some(value.to_string()),
        "pid" => passport.pid = Some(value.to_string()),
        "cid" => passport.cid = Some(value.to_string()),
        _ => panic!("unrecognized key {}", key),
    };
}

fn load_file() -> Vec<Passport> {
    let contents: String =
        fs::read_to_string(get_filename()).expect("Should have been able to read the file");

    let data: Vec<Passport> = contents
        .split("\n\n")
        .filter(|s| !s.is_empty())
        .map(|s| {
            let mut passport: Passport = Passport::default();
            let passport_record: Vec<&str> = s
                .split(|c| c == ' ' || c == '\n')
                .filter(|s| !s.is_empty())
                .collect();
            for pass_rec in passport_record {
                let field_record: Vec<&str> = pass_rec.split(':').collect();
                fill_data_by_key(field_record[0], field_record[1], &mut passport);
            }
            passport
        })
        .collect();

    data
}

fn part_1(data: &Vec<Passport>) -> u32 {
    let mut output: u32 = 0;

    for pass in data {
        if pass.byr.is_some()
            && pass.iyr.is_some()
            && pass.eyr.is_some()
            && pass.hgt.is_some()
            && pass.hcl.is_some()
            && pass.ecl.is_some()
            && pass.pid.is_some()
        {
            output += 1;
        }
    }

    output
}

fn part_2(data: &Vec<Passport>) -> u32 {
    let mut output: u32 = 0;

    for pass in data {
        if pass
            .byr
            .as_ref()
            .and_then(|x| x.parse::<u32>().ok())
            .map_or(false, |value| value >= 1920 && value <= 2002)
            && pass
                .iyr
                .as_ref()
                .and_then(|x| x.parse::<u32>().ok())
                .map_or(false, |value| value >= 2010 && value <= 2020)
            && pass
                .eyr
                .as_ref()
                .and_then(|x| x.parse::<u32>().ok())
                .map_or(false, |value| value >= 2020 && value <= 2030)
            && pass.hgt.as_ref().map_or(false, |x| {
                let (begin, last) = x.split_at(x.len() - 2);
                match last {
                    "cm" => begin
                        .parse::<u32>()
                        .map_or(false, |value| value >= 150 && value <= 193),
                    "in" => begin
                        .parse::<u32>()
                        .map_or(false, |value| value >= 59 && value <= 76),
                    _ => false,
                }
            })
            && pass.hcl.as_ref().map_or(false, |x| {
                let (begin, last) = x.split_at(1);
                match begin {
                    "#" => u32::from_str_radix(last, 16).is_ok(),
                    _ => false,
                }
            })
            && pass
                .ecl
                .as_deref()
                .map_or(false, |x| EYE_COLOR.contains(&x))
            && pass
                .pid
                .as_ref()
                .map_or(false, |x| x.len() == 9 && x.parse::<u32>().is_ok())
        {
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
