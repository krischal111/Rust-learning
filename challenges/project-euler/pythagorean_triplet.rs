fn main() {
    let required_sum:i64 = 1000;
    let (mut a, mut b, mut c) : (i64, i64, i64);
    for i in 1..required_sum {
        for j in (i + 1)..(required_sum - i - 1) {
            c = required_sum - (i + j);
            if c > j {
                if (i * i + j * j) == (c * c) {
                    (a, b, c) = (i, j, c);
                    println!(
                        "{a}+{b}+{c} = {} \n{a}*{a} (= {}) + {b}*{b} (={}) = {c} * {c} (= {})\n{a} * {b} * {c} = {} \n",
                        a + b + c, a*a, b*b, c*c, a*b*c
                    );
                }
            }
        }
    }
}
