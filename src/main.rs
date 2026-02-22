use std::io;

#[derive(PartialEq)]
enum ConversionType {
    DegreeToFahrenheit,
    FahrenheitToDegree,
    Unknown
}


fn fahrenheit_to_degree(degree_f: f64) -> f64 {
    ((degree_f - 32.0)*5.0)/9.0
}

fn degree_to_fahrenheit(degree_c: f64) -> f64 {
    ((degree_c*9.0)/5.0)+32.0
}

fn print_character_n_times(character: char, character_count: usize) {
    for _iteration in 1..=character_count {
        print!("{character}");
    }
    println!("");
}

fn get_degree_symbol(conversion_type : ConversionType) -> char {
    if conversion_type == ConversionType::DegreeToFahrenheit {'C'} else {'F'}
}

fn main() {
    // display program title
    let program_title = String::from("Fahrenheit <--> Celcius converter program");
    print_character_n_times('*', program_title.len());
    println!("{program_title}");
    print_character_n_times('*', program_title.len());

    // process conversion type
    let conversion_type : ConversionType = loop {
        println!("Select the conversion type (d/D: degree -> fahrenheit, f/F: fahrenheit -> degree): ");
        let mut conversion_type = String::new();
        io::stdin()
            .read_line(&mut conversion_type)
            .expect("Failed to read line.");

        let conversion_type: ConversionType = match conversion_type.trim() {
            "d" => ConversionType::DegreeToFahrenheit,
            "D" => ConversionType::DegreeToFahrenheit,
            "f" => ConversionType::FahrenheitToDegree,
            "F" => ConversionType::FahrenheitToDegree,
            _ => ConversionType::Unknown
        };

        if conversion_type == ConversionType::Unknown {
            println!("Please select a valid conversion type (f/F/d/D).");
            continue;
        }
        break conversion_type;
    };

    // process degree value
    let degree : f64 = loop {
        println!("Enter the value you want to convert (a floating value is acceptable): ");
        let mut degree = String::new();

        io::stdin()
            .read_line(&mut degree)
            .expect("Failed to read line.");

        let degree : f64 = match degree.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("You didn't enter a valid degree value.");
                continue;
            }
        };
        break degree;
    };

    let converted_degree = match conversion_type {
        ConversionType::DegreeToFahrenheit => degree_to_fahrenheit(degree),
        ConversionType::FahrenheitToDegree => fahrenheit_to_degree(degree),
        ConversionType::Unknown => f64::NAN
    };

    let degree_symbol: char = get_degree_symbol(conversion_type);
    println!("Conversion result: {} °{} = {} °{}",
        degree,
        degree_symbol,
        converted_degree,
        degree_symbol
    );
}
