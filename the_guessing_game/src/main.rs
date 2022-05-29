use rand::Rng;
use std::io;
use std::io::Write;
use termion::{color, style};
use std::cmp::Ordering;

fn main() {
    print!("\n Guess a number from 0 to 9999: {}",color::Fg(color::Blue));
    io::stdout().flush().unwrap();

    let secret_number = rand::thread_rng().gen_range(1..10000);
    let secret = secret_number.to_string();
    
    loop
    {
        let mut guess = String::new(); // readline just appends to the end
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");
        guess.truncate(guess.trim_end().len());

if false
{
    println!(
        "{}\n Your number = {}'{guess}'{}\n\
Secret number = {}'{secret}'{}.",
        style::Reset,
        color::Fg(color::Blue),
        style::Reset,
        color::Fg(color::Red),
        style::Reset
    );
    io::stdout().flush().unwrap();
}

        match guess.cmp(&secret){
            Ordering::Less => println!("{}Too small!",color::Fg(color::Yellow)),
            Ordering::Greater => println!("{}Too big!",color::Fg(color::Yellow)),
            Ordering::Equal => 
            {
                println!("{}\nYay! You guessed it!",color::Fg(color::Green));
                print!("{}",style::Reset);
                break;
            }
        }

        print!("{}\nEnter a number again: ",style::Reset);
        io::stdout().flush().unwrap();
    }
}
