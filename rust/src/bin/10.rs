fn main() {
    let target = 2000000;
    let mut total = 0;
    let mut primes = vec![true; target];

    let max_target = (target as f64).sqrt();
    for i in 2..(max_target as usize) {
        if primes[i] {
            let mut j = i*2;
            while j < target {
                primes[j] = false;
                j += i;
            }
        }
    }
    for (i, prime) in primes.iter().enumerate().take(target).skip(2) {
        if *prime {
            total += i;
        }
    }
    println!("{}", total);
}
