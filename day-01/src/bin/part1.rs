fn extract_integers(s: &str) -> i32 {
    let digits: Vec<char> = s.chars().filter(|c| c.is_ascii_digit()).collect();
    let lv = digits.len();
    let mut numb: Vec<char> = vec![];
    match lv {
        0 => numb.push('0'),
        1 => {
            numb.push(digits[0]);
            numb.push(digits[0]);
        }
        _ => {
            numb.push(digits[0]);
            numb.push(digits[lv - 1]);
        }
    };
    let str_num: String = numb.into_iter().collect();
    str_num.parse::<i32>().unwrap()
}

fn part1(input: &str) -> i32 {
    let lines = input.trim().split('\n');
    let total: i32 = lines.map(extract_integers).sum();

    total
}

fn main() {
    let input = include_str!("./input01.txt");
    let res1 = part1(input);
    println!("{res1}");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let s1 = "1abc2";
        let actual = extract_integers(s1);
        assert_eq!(actual, 12);
        let s2 = "pqr3stu8vwx";
        let a2 = extract_integers(s2);
        assert_eq!(a2, 38);
        let s3 = "a1b2c3d4e5f";
        let a3 = extract_integers(s3);
        assert_eq!(a3, 15);
        let s4 = "treb7uchet";
        let a4 = extract_integers(s4);
        assert_eq!(a4, 77);
        let s5 = "trebuchet";
        let a5 = extract_integers(s5);
        assert_eq!(a5, 0);
    }
}
