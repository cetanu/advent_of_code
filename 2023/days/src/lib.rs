pub fn day_1(lines: Vec<String>) -> i32 {
    let mut sum = 0;
    for line in lines {
        let mut first = None;
        let mut last = None;

        line.chars().filter(|c| c.is_ascii_digit()).for_each(|n| {
            if first.is_none() {
                first = Some(n);
            }
            last = Some(n);
        });

        let digit = format!("{}{}", first.unwrap(), last.unwrap())
            .parse::<i32>()
            .unwrap_or_default();

        sum += digit;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;

    fn read_input<P>(filename: P) -> io::Result<Vec<String>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        let lines = io::BufReader::new(file).lines();
        lines.collect()
    }

    #[test]
    fn day_one() {
        let example = vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"]
            .into_iter()
            .map(String::from)
            .collect();
        assert_eq!(day_1(example), 142);

        let input = read_input("inputs/day_1.txt").unwrap();
        let answer = day_1(input);
        assert_eq!(answer, 55712);
    }
}
