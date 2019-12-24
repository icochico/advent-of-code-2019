use std::{env, io};
use std::fs::File;
use std::io::{prelude::*, BufReader};

use std::error::Error;

/// Fuel required to launch a given module is based on its mass.
/// Specifically, to find the fuel required for a module, take its mass, divide by three, round down, and subtract 2.
///
///For example:
///
///For a mass of 12, divide by 3 and round down to get 4, then subtract 2 to get 2.
///For a mass of 14, dividing by 3 and rounding down still yields 4, so the fuel required is also 2.
///For a mass of 1969, the fuel required is 654.
///For a mass of 100756, the fuel required is 33583.
///The Fuel Counter-Upper needs to know the total fuel requirement.
/// To find it, individually calculate the fuel needed for the mass of each module (your puzzle input),
/// then add together all the fuel values.
fn main() -> std::result::Result<(), Box<dyn Error>> {

    let file = read_file().unwrap();
    let reader = BufReader::new(file);
    // closure to calculate fuel
    let fuel = |mass| ((((mass / 3) as f32).floor() as i32) - 2);

    let mut total_fuel = 0;
    for line in reader.lines() {
        let mass: u32 = line?.parse().unwrap();
        println!("Mass: {}", mass);
        let module_fuel = fuel(mass);
        total_fuel += module_fuel;
    }

    println!("Total fuel needed: {}", total_fuel);
    Ok(())
}

fn read_file() -> Result<File, io::Error> {

    let args: Vec<String> = env::args().collect();
    let file_path: &str = &args[1];
    println!("Reading file: {}", file_path);
    let file = File::open(file_path)?;
    Ok(file)
}
