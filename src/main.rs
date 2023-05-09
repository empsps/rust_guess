use std::io::{stdin, stdout, Write};
use std::ops::{Range, RangeInclusive};
use std::str::FromStr;
use std::{fmt, u16};

use rand::seq::SliceRandom;
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
            Hint::Divisible => write!(f, "divisível por"),
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

fn guess_loop(
    rand_number: &u16,
    max_guesses: &u16,
    max_less: &mut u16,
    min_greater: &mut u16,
    used_multiple: &mut bool,
    used_divisible: &mut bool,
) -> Result<u16, ()> {
    let mut current_guesses: u16 = 0;

    loop {
        let user_guess: u16 = input_parse_u16();
        current_guesses += 0;

        if user_guess == *rand_number {
            return Ok(current_guesses);
        } else {
            if current_guesses == *max_guesses {
                return Err(());
            }
            println!(
                "Você errou! {} tentativas restantes.",
                *max_guesses - current_guesses
            );

            let hint = give_hint(
                rand_number,
                max_less,
                min_greater,
                used_multiple,
                used_divisible,
            );
            println!("{}", hint);

            print!("Tente novamente -> ");
            stdout().flush().unwrap();
        }
    }
}

#[allow(dead_code)]
fn less_than_hint(number: &u16, max_less: &u16) -> u16 {
    let mut rng = thread_rng();
    let range: Range<u16> = *number + 1..*max_less;

    rng.gen_range(range)
}

#[allow(dead_code)]
fn greater_than_hint(number: &u16, min_greater: &u16) -> u16 {
    let mut rng = thread_rng();
    let range: Range<u16> = *min_greater..*number;

    rng.gen_range(range)
}

#[allow(dead_code)]
fn multiple_hint(number: &u16) -> u16 {
    let mut rng = thread_rng();
    // TODO
    0
}

#[allow(dead_code)]
fn divisible_hint(number: &u16) -> u16 {
    let mut rng = thread_rng();
    // TODO
    0
}

fn give_hint(
    number: &u16,
    max_less: &mut u16,
    min_greater: &mut u16,
    used_multiple: &mut bool,
    used_divisible: &mut bool,
) -> String {
    let mut rng = thread_rng();
    let mut hint_vec: Vec<Hint> = Vec::new();

    if !(*number + 1 == *max_less) {
        hint_vec.push(Hint::LessThan);
    }

    if !(*number == *min_greater) {
        hint_vec.push(Hint::GreaterThan);
    }

    // if !*used_multiple {
    //     hint_vec.push(Hint::Multiple);
    // }

    // if !*used_divisible {
    //     hint_vec.push(Hint::Divisible);
    // }

    if hint_vec.len() == 0 {
        return String::from("Não há mais dicas disponíveis :/");
    }

    let choice: &Hint = hint_vec.choose(&mut rng).unwrap();

    let hint_result: u16;

    match *choice {
        Hint::LessThan => {
            hint_result = less_than_hint(&number, &max_less);
            *max_less = hint_result;
        }
        Hint::GreaterThan => {
            hint_result = greater_than_hint(&number, &min_greater);
            *min_greater = hint_result + 1;
        }
        Hint::Multiple => {
            hint_result = multiple_hint(&number);
            *used_multiple = true;
        }
        Hint::Divisible => {
            hint_result = divisible_hint(&number);
            *used_divisible = true;
        }
    }

    format!("Dica: o número é {} {}.", choice, hint_result)
}

fn main() {
    println!("Bem-vindo ao Number Guesser\n");
    print!("Escolha a dificuldade (1 = fácil, 2 = médio, 3 = difícil): ");
    stdout().flush().unwrap();

    let dif: Difficulty = input_parse_dif();

    println!("Você escolheu: {}", &dif);

    let mut max_less: u16 = match dif {
        Difficulty::Easy => 20,
        Difficulty::Medium => 50,
        Difficulty::Hard => 100,
    };
    let mut min_greater: u16 = 1;
    let mut used_multiple: bool = false;
    let mut used_divisible: bool = false;

    let (rand_number, max_guesses, range_min, range_max): (u16, u16, String, String) =
        generate_number_and_guesses(dif);
    println!("{}", rand_number);

    print!("Adivinhe o número de {} a {} -> ", range_min, range_max);
    stdout().flush().unwrap();

    let result: Result<u16, ()> = guess_loop(
        &rand_number,
        &max_guesses,
        &mut max_less,
        &mut min_greater,
        &mut used_multiple,
        &mut used_divisible,
    );
    match result {
        Ok(guesses) => println!(
            "Você acertou em {0} tentativa{2}! O número era {1}.",
            guesses,
            rand_number,
            if guesses > 1 { "s" } else { "" }
        ),
        Err(_) => println!("Você usou todas as tentativas e não acertou :/"),
    }
}
