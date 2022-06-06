fn main() {
    let mut a: i64 = 0;
    let mut prime_list: Vec<i64> = vec![];
    let mut i = 2;
    loop {
        let mut isprime = true;
        'primecheck: for prime_number in &prime_list {
            if (i % prime_number) == 0 {
                isprime = false;
                break 'primecheck;
            }
        }
        if isprime {
            prime_list.push(i);
            a += 1;
            println!("{a}th prime = {i}");
            if( a == 10_001)
            {break;}
        }
        i+=1;
    }
    // println!("Prime number list is\n {:?}", prime_list);
}
