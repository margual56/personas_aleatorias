use chrono::{Days, Months, NaiveDate};
use rand::{seq::SliceRandom, Rng};

use phf::phf_map;

pub const DNI_LETTERS: phf::Map<u8, &'static str> = phf_map! {
        0u8 => "T",
        1u8 => "R",
        2u8 => "W",
        3u8 => "A",
        4u8 => "G",
        5u8 => "M",
        6u8 => "Y",
        7u8 => "F",
        8u8 => "P",
        9u8 => "D",
        10u8 => "X",
        11u8 => "B",
        12u8 => "N",
        13u8 => "J",
        14u8 => "Z",
        15u8 => "S",
        16u8 => "Q",
        17u8 => "V",
        18u8 => "H",
        19u8 => "L",
        20u8 => "C",
        21u8 => "K",
        22u8 => "E",
};

pub fn generate_info(
    categories: Vec<u32>,
    is_random: bool,
    times: u32,
    start_date: NaiveDate,
    names: &Vec<String>,
    surnames: &Vec<String>,
) {
    for age in categories {
        for _ in 0..times {
            let dni = random_dni();
            let (name, surname1, surname2) = random_name(&names, &surnames);

            let mut birthday: NaiveDate;
            if is_random && rand::thread_rng().gen_bool(0.7) {
                birthday = start_date
                    .checked_sub_months(Months::new(
                        (age + rand::thread_rng().gen_range(1..4)) * 12
                            + rand::thread_rng().gen_range(1..4),
                    ))
                    .unwrap();
                birthday = birthday
                    .checked_sub_days(Days::new(rand::thread_rng().gen_range(1..28)))
                    .unwrap();
            } else {
                birthday = start_date
                    .checked_sub_months(Months::new(age * 12))
                    .unwrap();
            }

            println!(
                "{}\t{}\t{}\t{}\t{}",
                dni,
                name,
                format!("{} {}", surname1, surname2),
                birthday.format("%d/%m/%Y"),
                age
            );
        }
    }
}

fn random_dni() -> String {
    let mut dni = String::new();
    for _ in 0..8 {
        let number = rand::thread_rng().gen_range(0..=9);
        dni.push_str(&number.to_string());
    }

    // takes the dni, converts it to a number and calculates the remainder of the division by 23
    let dni_number = dni.parse::<u64>().unwrap();
    let dni_remainder = dni_number % 23;

    dni.push_str(DNI_LETTERS.get(&(dni_remainder as u8)).unwrap());

    dni
}

fn random_name(names: &Vec<String>, surnames: &Vec<String>) -> (String, String, String) {
    let name = names.choose(&mut rand::thread_rng()).unwrap();

    let surname1 = surnames.choose(&mut rand::thread_rng()).unwrap();
    let surname2 = surnames.choose(&mut rand::thread_rng()).unwrap();

    (
        String::from(name),
        String::from(surname1),
        String::from(surname2),
    )
}
