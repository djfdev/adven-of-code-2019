use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let mut total = 0;

    for line in input.lines() {
        let mass = line.parse()?;
        total += get_fuel(mass);
    }

    println!("{}", total);
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut total = 0;

    for line in input.lines() {
        let mass = line.parse()?;
        total += get_fuel_recursive(mass);
    }

    println!("{}", total);
    Ok(())
}

fn get_fuel(mass: i32) -> i32 {
    mass / 3 - 2
}

fn get_fuel_recursive(mass: i32) -> i32 {
    let result = get_fuel(mass);

    if result <= 0 {
        0
    } else {
        result + get_fuel_recursive(result)
    }
}
