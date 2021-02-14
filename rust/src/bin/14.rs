use std::collections::HashMap;

fn main() {
    let mut longest_start = 1;
    let mut longest_count = 1;
    let mut cached: HashMap<i64, i64> = HashMap::new();
    let mut steps=0;
    for i in 1..1000000 {
        let mut cur = i;
        let mut count = 1;

        while cur != 1 {
            steps += 1;
            if cached.contains_key(&cur) {
                let to_add = cached.get(&cur).unwrap();
                count += to_add - 1;
                break;
            }
            if cur % 2 == 0 {
                cur = cur / 2;
            }
            else {
                cur = (cur * 3) + 1;
            }
            count += 1;
        }
        cached.insert(i, count);
        if count > longest_count {
            longest_start = i;
            longest_count = count;
        }
    }
    println!("\nStart Number: {}; Count: {}; Steps: {}", longest_start, longest_count, steps);
}
