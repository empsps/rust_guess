use std::fmt;
use std::io::{stdout, Write};
use std::{io::stdin, str::FromStr};

#[derive(PartialEq, Eq)]
enum Unit {
    CELSIUS,
    FAHRENHEIT,
    KELVIN,
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Unit::CELSIUS => write!(f, "°C"),
            Unit::FAHRENHEIT => write!(f, "°F"),
            Unit::KELVIN => write!(f, "°K"),
        }
    }
}

impl FromStr for Unit {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "c" => Ok(Unit::CELSIUS),
            "f" => Ok(Unit::FAHRENHEIT),
            "k" => Ok(Unit::KELVIN),

            "celsius" => Ok(Unit::CELSIUS),
            "fahrenheit" => Ok(Unit::FAHRENHEIT),
            "kelvin" => Ok(Unit::KELVIN),
            _ => Err(()),
        }
    }
}

fn main() {
    println!("Temperature converter");
    print!("\nInsert the unit you want to convert from (C, F, K): ");
    stdout().flush().unwrap();

    let unit_from: Unit = input_parse_unit();

    print!("\nNow, the unit you want to convert to: ");
    stdout().flush().unwrap();

    let unit_to: Unit = loop {
        let result = input_parse_unit();
        if result == unit_from {
            print!("You cannot convert to the same unit, choose a different one: ");
            stdout().flush().unwrap();
            continue;
        } else {
            break result;
        }
    };

    print!("Now, enter the value you want to convert: ");
    stdout().flush().unwrap();

    let value = input_parse_i32();

    let converted = convert_temp(&value, &unit_from, &unit_to);
    println!(
        "\n{}{} is equal to {}{}",
        value, unit_from, converted, unit_to
    );
}

fn convert_temp(temp: &i32, from: &Unit, to: &Unit) -> i32 {
    match from {
        Unit::CELSIUS => match to {
            Unit::FAHRENHEIT => (temp * 9) / 5 + 32,
            Unit::KELVIN => temp + 273,
            _ => 0,
        },
        Unit::FAHRENHEIT => match to {
            Unit::CELSIUS => (temp - 32) * 5 / 9,
            Unit::KELVIN => (temp - 32) * 5 / 9 + 273,
            _ => 0,
        },
        Unit::KELVIN => match to {
            Unit::CELSIUS => temp - 273,
            Unit::FAHRENHEIT => (temp - 273) * 9 / 5 + 32,
            _ => 0,
        },
    }
}

fn read_input() -> String {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_owned()
}

fn input_parse_unit() -> Unit {
    loop {
        let input = read_input();
        match input.parse::<Unit>() {
            Ok(unit) => break unit,
            Err(_) => {
                print!("Invalid input, choose from: C, F, K: ");
                stdout().flush().unwrap();
                continue;
            }
        }
    }
}

fn input_parse_i32() -> i32 {
    loop {
        let input = read_input();
        match input.parse::<i32>() {
            Ok(temp) => break temp,
            Err(_) => {
                print!("That's not a valid temperature, try again: ");
                stdout().flush().unwrap();
                continue;
            }
        };
    }
}
