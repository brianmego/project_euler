///If the numbers 1 to 5 are written out in words: one, two, three, four, five, then there
/// are 3+3+5+4+4 = 19 letters used in total.
///
///If all the numbers from 1 to 1000 (one thousand) inclusive were written out in words,
/// how many letters would be used?
///
///
///NOTE: Do not count spaces or hyphens. For example, 342
/// (three hundred and forty-two) contains
/// 23 letters and 115 (one hundred and fifteen) contains 20
/// letters. The use of "and" when writing out numbers is in compliance with British usage.
///
use std::convert::TryInto;

fn main() {
    let total: usize = (1..=1000).map(|d| d.count_letters()).sum();
    println!("{}", total);
}

trait SerializeAsWords {
    fn as_words(&self) -> String;

    fn count_letters(&self) -> usize;
}

impl SerializeAsWords for u16 {
    fn as_words(&self) -> String {
        let mut integer = *self;
        let mut digits = Vec::with_capacity(4);
        for _i in 1..=4 {
            let rem = integer % 10;
            integer -= rem;
            integer /= 10;
            digits.push(rem.try_into().unwrap());
        }
        let thousands = match digits[3] {
            0 => "".to_string(),
            d if d > 0 && d < 10 => format!("{} thousand", ones_digit_names(d)),
            _ => unreachable!()
        };
        let hundreds = match digits[2] {
            0 => "".to_string(),
            d if d > 0 && d < 10 => format!("{} hundred", ones_digit_names(d)),
            _ => unreachable!()
        };
        let tens = match digits[1] {
            0 => "",
            1 => match digits[0] {
                0 => "ten",
                1 => "eleven",
                2 => "twelve",
                3 => "thirteen",
                4 => "fourteen",
                5 => "fifteen",
                6 => "sixteen",
                7 => "seventeen",
                8 => "eighteen",
                9 => "nineteen",
                _ => unreachable!(),
            },
            2 => "twenty",
            3 => "thirty",
            4 => "forty",
            5 => "fifty",
            6 => "sixty",
            7 => "seventy",
            8 => "eighty",
            9 => "ninety",
            _ => unreachable!("Should be < 100"),
        };
        let ones = match digits[1] == 1 {
            true => "".to_string(),
            false => ones_digit_names(digits[0])
        };

        let output = format!("{} {} and {} {}", thousands, hundreds, tens, ones).to_string();
        output.trim().trim_start_matches("and").trim_end_matches("and").trim().to_string()
    }

    fn count_letters(&self) -> usize {
        self.as_words().chars().filter(|c| c != &' ').count()
    }
}

fn ones_digit_names(digit: u8) -> String {
    match digit {
        0 => "",
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        _ => unreachable!("Should be < 10"),
    }.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_words() {
        assert_eq!(22.as_words(), "twenty two".to_string());
        assert_eq!(123.as_words(), "one hundred and twenty three".to_string());
        assert_eq!(517.as_words(), "five hundred and seventeen".to_string());
    }

    #[test]
    fn test_count_letters() {
        assert_eq!(342.count_letters(), 23);
        assert_eq!(115.count_letters(), 20);
    }
}
