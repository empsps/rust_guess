use std::io::{stdin, stdout, Write};
use std::ops::RangeInclusive;
use std::str::FromStr;
use std::{fmt, u16};

use rand::{thread_rng, Rng};

#[derive(PartialEq, Eq)]
enum Difficulty {
    EASY,
    MEDIUM,
    HARD,
}

impl FromStr for Difficulty {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Difficulty::EASY),
            "2" => Ok(Difficulty::MEDIUM),
            "3" => Ok(Difficulty::HARD),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Difficulty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Difficulty::EASY => write!(f, "fácil"),
            Difficulty::MEDIUM => write!(f, "médio"),
            Difficulty::HARD => write!(f, "difícil"),
        }
    }
}

fn read_input() -> String {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_owned()
}

fn input_parse_dif() -> Difficulty {
    loop {
        let input: String = read_input();
        match input.parse::<Difficulty>() {
            Ok(dif) => break dif,
            Err(_) => {
                print!("Opção inválida. Tente novamente: ");
                stdout().flush().unwrap();
                continue;
            }
        }
    }
}

fn input_parse_u16() -> u16 {
    loop {
        let input: String = read_input();
        match input.parse::<u16>() {
            Ok(number) => break number,
            Err(_) => {
                print!("Número inválido, tente novamente: ");
                stdout().flush().unwrap();
                continue;
            }
        }
    }
}

fn generate_number_and_guesses(dif: Difficulty) -> (u16, u16, String, String) {
    let mut rng = thread_rng();

    let range: RangeInclusive<u16>;
    let max_guesses: u16;
    let range_min: String;
    let range_max: String;

    match dif {
        Difficulty::EASY => {
            range = 1..=20;
            max_guesses = 5;
            range_min = String::from("1");
            range_max = String::from("20");
        }
        Difficulty::MEDIUM => {
            range = 1..=50;
            max_guesses = 4;
            range_min = String::from("1");
            range_max = String::from("50");
        }
        Difficulty::HARD => {
            range = 1..=100;
            max_guesses = 3;
            range_min = String::from("1");
            range_max = String::from("100");
        }
    }

    (rng.gen_range(range), max_guesses, range_min, range_max)
}

fn guess_loop(rand_number: &u16, max_guesses: &u16) -> Result<u16, ()> {
    let mut current_guesses: u16 = 0;

    loop {
        let user_guess: u16 = input_parse_u16();
        current_guesses += 1;

        if user_guess == *rand_number {
            return Ok(current_guesses);
        } else {
            if current_guesses == *max_guesses {
                return Err(());
            }
            println!("Você errou! {} tentativas restantes.", current_guesses);
            print!("Tente novamente -> ");
            stdout().flush().unwrap();
        }
    }
}

fn main() {
    println!("Bem-vindo ao Number Guesser\n");
    print!("Escolha a dificuldade (1 = fácil, 2 = médio, 3 = difícil): ");
    stdout().flush().unwrap();
    let dif: Difficulty = input_parse_dif();

    println!("Você escolheu: {}", &dif);

    let (rand_number, max_guesses, range_min, range_max): (u16, u16, String, String) =
        generate_number_and_guesses(dif);
    println!("{}", rand_number);

    print!("Adivinhe o número de {} a {} -> ", range_min, range_max);
    stdout().flush().unwrap();

    let result: Result<u16, ()> = guess_loop(&rand_number, &max_guesses);
    match result {
        Ok(guesses) => println!(
            "Você acertou em {} tentativas! O número era {}.",
            guesses, rand_number
        ),
        Err(()) => println!("Você usou todas as tentativas e não acertou :/"),
    }
}
