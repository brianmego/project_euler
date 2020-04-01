fn main() {
    let target_index: u32 = 10001;
    println!("{}", get_prime(target_index));
}

fn get_prime(target_index: u32) -> u32 {
    let mut primes = vec![2];
    let mut current: u32 = 3;

    let mut length = primes.len();
    while length < target_index as usize {
        let mut is_prime: bool = true;
        for i in &primes {
            if current % i == 0 {
                is_prime = false;
                break
            }
        }
        if is_prime {
            primes.push(current);
            length = primes.len();
        }
        current += 2;

    }
    primes[(target_index - 1) as usize]
}
