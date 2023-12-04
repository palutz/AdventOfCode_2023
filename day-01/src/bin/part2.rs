const name_to_ num : Vec<(&str, &str>)> =
    vec![
        ("uno", "1"),
    ];

fn part2(input: &str) -> i32 {
    let lines = input.trim().split('\n');
    let total: i32 = lines
                        .map(extract_integers)
                        .sum();

    total
}

fn main() {
    let input = include_str!("./input01.txt");
    let res2 = part2(input);
    println!("{res1}");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {}
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let s1 = "two1nine";
        let actual = extract_integers(s1);
        assert_eq!(actual, 12);

        // let s2 = "eightwothree";
        // let s3 = "abcone2threexyz";
        // let s4 = "xtwone3four";
        // let s5 = "4nineeightseven2";
        // let s6 = "zoneight234";
        // let s7 = "7pqrstsixteen";
    }
}
