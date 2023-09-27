/// Let d(n) be defined as the sum of proper divisors of n
/// (numbers less than n which divide evenly into n).
/// If d(a) = b and d(b) = a, where a != b, then a and b
///  are an amicable pair and each of a and b are called amicable numbers.
///
/// For example, the proper divisors of 220
/// are 1, 2, 4, 5, 10, 11, 20, 22, 44, 55, and 110; therefore
/// d(220) = 284. The proper divisors of 284 are 1, 2, 4, 71, and 142;
/// so d(284) = 220.
///
/// Evaluate the sum of all the amicable numbers under 10000

use std::collections::HashMap;

fn main() {
    let mut all_solutions: HashMap<u16, u16> = HashMap::new();
    let _ = (1..10000).map(|n| all_solutions.insert(n, d(n))).collect::<Vec<_>>();
    let amicable_numbers = all_solutions.iter().filter(|x| all_solutions.get(x.1) == Some(x.0) && x.0 != x.1);
    println!("{}", amicable_numbers.map(|x| x.0).sum::<u16>());
}

fn d(n: u16) -> u16 {
    let min = 1;
    let max = n / 2;
    let proper_divisors = (min..=max).filter(|x| n % x == 0);
    proper_divisors.sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        assert_eq!(d(220), 284);
        assert_eq!(d(284), 220);
    }
}
