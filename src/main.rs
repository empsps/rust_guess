use std::io::{stdin, stdout, Write};
use std::ops::RangeInclusive;
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

fn generate_number_and_guesses(dif: &Difficulty) -> (u16, u16, String, String) {
    let mut rng = thread_rng();

    let range: RangeInclusive<u16>;
    let max_guesses: u16;
    let range_min: String;
    let range_max: String;

    match *dif {
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
    less_than_nums: &mut Vec<u16>,
    greater_than_nums: &mut Vec<u16>,
    divisible_nums: &mut Vec<u16>,
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
                less_than_nums,
                greater_than_nums,
                divisible_nums,
            );
            println!("{}", hint);

            print!("Tente novamente -> ");
            stdout().flush().unwrap();
        }
    }
}

#[allow(dead_code)]
fn less_than_hint(number: &u16, less_than_nums: &mut Vec<u16>) -> u16 {
    let mut rng = thread_rng();
    let hint_index = rng.gen_range(0..less_than_nums.len());
    let actual_hint = less_than_nums[hint_index];
    less_than_nums.drain(hint_index..less_than_nums.len());

    actual_hint
}

#[allow(dead_code)]
fn greater_than_hint(number: &u16, greater_than_nums: &mut Vec<u16>) -> u16 {
    let mut rng = thread_rng();
    let hint_index = rng.gen_range(0..greater_than_nums.len());
    let actual_hint = greater_than_nums[hint_index];
    greater_than_nums.drain(0..=hint_index);

    actual_hint
}

#[allow(dead_code)]
fn divisible_hint(number: &u16, divisible_nums: &mut Vec<u16>) -> u16 {
    let mut rng = thread_rng();

    let hint_index = rng.gen_range(0..divisible_nums.len());
    let actual_hint = divisible_nums[hint_index];

    divisible_nums.clear();

    actual_hint
}

fn give_hint(
    number: &u16,
    less_than_nums: &mut Vec<u16>,
    greater_than_nums: &mut Vec<u16>,
    divisible_nums: &mut Vec<u16>,
) -> String {
    let mut rng = thread_rng();
    let mut hint_vec: Vec<Hint> = Vec::new();

    if less_than_nums.len() > 0 {
        hint_vec.push(Hint::LessThan);
    }

    if greater_than_nums.len() > 0 {
        hint_vec.push(Hint::GreaterThan);
    }

    if divisible_nums.len() > 0 {
        hint_vec.push(Hint::Divisible);
    }

    if hint_vec.len() == 0 {
        return String::from("Não há mais dicas disponíveis :/");
    }

    let choice: &Hint = hint_vec.choose(&mut rng).unwrap();

    let hint_result: u16;

    match *choice {
        Hint::LessThan => {
            hint_result = less_than_hint(&number, less_than_nums);
        }
        Hint::GreaterThan => {
            hint_result = greater_than_hint(&number, greater_than_nums);
        }
        Hint::Divisible => {
            hint_result = divisible_hint(&number, divisible_nums);
        }
    }

    format!("Dica: o número é {} {}.", choice, hint_result)
}

fn build_hint_arrays(
    number: &u16,
    dif: &Difficulty,
    less_than_nums: &mut Vec<u16>,
    greater_than_nums: &mut Vec<u16>,
    divisible_nums: &mut Vec<u16>,
) {
    let max_num = match *dif {
        Difficulty::Easy => 20,
        Difficulty::Medium => 50,
        Difficulty::Hard => 100,
    };

    for num in *number + 1..max_num {
        less_than_nums.push(num);
    }

    for num in 1..*number {
        greater_than_nums.push(num);
    }

    for num in 2..=20 {
        if *number % num == 0 {
            divisible_nums.push(num);
        }
    }
}

fn main() {
    println!("Bem-vindo ao Number Guesser\n");
    print!("Escolha a dificuldade (1 = fácil, 2 = médio, 3 = difícil): ");
    stdout().flush().unwrap();

    let dif: Difficulty = input_parse_dif();

    println!("Você escolheu: {}", &dif);

    let mut less_than_nums: Vec<u16> = Vec::new();
    let mut greater_than_nums: Vec<u16> = Vec::new();
    let mut divisible_nums: Vec<u16> = Vec::new();

    let (rand_number, max_guesses, range_min, range_max): (u16, u16, String, String) =
        generate_number_and_guesses(&dif);
    println!("{}", rand_number);

    build_hint_arrays(
        &rand_number,
        &dif,
        &mut less_than_nums,
        &mut greater_than_nums,
        &mut divisible_nums,
    );

    print!("Adivinhe o número de {} a {} -> ", range_min, range_max);
    stdout().flush().unwrap();

    let result: Result<u16, ()> = guess_loop(
        &rand_number,
        &max_guesses,
        &mut less_than_nums,
        &mut greater_than_nums,
        &mut divisible_nums,
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
