fn main()
{
    let mut lcm = 1;
    for i in 1 ..= 20 {
        print!("a = {lcm} b = {i}");
        lcm = LCM(lcm, i);
        println!(" LCM = {lcm}");
    }
   println!("{lcm}");
}

fn HCF(a:i64,b:i64) -> i64
{
    let mut hcf = 1;
    let c = if a<b {a} else {b};
    for i in 1..=c {
        if (a %i == 0) && (b %i == 0) {
            hcf = i;
        }
    }
    hcf
}

fn LCM(a:i64,b:i64) -> i64
{
    let hcf = HCF(a,b);
    // println!("For a = {a} and b = {b} \nHCF = {hcf}");
    (a * b) / hcf
}