use chrono::{NaiveDate, Utc};
use clap::Parser;
use date_time_parser::DateParser;
use json::JsonValue;
use random_person::{generate_female, generate_male};
use std::{collections::HashMap, fs::File, io::Read};

#[derive(Parser, Debug)]
#[clap(
    author,
    version,
    about,
    long_about = "Note: When playing, all the keybindings of mpv can be used, and `q` is reserved for exiting the program"
)]
pub struct Cli {
    #[clap(
        help = "The date to start from, in fuzzy format. For example: `today`, `yesterday`, `tomorrow`, `last week`, `next month`, `last year`"
    )]
    date: String,

    #[clap(
        short,
        long,
        help = "Whether to randomize the dates or not",
        default_value = "false"
    )]
    random: bool,

    #[clap(
        short,
        long,
        help = "The sex to generate: 1-Masculine, 2-Feminine. If not present, both will be generated"
    )]
    sex: Option<u8>,
}

fn main() {
    let args = Cli::parse();

    // parse the first argument to a number
    let start_date: NaiveDate =
        match DateParser::parse_relative(&args.date, Utc::now().date_naive()) {
            Some(date) => date,
            None => {
                println!("Invalid date");
                std::process::exit(1);
            }
        };

    let mut buf = String::new();
    File::open("data.json")
        .unwrap()
        .read_to_string(&mut buf)
        .unwrap();
    let json = match json::parse(&buf) {
        Ok(json) => json,
        Err(e) => {
            println!("Error parsing JSON: {}", e);
            std::process::exit(1);
        }
    };

    let male = match &json["male"] {
        JsonValue::Array(arr) => arr
            .into_iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>(),
        _ => panic!("Expected array"),
    };
    let female = match &json["female"] {
        JsonValue::Array(arr) => arr
            .into_iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>(),
        _ => panic!("Expected array"),
    };
    let surnames = match &json["surnames"] {
        JsonValue::Array(arr) => arr
            .into_iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>(),
        _ => panic!("Expected array"),
    };

    let dni_letters: HashMap<u8, &str> = [
        (0, "T"),
        (1, "R"),
        (2, "W"),
        (3, "A"),
        (4, "G"),
        (5, "M"),
        (6, "Y"),
        (7, "F"),
        (8, "P"),
        (9, "D"),
        (10, "X"),
        (11, "B"),
        (12, "N"),
        (13, "J"),
        (14, "Z"),
        (15, "S"),
        (16, "Q"),
        (17, "V"),
        (18, "H"),
        (19, "L"),
        (20, "C"),
        (21, "K"),
        (22, "E"),
    ]
    .iter()
    .cloned()
    .collect();

    let male_categories = vec![18, 35, 40, 45, 50, 55, 60, 65, 70];
    let female_categories = vec![18, 40, 45, 50, 55, 60, 65, 70];

    println!("DNI\tNombre\tApellidos\tFecha de nacimiento\tRango de edad");

    if let Some(sex) = args.sex {
        if sex == 1 {
            generate_male(
                male_categories,
                args.random,
                &dni_letters,
                start_date,
                &male,
                &female,
                &surnames,
            );
        } else if sex == 2 {
            generate_female(
                female_categories,
                args.random,
                &dni_letters,
                start_date,
                &male,
                &female,
                &surnames,
            );
        } else {
            println!("The sex has to be either 1 (male) or 2 (female)");
            std::process::exit(1);
        }
    } else {
        generate_male(
            male_categories,
            args.random,
            &dni_letters,
            start_date,
            &male,
            &female,
            &surnames,
        );
        generate_female(
            female_categories,
            args.random,
            &dni_letters,
            start_date,
            &male,
            &female,
            &surnames,
        );
    }
}
