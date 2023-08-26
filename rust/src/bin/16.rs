use num::bigint::BigInt;

fn main() {
    let initial = BigInt::from(2);
    let raised = initial.pow(1000);
    let sum: u32 = raised
        .to_string()
        .chars()
        .map(|c| c.to_string().parse::<u32>().unwrap())
        .sum();
    println!("Raised to power: {}", raised);
    println!("Sum: {}", sum);
}
