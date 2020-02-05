fn main() {
    let interim: i32 = 999;
    let largest: i32 = interim.pow(2);
    let smallest: i32 = 100;
    for i in (smallest..largest).rev() {
        if is_palindrome(i) {
            if has_three_digit_factors(i) {
                    println!("{}", i);
                    break;
            }
        }
    }
}

fn is_palindrome(i: i32) -> bool {
    let str_i = i.to_string();
    let str_j: String = str_i.chars().rev().collect();
    return str_i == str_j;
}

fn has_three_digit_factors(i: i32) -> bool {
    for j in (500..999).rev() {
        if i % j == 0 {
            let quotient = i / j;
            if (quotient > 100) && (quotient <= 999) {
                return true;
            }
        }
    }
    return false;
}
