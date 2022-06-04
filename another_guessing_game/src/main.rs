use rand::Rng;
use std::io;
use std::io::Write;
use termion::{color,style};
// use std::cmp::Ordering;

fn main()
{
    print!("{}Enter a number from 0 to 9999: {}",
            style::Reset, color::Fg(color::Blue));
    io::stdout().flush().unwrap();

    let secret_number:i16 = rand::thread_rng().gen_range(1..10000);
    let s_digit:[u8;4] = [( secret_number/1000)     as u8,
                          ((secret_number/100)%10)  as u8,
                          ((secret_number/10)%10)   as u8,
                          ( secret_number%10)       as u8];
    loop
    {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading line");
        
        let guess:i16 = guess.trim().parse().expect("Please enter a number.");
        let g_digit:[u8;4] = [((guess/1000)%10) as u8,
                              ((guess/100)%10)  as u8,
                              ((guess/10)%10)   as u8,
                              ( guess%10)       as u8];
        
        print!("{}Your number = ",style::Reset);
        let mut correct = true;
        for i in 0..4
        {
            let mut exists = false;
            let mut same   = false;    
            for j in 0..4
            {
                if g_digit[i] == s_digit[j]
                {
                    exists = true;
                    if i == j
                    {
                        same = true;
                    }
                }
            }
            correct &= same;
            if exists
            {
                print!("{}{}",color::Bg(color::Rgb(0xffu8,0xffu8,00u8)),color::Fg(color::White));
                if same
                {
                    print!("{}{}",color::Bg(color::Rgb(0x00u8,0xffu8,00u8)),color::Fg(color::White));
                }
            }
            else
            {
                print!("{}", style::Reset);
            }
            print!("{}",g_digit[i]);
        }
        // io::stdout().flush().unwrap();

        if correct
        {
            println!("{}\n\n{}You guess was correct!{}",style::Reset, color::Fg(color::Yellow),style::Reset);
            break;
        }

        print!("{}\n\nYour guess wasn't correct, enter new number = {}", style::Reset, color::Fg(color::Blue));
        io::stdout().flush().unwrap();
    }
    
}