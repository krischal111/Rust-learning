fn main() {
    let a: i64 = 600851475143;
    let mut prime_list: Vec<i64> = vec![];
    let mut prime_factors: Vec<i64> = vec![];
    for i in 2..=(((a*2) as f64).sqrt() as i64) {
        let mut isprime = true;
        'primecheck: for prime_number in &prime_list {
            if (i % prime_number) == 0 {
                isprime = false;
                break 'primecheck;
            }
        }
        if isprime {
            prime_list.push(i);
            if (a % i) == 0 {
                prime_factors.push(i);
            }
        }
    }
    println!("Prime number list is\n {:?}", prime_list);
    println!("The prime factors of {a} are \n {:?}", prime_factors);
}
