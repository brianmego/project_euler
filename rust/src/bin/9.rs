fn main() {
    let desired_sum = 1000;
    let mut product = 0;
    let mut max_b;
    let mut c;

    for a in 0..desired_sum {
        if product != 0 {
            break;
        }
        max_b = (desired_sum - a) / 2;
        for b in a..max_b {
            c = 1000 - a - b;
            if a*a + b*b == c*c {
                product = a * b * c;
            }
        }
    }
    println!("{}", product);
}

