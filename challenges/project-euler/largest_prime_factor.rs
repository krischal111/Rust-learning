fn main() {
    let num: i64 = 600851475143;
    let mut prime_list: Vec<i64> = vec![];
    let mut prime_factors: Vec<i64> = vec![];

    let mut a = num;
    let mut end = ((a * 2) as f64).sqrt() as i64;
    let mut i = 2; 
    loop {
        if i > end {
            break;
        }
        let mut isprime = true;
        'primecheck: for prime_number in &prime_list {
            if (i % prime_number) == 0 {
                isprime = false;
                break 'primecheck;
            }
        }
        if isprime {
            // println!("Prime number = {}", i);
            prime_list.push(i);
            if (a % i) == 0 {
                prime_factors.push(i);
                let mut b = a;
                while (b%i) == 0 {
                    println!("Prime factor = {}", i);
                    b /= i;
                    if (b % i) != 0 {
                        a = b;
                        break;
                    }
                }
            }
        }
        end = ((a * 2) as f64).sqrt() as i64;
        i += 1;
    }
    println!("Last prime factor = {}",a);
    prime_factors.push(a);
    println!("Prime numbers generated in the process = \n {:?}\nNumber of primes generated = {}",prime_list, prime_list.len());
    println!("The prime factors of {} are \n {:?}",num , prime_factors);
}
