const NAMENUM : [(&'static str, &'static str); 10] = [
    ("zero", "0"),
    ("one", "1"),
    ("two", "2"),
    ("three", "3"),
    ("four", "4"),
    ("five", "5"),
    ("six", "6"),
    ("seven", "7"),
    ("eight", "8"),
    ("nine", "9"),
];

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


fn words_to_num(mut row: String) -> String {
    let mut finding : Vec<(i32, &str, &str)> = vec!();
    let r2 = row.clone();   // need to clone it for the match_indices reference lifetime
    for nn in NAMENUM {     // find all the occurrences of every single string in the vector
        let v : Vec<(usize, &str)> = r2.as_str().match_indices(nn.0).collect();
        finding.append(&mut v.iter().map(|elem| (elem.0 as i32, elem.1, nn.1)).collect());
    }

    //dbg!(&finding);
    if finding.len() >=2 {
        finding.sort_by_key(|k| k.0);
        // I need only the last 2 elements in any case
        let first = finding.first().unwrap();
        let last = finding.last().unwrap();
        // dbg!(&first);
        // dbg!(&last);
        // if the 2 numbers are overlapping, I will create a range that is the combination of the 2
        // strings
        if first.0 + (first.1.len() as i32) > last.0 {
            let idx = first.0 as usize;
            let idxlast = last.0 as usize;
            let part1 = &row.as_str()[..idx];
            let part2 = &row.as_str()[(idxlast + last.1.len()) ..];
            row = [part1, first.2, last.2, part2].concat();
        } else {
            row = row.replace(first.1, first.2);
            row = row.replace(last.1, last.2);
        }
    } else {
        if finding.len() == 1 {
            row = row.replace(finding[0].1, finding[0].2);
        }
    }
    row.to_string()
}

fn part2(input: &str) -> i32 {
    let lines = input.trim().split('\n');
    let l2 = lines.map(|row| words_to_num(row.to_string()));
    let total: i32 = l2
                    .map(|row2| extract_integers(row2.as_str()))
                    .sum();

    total
}

fn main() {
    let input = include_str!("./input01.txt");
    let res2 = part2(input);
    println!("{res2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s0 = "oneight";
        let actual = part2(s0);
        assert_eq!(actual, 18);

        let s1 = "two1nine";
        let actual = part2(s1);
        assert_eq!(actual, 29);

        let s2 = "eightwothree";
        let actual = part2(s2);
        assert_eq!(actual, 83);

        let s3 = "abcone2threexyz";
        let actual = part2(s3);
        assert_eq!(actual, 13);

        let s4 = "xtwone3four";
        let actual = part2(s4);
        assert_eq!(actual, 24);

        let s5 = "4nineeightseven2";
        let actual = part2(s5);
        assert_eq!(actual, 42);

        let s6 = "zoneight234";
        let actual = part2(s6);
        assert_eq!(actual, 14);

        let s7 = "7pqrstsixteen";
        let actual = part2(s7);
        assert_eq!(actual, 76);

        let s8 = "bhfhszrhzgrhsfd2threeseventwosevenoneseven";
        let actual = part2(s8);
        assert_eq!(actual, 27);
    }
}
