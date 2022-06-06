use std::{io, io::Write};
fn main()
{
    print!("Please enter a temperature in fahrenheit = ");
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let f:f32 = input.trim().parse::<f32>().unwrap_or(32.0);
    let c:f32 = fahrenheit_to_celcius(f);

    println!("The temperature {f} F in celcius is {c} C.")

}
fn fahrenheit_to_celcius(f:f32) -> f32 {
    (f-32.0)/1.8
}