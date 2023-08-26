fn main() {
    let mut a = 1;
    let mut b = 1;
    let mut total = 0;

    while b < 4000000 {
        let c = b;
        b += a;
        a = c;
        if b % 2 == 0 {
            total += b;
        }
    }
    println!("{}", total)
}
