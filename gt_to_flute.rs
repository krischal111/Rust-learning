#[allow(unused_variables, dead_code, unused_assignments)]
use std::io::stdin;
use std::convert::TryInto;

fn key_to_shift(a: char) -> i32
{
    match a {
        'E' =>  4,
        'B' => -1,
        'G' =>  2,
        'D' =>  2,
        'A' => -3,
        'F' =>  5,
        _   =>  0,
    }
}


fn main()
{
    let names = [
        "Sa",
        "(Komal)Re",
        "Re",
        "(Komal)Ga",
        "Ga",
        "Ma",
        "Ma(Tivra)",
        "Pa",
        "(Komal)Dha",
        "Dha",
        "(Komal)Ni",
        "Ni",
        "Sa"
    ];

    let mut tabstring = String::new();
    let mut shiftvalue:i32 = 0;

    println!("Please enter key and frets of guitar: ");

    stdin().read_line(&mut tabstring)
    	.ok()
        .expect("Failed to read line");
    
    for token in tabstring.split_whitespace()
    {
        let c = token.chars().next().unwrap();
        if c.is_alphabetic()
        {   
            shiftvalue = key_to_shift(c);
            println!("");
            continue;
        }

        let a = token.parse::<i32>().unwrap();
        let tone:usize = (((a + shiftvalue)%12 + 12) % 12).try_into().unwrap();
        println!("{}", names[tone]);

        // println!( " {}  ",shiftvalue);
        // println!(" {} ", token);
    }

    println!("Your input-text was {}", tabstring);
}