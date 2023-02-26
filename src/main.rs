use chrono::{Days, Months, NaiveDate, Utc};
use clap::Parser;
use date_time_parser::DateParser;
use rand::{prelude::SliceRandom, Rng};
use std::collections::HashMap;

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

    // create a vector with all the names and surnames

    let male = vec![
        "Antonio",
        "Adrián",
        "Agustín",
        "Aitor",
        "Albert",
        "Alberto",
        "Alejandro",
        "Álex",
        "Alfonso",
        "Alfredo",
        "Álvaro",
        "Andrés",
        "Ángel",
        "Carlos",
        "César",
        "Cristian",
        "Daniel",
        "David",
        "Diego",
        "Domingo",
        "Eduardo",
        "Emilio",
        "Enrique",
        "Felipe",
        "Félix",
        "Fernando",
        "Francisco",
        "Francisco Javier",
        "Francisco José",
        "Gabriel",
        "Gonzalo",
        "Gregorio",
        "Guillermo",
        "Héctor",
        "Hugo",
        "Ignacio",
        "Íker",
        "Ismael",
        "Iván",
        "Jaime",
        "Javier",
        "Jesús",
        "Joan",
        "Joaquín",
        "Jordi",
        "Jorge",
        "José",
        "José Ángel",
        "José Antonio",
        "José Carlos",
        "José Francisco",
        "José Ignacio",
        "José Luis",
        "José Manuel",
        "José María",
        "José Miguel",
        "José Ramón",
        "Josep",
        "Juan",
        "Juan Antonio",
        "Juan Carlos",
        "Juan Francisco",
        "Juan José",
        "Juan Luis",
        "Juan Manuel",
        "Julián",
        "Julio",
        "Lorenzo",
        "Luis",
        "Luis Miguel",
        "Manuel",
        "Marc",
        "Marcos",
        "Mariano",
        "Mario",
        "Martín",
        "Miguel",
        "Miguel Ángel",
        "Mohamed",
        "Nicolás",
        "Óscar",
        "Pablo",
        "Pedro",
        "Rafael",
        "Ramón",
        "Raúl",
        "Ricardo",
        "Roberto",
        "Rodrigo",
        "Rubén",
        "Salvador",
        "Samuel",
        "Santiago",
        "Sebastián",
        "Sergio",
        "Tomás",
        "Vicente",
        "Víctor",
        "Víctor Manuel",
        "Xavier",
    ];
    let female = vec![
        "Alba",
        "Alicia",
        "Amparo",
        "Ana",
        "Ana Belén",
        "Ana Isabel",
        "Ana María",
        "Andrea",
        "Ángela",
        "Ángeles",
        "Antonia",
        "Aurora",
        "Beatriz",
        "Carla",
        "Carmen",
        "Carolina",
        "Catalina",
        "Celia",
        "Claudia",
        "Concepción",
        "Consuelo",
        "Cristina",
        "Dolores",
        "Elena",
        "Emilia",
        "Encarnación",
        "Esperanza",
        "Esther",
        "Eva",
        "Eva María",
        "Francisca",
        "Gloria",
        "Inés",
        "Inmaculada",
        "Irene",
        "Isabel",
        "Josefa",
        "Josefina",
        "Juana",
        "Julia",
        "Laura",
        "Lidia",
        "Lorena",
        "Lucía",
        "Luisa",
        "Manuela",
        "Margarita",
        "María",
        "María Angeles",
        "María Antonia",
        "María Carmen",
        "María Concepción",
        "María Cristina",
        "María Dolores",
        "María Elena",
        "María Isabel",
        "María Jesús",
        "María José",
        "María Josefa",
        "María Luisa",
        "María del Mar",
        "María Mercedes",
        "María Nieves",
        "María Pilar",
        "María Rosa",
        "María Rosario",
        "María Soledad",
        "María Teresa",
        "María Victoria",
        "Marina",
        "Marta",
        "Mercedes",
        "Milagros",
        "Miriam",
        "Mónica",
        "Montserrat",
        "Natalia",
        "Nerea",
        "Noelia",
        "Nuria",
        "Olga",
        "Patricia",
        "Paula",
        "Pilar",
        "Raquel",
        "Rocío",
        "Rosa",
        "Rosa María",
        "Rosario",
        "Sandra",
        "Sara",
        "Silvia",
        "Sofía",
        "Sonia",
        "Susana",
        "Teresa",
        "Verónica",
        "Victoria",
        "Virginia",
        "Yolanda",
    ];
    let surnames = vec![
        "Aguilar",
        "Alonso",
        "Álvarez",
        "Arias",
        "Benítez",
        "Blanco",
        "Bravo",
        "Caballero",
        "Cabrera",
        "Calvo",
        "Campos",
        "Cano",
        "Carmona",
        "Carrasco",
        "Castillo",
        "Castro",
        "Cortés",
        "Crespo",
        "Cruz",
        "Delgado",
        "Díaz",
        "Díez",
        "Domínguez",
        "Durán",
        "Esteban",
        "Fernández",
        "Ferrer",
        "Flores",
        "Fuentes",
        "Gallardo",
        "Gallego",
        "García",
        "Garrido",
        "Gil",
        "Giménez",
        "Gómez",
        "González",
        "Guerrero",
        "Gutiérrez",
        "Hernández",
        "Herrera",
        "Herrero",
        "Hidalgo",
        "Ibáñez",
        "Iglesias",
        "Jiménez",
        "León",
        "López",
        "Lorenzo",
        "Lozano",
        "Marín",
        "Márquez",
        "Martín",
        "Martínez",
        "Medina",
        "Méndez",
        "Molina",
        "Montero",
        "Mora",
        "Morales",
        "Moreno",
        "Moya",
        "Muñoz",
        "Navarro",
        "Nieto",
        "Núñez",
        "Ortega",
        "Ortiz",
        "Parra",
        "Pascual",
        "Pastor",
        "Peña",
        "Pérez",
        "Prieto",
        "Ramírez",
        "Ramos",
        "Reyes",
        "Rodríguez",
        "Rojas",
        "Román",
        "Romero",
        "Rubio",
        "Ruiz",
        "Sáez",
        "Sánchez",
        "Santana",
        "Santiago",
        "Santos",
        "Sanz",
        "Serrano",
        "Soler",
        "Soto",
        "Suárez",
        "Torres",
        "Vargas",
        "Vázquez",
        "Vega",
        "Velasco",
        "Vicente",
        "Vidal",
    ];

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

    for age in male_categories {
        let dni = random_dni(&dni_letters);
        let (name, surname1, surname2) = random_name(true, &male, &female, &surnames);

        let mut birthday: NaiveDate;
        if args.random && rand::thread_rng().gen_bool(0.7) {
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
    for age in female_categories {
        let dni = random_dni(&dni_letters);
        let (name, surname1, surname2) = random_name(false, &male, &female, &surnames);

        let mut birthday: NaiveDate;
        if args.random && rand::thread_rng().gen_bool(0.7) {
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

fn random_dni(dni_letters: &HashMap<u8, &str>) -> String {
    let mut dni = String::new();
    for _ in 0..8 {
        let number = rand::thread_rng().gen_range(0..=9);
        dni.push_str(&number.to_string());
    }

    // takes the dni, converts it to a number and calculates the remainder of the division by 23
    let dni_number = dni.parse::<u64>().unwrap();
    let dni_remainder = dni_number % 23;

    dni.push_str(dni_letters.get(&(dni_remainder as u8)).unwrap());

    dni
}

fn random_name(
    is_male: bool,
    male: &Vec<&str>,
    female: &Vec<&str>,
    surnames: &Vec<&str>,
) -> (String, String, String) {
    let name: &str;

    if is_male {
        name = *male.choose(&mut rand::thread_rng()).unwrap();
    } else {
        name = *female.choose(&mut rand::thread_rng()).unwrap();
    }

    let surname1 = *surnames.choose(&mut rand::thread_rng()).unwrap();
    let surname2 = *surnames.choose(&mut rand::thread_rng()).unwrap();

    (
        String::from(name),
        String::from(surname1),
        String::from(surname2),
    )
}
