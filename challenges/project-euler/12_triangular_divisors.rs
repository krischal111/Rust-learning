fn main() {
    let mut num: u64 = 0;
    let mut triangular_num: u64 = 0;
    let factor_count = loop {
        num += 1;
        triangular_num += num;
        let fc = count_factors(triangular_num);
        if fc > 500 {
            break fc;
        }
        println!(
            "The {num} th triangle number {triangular_num} has {fc} factors");
    };

    println!(
        "The {num} th triangle number {triangular_num} has {factor_count} factors");
}

fn count_factors(num: u64) -> u64 {
    let mut factor_count: u64 = 0;
    for i in 1..=(num >> 1) {
        if (num % i) == 0 {
            factor_count += 1;
        }
    }
    return factor_count+1;
}
