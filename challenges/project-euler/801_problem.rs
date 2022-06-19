#![allow(unused_parens)]
use std::thread;
use thread::JoinHandle;
// use std::io::Duration;
fn main()
{
    const thread_count : usize = 17;
    // let lower : i64 = 1_00000_00000_00000_0;
    // let upper : i64 =     lower + 1_00000_0;

    let lower : i64 = 2;
    let upper : i64 = 20;
    
    // let mut n:[usize; thread_count] = [0;thread_count];
    let mut handle: Vec<JoinHandle<i64>> = vec![];

    for i in 0 .. thread_count
    {
        let ii = i;
        let ll = lower;
        let uu = upper;
        let this_handle = thread::spawn( move || {
            let mut n = 0;
            let mut number = ll + (ii as i64);
            let mut primesum : i64 = 0;
            while number < uu {
                if is_prime(number)
                {
                    n += 1;
                    primesum += number;
                    primesum %= 99_33533_99;
                    println!("{} th prime number = {} sum = {}  from thread {}",n,number, primesum,ii);

                }
                number += thread_count as i64;
            }
            primesum
        } );
        handle.push(this_handle);
    }



    let mut prime_sum:[i64;thread_count] = [0;thread_count];
    let mut total_prime_sum : i64 = 0;
    for thread_number in (0 .. thread_count)
    {
        let a = handle.pop().unwrap().join().unwrap();

        total_prime_sum += 
            {
                prime_sum[thread_count - thread_number -1] = a;
                a
            };
        total_prime_sum %= 99_33533_99;
    }

    println!("The sum of primes are: ");
    for i in 0 .. thread_count
    {
        println!("Sum of prime from {i}th thread = {}",prime_sum[i]);
    }
    println!("The total prime sum is = {total_prime_sum}");
}

fn is_prime(n:i64) -> bool
{
    // if n == 1
    // {
    //     return false;
    // }
    if n < 4
    {
        return true;
    }
    if (n % 2 == 0) || (n % 3 == 0)
    {
        return false;
    }
    else
    {
        let mut i:i64 = 5;
        while (i * i) <= n {
            if (n % i == 0) || (n % (i+2) == 0) {
                return false;
            }
            i += 6;
        }
    }
    true
}