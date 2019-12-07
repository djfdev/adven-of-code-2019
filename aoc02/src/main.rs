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
    let vec = parse_input(input)?;
    let result = run_program(&vec, 12, 2);

    println!("{}", result);
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let vec = parse_input(input)?;
    let mut result: Option<(usize, usize)> = None;

    for noun in 0..100 {
        for verb in 0..100 {
            if run_program(&vec, noun, verb) == 19_690_720 {
                result = Some((noun, verb));
            }
        }
    }

    if let Some((noun, verb)) = result {
        println!("{}", 100 * noun + verb);
    }

    Ok(())
}

fn parse_input(input: &str) -> Result<Vec<usize>> {
    let result = input.trim().split(',').map(|s| s.parse()).collect();

    match result {
        Ok(vec) => Ok(vec),
        Err(_) => Err(From::from("a non-integer value was found in the input")),
    }
}

fn run_program(input_vec: &[usize], noun: usize, verb: usize) -> usize {
    let mut vec = input_vec.to_owned();

    vec[1] = noun;
    vec[2] = verb;

    let mut i = 0;

    loop {
        let opcode = vec[i];
        let input1 = vec[vec[i + 1]];
        let input2 = vec[vec[i + 2]];
        let target = vec[i + 3];

        match opcode {
            1 => {
                vec[target] = input1 + input2;
            }
            2 => {
                vec[target] = input1 * input2;
            }
            _ => {
                break;
            }
        }

        i += 4;
    }

    vec[0]
}
