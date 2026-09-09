use std::io;

#[derive(Clone, Copy, Debug)]
enum TemperatureUnit {
    Celsius,
    Fahrenheit,
    Kelvin,
}

struct Temperature {
    unit: TemperatureUnit,
    value: f64,
}

impl TemperatureUnit {
    fn as_str(&self) -> &'static str {
        match self {
            TemperatureUnit::Celsius => "°C",
            TemperatureUnit::Fahrenheit => "°F",
            TemperatureUnit::Kelvin => "K",
        }
    }
}

fn convert_temperature(temp: Temperature, target: TemperatureUnit) -> f64 {
    let celsius_value = match temp.unit {
        TemperatureUnit::Celsius => temp.value,
        TemperatureUnit::Fahrenheit => (temp.value - 32.0) / 1.8,
        TemperatureUnit::Kelvin => temp.value - 273.15,
    };

    match target {
        TemperatureUnit::Celsius => celsius_value,
        TemperatureUnit::Fahrenheit => (celsius_value * 1.8) + 32.0,
        TemperatureUnit::Kelvin => celsius_value + 273.15,
    }
}

fn read_unit_selection(prompt: &str) -> TemperatureUnit {
    println!(); // Prints a blank line for spacing before the prompt
    println!("{}", prompt);

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line!");

    match input.trim().parse::<u32>().expect("Input must be a number!") {
        1 => TemperatureUnit::Celsius,
        2 => TemperatureUnit::Fahrenheit,
        3 => TemperatureUnit::Kelvin,
        _ => panic!("Please enter 1, 2, or 3!"),
    }
}

fn main() {
    println!("Input temperature value:");

    let mut value = String::new();
    io::stdin().read_line(&mut value).expect("Failed to read line!");
    let value: f64 = value.trim().parse().expect("Failed to parse the input as a number.");
    
    let source_unit = read_unit_selection("Select source unit:\n\t1. Celsius\n\t2. Fahrenheit\n\t3. Kelvin");
    let target_unit = read_unit_selection("Select target unit:\n\t1. Celsius\n\t2. Fahrenheit\n\t3. Kelvin");

    let temp = Temperature { unit: source_unit, value };
    let output = convert_temperature(temp, target_unit);

    println!(); // Prints a blank line before the final result
    println!(
        "According to Ferris the Crab, {:.2} {} is equal to {:.2} {}",
        value,
        source_unit.as_str(),
        output,
        target_unit.as_str()
    );
}