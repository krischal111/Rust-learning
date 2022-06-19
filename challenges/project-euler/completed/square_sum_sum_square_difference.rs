fn main()
{
    let mut sum : i64 = 0;
    let mut squaresum : i64 = 0;
    for i in 1..=10 {
        sum+=i;
        squaresum += i * i;
    }
    println!("{}", sum*sum - squaresum)
}