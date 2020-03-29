fn main() {
    let step_size: i32 = 20;
    let mut target = step_size;

    loop {
        let mut success: bool = true;
        for i in 1..step_size {
            if target % i != 0 {
                success=false;
                break;
            }
        }
        if success == true {
            println!("{}", target);
            break;
        }
        target += step_size;
    }
}
