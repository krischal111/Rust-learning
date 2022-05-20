fn main()
{
    let n = 6;
    println!("The fibonacci sequence upto {n} is {}",fibonacci(n));
}
fn fibonacci(n:u32) -> u32
{
    if n<1
    {
        return 1;
    }

    return fibonacci(n-1)+fibonacci(n-2);
}