use std::io::{stdin, stdout, Write};
use std::ops::RangeInclusive;
use std::str::FromStr;
use std::{fmt, u16};

use rand::{thread_rng, Rng};

#[derive(PartialEq, Eq)]
enum Difficulty {
    Easy,
    Medium,
    Hard,
}

#[derive(PartialEq, Eq)]
enum Hint {
    LessThan,
    GreaterThan,
    Multiple,
    Divisible,
}

impl FromStr for Difficulty {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Difficulty::Easy),
            "2" => Ok(Difficulty::Medium),
            "3" => Ok(Difficulty::Hard),
            _ => Err(()),
        }
    }
}

impl FromStr for Hint {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Hint::LessThan),
            "2" => Ok(Hint::GreaterThan),
            "3" => Ok(Hint::Multiple),
            "4" => Ok(Hint::Divisible),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Difficulty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Difficulty::Easy => write!(f, "fácil"),
            Difficulty::Medium => write!(f, "médio"),
            Difficulty::Hard => write!(f, "difícil"),
        }
    }
}

impl fmt::Display for Hint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Hint::LessThan => write!(f, "menor que"),
            Hint::GreaterThan => write!(f, "maior que"),
            Hint::Multiple => write!(f, "múltiplo de"),
            Hint::Divisible => write!(f, "divisível po"),
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
        Difficulty::Easy => {
            range = 1..=20;
            max_guesses = 5;
            range_min = String::from("1");
            range_max = String::from("20");
        }
        Difficulty::Medium => {
            range = 1..=50;
            max_guesses = 4;
            range_min = String::from("1");
            range_max = String::from("50");
        }
        Difficulty::Hard => {
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

fn less_than_hint(number: &u16) -> u16 {
    let mut rng = thread_rng();
    let range: RangeInclusive<u16> = 2..=*number;

    rng.gen_range(range)
}

fn greater_than_hint(number: &u16) -> u16 {
    let mut rng = thread_rng();
    let range: RangeInclusive<u16> = 2..=*number;

    rng.gen_range(range)
}

fn multiple_hint(number: &u16) -> u16 {
    let mut rng = thread_rng();
    let range: RangeInclusive<u16> = 2..=*number;

    rng.gen_range(range)
}

fn divisible_hint(number: &u16) -> u16 {
    let mut rng = thread_rng();
    let range: RangeInclusive<u16> = 2..=*number;

    rng.gen_range(range)
}

fn give_hint(number: &u16) -> String {
    let mut rng = thread_rng();
    let choice = rng.gen_range(1..=4).to_string();

    let mut hint_result: u16 = 102;
    let mut hint_type: Option<Hint> = None;

    match choice.parse::<Hint>() {
        Ok(chosen_hint) => match chosen_hint {
            Hint::LessThan => {
                hint_type = Some(Hint::LessThan);
                hint_result = less_than_hint(number)
            }
            Hint::GreaterThan => {
                hint_type = Some(Hint::GreaterThan);
                hint_result = greater_than_hint(number)
            }
            Hint::Multiple => {
                hint_type = Some(Hint::Multiple);
                hint_result = multiple_hint(number)
            }
            Hint::Divisible => {
                hint_type = Some(Hint::Divisible);
                hint_result = divisible_hint(number)
            }
        },
        Err(_) => (),
    }

    if hint_type.is_none() {
        return String::from("Ocorreu um erro na geração de dicas. O jogo irá terminar.");
    }

    format!("Dica: o número é {} {}.", hint_type.unwrap(), hint_result)
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
        Err(_) => println!("Você usou todas as tentativas e não acertou :/"),
    }
}
