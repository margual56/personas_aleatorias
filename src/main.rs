use chrono::{NaiveDate, Utc};
use clap::Parser;
use date_time_parser::DateParser;
use json::JsonValue;
use random_person::generate_info;
use std::{fs::File, io::Read};

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

    #[clap(
        short,
        long,
        help = "The number of people to generate per category. Default is 1",
        default_value = "1"
    )]
    number: u32,

    #[clap(
        short,
        long,
        help = "Output just this age. If not present, all ages will be generated"
    )]
    age: Option<u32>,
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

    let male_categories = vec![18, 35, 40, 45, 50, 55, 60, 65, 70];
    let female_categories = vec![18, 40, 45, 50, 55, 60, 65, 70];

    println!("DNI\tNombre\tApellidos\tFecha de nacimiento\tRango de edad");

    if let Some(sex) = args.sex {
        if sex == 1 {
            // Generate male
            generate_info(
                match args.age {
                    Some(age) => vec![age],
                    None => male_categories,
                },
                args.random,
                args.number,
                start_date,
                &male,
                &surnames,
            );
        } else if sex == 2 {
            // Generate female
            generate_info(
                match args.age {
                    Some(age) => vec![age],
                    None => female_categories,
                },
                args.random,
                args.number,
                start_date,
                &female,
                &surnames,
            );
        } else {
            println!("The sex has to be either 1 (male) or 2 (female)");
            std::process::exit(1);
        }
    } else {
        // Generate male
        generate_info(
            match args.age {
                Some(age) => vec![age],
                None => male_categories,
            },
            args.random,
            args.number,
            start_date,
            &male,
            &surnames,
        );

        // Generate female
        generate_info(
            match args.age {
                Some(age) => vec![age],
                None => female_categories,
            },
            args.random,
            args.number,
            start_date,
            &female,
            &surnames,
        );
    }
}
