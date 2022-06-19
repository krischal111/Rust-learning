fn main()
{
    let mut a = 1;
    let mut b = 2;
    let mut total = 0;
    while a < 4_000_000 {
        if a & 1 == 0
        {
            total += a;
            println!("{a}, ")
        }
        (a, b) = (b, a+b);
    }
    println!("{total}");
}