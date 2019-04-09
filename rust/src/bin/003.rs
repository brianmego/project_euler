fn main() {
    let mut target_num: i64 = 600851475143;
    let mut test_val = 2;
    while test_val < ((target_num as f64).sqrt() as i64) {
        while target_num % test_val == 0 {
            target_num /= test_val;
        }
        test_val += 1;
    }
    println!("{}", target_num)
}
