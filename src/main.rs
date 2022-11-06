use std::io;

fn main() {
    println!("Convert temperature");
    let mut option = String::new();

    loop {
        println!("Press 'F' if you want to convert Celsius to Fahrenheit");
        println!("'C' if you want to convert Fahrenheit to Celsius.");
        println!("or 'E' if you want to exit.");

        option.clear();
        io::stdin().read_line(&mut option).unwrap();

        let grades = option.clone();

        match option.to_uppercase().trim() {
            "C" | "F" => convert_temperature(grades),
            "E" => break,
            &_ => {},
        };
    }

}
fn convert_temperature(option: String) {
    let mut temperature = String::new();
    let new_temperature: f32;

    println!("Enter the temperature to convert");

    io::stdin().read_line(&mut temperature)
        .expect("Failed to read line");

    let temperature = temperature.trim().parse().unwrap();


    match option.to_uppercase().trim() {           
        "C" => new_temperature = convert_to_celsius(temperature),
        "F" => new_temperature = convert_to_fahrenheit(temperature),
        &_ => new_temperature = 0.0,
    };

    println!("The temperature is {} º{}", new_temperature, option);
}

fn convert_to_fahrenheit(temperature: f32) -> f32 {
    let fahrenheit: f32;
    fahrenheit = (temperature * 1.8) + 32.0;
    return fahrenheit;
}

fn convert_to_celsius(temperature: f32) -> f32 {
    let celsius: f32;
    celsius = (temperature - 32.0) * 0.55;
    return celsius;
}
