fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> u32 {
    let mut result = 0;
    for line in input.lines() {
        // first digit
        for c in line.chars() {
            if c.is_numeric() {
                result += 10 * c.to_digit(10).unwrap();
                break;
            }
        }
        // last digit
        for c in line.chars().rev() {
            if c.is_numeric() {
                result += c.to_digit(10).unwrap();
                break;
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_input() {
        let input = include_str!("./example1.txt");
        let result = part1(input);
        assert_eq!(result, 142);
    }
}
