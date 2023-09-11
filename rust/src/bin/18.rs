use std::{num::ParseIntError, str::FromStr};

/// By starting at the top of the triangle below and moving to adjacent numbers on the row below, the maximum total from top to bottom is 23.

///    3
///   7 4
///  2 4 6
/// 8 5 9 3

/// That is, 3 + 7 + 4 + 9 = 23.

/// Find the maximum total from top to bottom of the triangle below:

///               75
///              95 64
///             17 47 82
///            18 35 87 10
///           20 04 82 47 65
///          19 01 23 75 03 34
///         88 02 77 73 07 63 67
///        99 65 04 28 06 16 70 92
///       41 41 26 56 83 40 80 70 33
///      41 48 72 33 47 32 37 16 94 29
///     53 71 44 65 25 43 91 52 97 51 14
///    70 11 33 28 77 73 17 78 39 68 17 57
///   91 71 52 38 17 14 91 43 58 50 27 29 48
///  63 66 04 68 89 53 67 30 73 16 69 87 40 31
/// 04 62 98 27 23 09 70 98 73 93 38 53 60 04 23

const BIG_TRIANGLE: &str = "
               75
              95 64
             17 47 82
            18 35 87 10
           20 04 82 47 65
          19 01 23 75 03 34
         88 02 77 73 07 63 67
        99 65 04 28 06 16 70 92
       41 41 26 56 83 40 80 70 33
      41 48 72 33 47 32 37 16 94 29
     53 71 44 65 25 43 91 52 97 51 14
    70 11 33 28 77 73 17 78 39 68 17 57
   91 71 52 38 17 14 91 43 58 50 27 29 48
  63 66 04 68 89 53 67 30 73 16 69 87 40 31
 04 62 98 27 23 09 70 98 73 93 38 53 60 04 23
";

fn main() {
    let triangle = Triangle::from_str(BIG_TRIANGLE).unwrap();
    println!("{}", triangle.max_vertical_total());
}

struct Triangle {
    rows: Vec<Vec<usize>>,
}

impl FromStr for Triangle {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rows: Vec<Vec<usize>> = Vec::new();
        for line in s.trim().lines() {
            let mut row: Vec<usize> = Vec::new();
            for cell in line.split_whitespace() {
                row.push(cell.parse::<usize>()?);
            }
            rows.push(row);
        }
        Ok(Triangle { rows })
    }
}

impl std::ops::Index<usize> for Triangle {
    type Output = Vec<usize>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.rows[index]
    }
}

impl Triangle {
    fn max_vertical_total(&self) -> usize {
        let mut collapsed = vec![self.rows[0].clone()];
        for (idx, row) in self.rows[1..].iter().enumerate() {
            let prev_row = &collapsed[idx];
            let mut this_collapsed_row = vec![];
            for (inner_idx, cell) in row.iter().enumerate() {
                let max_choice = match inner_idx {
                    0 => prev_row[inner_idx],
                    _ if inner_idx < row.len() - 1 => {
                        prev_row[inner_idx - 1].max(prev_row[inner_idx])
                    }
                    _ => prev_row[inner_idx - 1],
                };
                this_collapsed_row.push(cell + max_choice);
            }
            collapsed.push(this_collapsed_row);
        }
        *collapsed.last().unwrap().iter().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_TRIANGLE: &str = "
       3
      7 4
     2 4 6
    8 5 9 3
    ";

    #[test]
    fn test_string_to_triangle() {
        let actual = Triangle::from_str(SMALL_TRIANGLE).expect("unit test");
        assert_eq!(actual[0], vec![3]);
        assert_eq!(actual[1], vec![7, 4]);
        assert_eq!(actual[2], vec![2, 4, 6]);
        assert_eq!(actual[3], vec![8, 5, 9, 3]);
    }

    #[test]
    fn test_maximum_vertical_total() {
        let actual = Triangle::from_str(SMALL_TRIANGLE).expect("unit test");
        assert_eq!(actual.max_vertical_total(), 23);
    }
}
