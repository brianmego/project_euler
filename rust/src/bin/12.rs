fn main() {
    let mut triangle: i64 = 0;
    let mut i = 1;
    loop {
        triangle += i;
        let mut divisors = 0;
        let mut j = 1;
        let mut max = triangle;
        while j <= max {
            if triangle % j == 0 {
                divisors += 2;
                max = triangle / j;
            }
            j += 1;
        }
        if divisors >= 500 {
            println!("i: {}, triangle: {}, {} divisors", i, triangle, divisors);
            break;
        }
        i += 1;
    }
}
