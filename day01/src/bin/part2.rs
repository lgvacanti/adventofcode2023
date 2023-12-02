fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> u32 {
    let mut result = 0;
    for line in input.lines() {
        let mut min_index = line.find(char::is_numeric).unwrap_or(usize::MAX);
        let mut min_digit: &str = &line.chars().nth(min_index).unwrap_or_default().to_string();

        for wdigit in [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ] {
            if let Some(i) = line.find(wdigit) {
                if i < min_index {
                    min_index = i;
                    min_digit = wdigit;
                }
            }
        }

        result += 10 * parse_digit(min_digit);

        let mut max_index = line.rfind(char::is_numeric).unwrap_or(usize::MIN);
        let mut max_digit: &str = &line.chars().nth(max_index).unwrap_or_default().to_string();

        for wdigit in [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ] {
            if let Some(i) = line.rfind(wdigit) {
                if i > max_index {
                    max_index = i;
                    max_digit = wdigit;
                }
            }
        }

        result += parse_digit(max_digit);
    }
    result
}

fn parse_digit(str_digit: &str) -> u32 {
    match str_digit {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => str_digit.chars().next().unwrap().to_digit(10).unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_input() {
        let input = include_str!("./example2.txt");
        let result = part2(input);
        assert_eq!(result, 281);
    }

    #[test]
    fn parse_digit_numeric() {
        assert_eq!(parse_digit("7"), 7)
    }

    #[test]
    fn parse_digit_word() {
        assert_eq!(parse_digit("seven"), 7)
    }
}
