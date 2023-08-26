fn main() {
    let target: i32 = 100;

    println!("{}", (square_of_sums(target) - sum_of_squares(target)));
}

fn square_of_sums(target: i32) -> i32 {
    let mut sum: i32 = 0;
    for i in 1..target+1 {
        sum += i;
    }
    sum * sum
}

fn sum_of_squares(target: i32) -> i32 {
    let mut sum: i32 = 0;
    for i in 1..(target + 1) {
        sum += i.pow(2);
    }
    sum
}
