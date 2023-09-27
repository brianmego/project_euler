///Using names.txt (right click and 'Save Link/Target As...'), a 46K text file containing over five-thousand first names, begin by sorting it into alphabetical order.
///Then working out the alphabetical value for each name, multiply this value by its alphabetical position in the list to obtain a name score.
///
///For example, when the list is sorted into alphabetical order, COLIN, which is worth
///3 + 15 + 12 + 9 + 14 = 53,
///is the 938th name in the list. So, COLIN would obtain a score of 938 x 53 = 49714.
///
///What is the total of all the name scores in the file?

fn main() {
    let names = include_str!("../../../problems/0022_names.txt");
    let mut sorted = names.split(',').map(|x| x.replace('"', "")).collect::<Vec<String>>();
    sorted.sort();
    let total: usize = sorted.iter().enumerate().map(|(idx, word)| (idx + 1) * get_alpha_val(word) as usize).sum();
    println!("{:?}", total);
}

fn get_alpha_val(word: &str) -> u16 {
    // 65 characters in is where letters start
    // 65 is A, 66 is B, etc.
    word.chars().map(|x| x as u16 - 64).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_alpha_val() {
        assert_eq!(get_alpha_val("COLIN"), 53);
    }
}

