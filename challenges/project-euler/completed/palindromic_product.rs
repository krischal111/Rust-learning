fn main()
{
    let (mut a, mut b, mut pp) : (i32, i32, i32) = (0,0,0);
    for i in 100 .. 1000
    {
        for j in 100 .. 1000
        {
            if ispalindromic(i * j)
            {
                if i*j > pp
                {
                    (a,b, pp) = (i,j, i*j);
                }
            }
        }
    }
    println!("{a} x {b} = {pp}");
    println!("{}", ispalindromic(913 * 993));
}

fn ispalindromic(pp:i32) -> bool
{
    let mut p = pp;
    let mut rev_p = 0;
    while p != 0 {
        rev_p *= 10;
        rev_p += p%10;
        p /= 10;
    }
    pp == rev_p
}