use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, multispace1, newline, space1},
    combinator::map_res,
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(i: &str) -> usize {
    let red_cubes = 12;
    let green_cubes = 13;
    let blue_cubes = 14;
    let mut result = 0;

    let (_, games) = parse_games(i).unwrap();
    // dbg!(&games);
    // dbg!(remainder);

    for (n, game) in games.iter().enumerate() {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        for subset in &game.subsets {
            max_red = subset.red.max(max_red);
            max_green = subset.green.max(max_green);
            max_blue = subset.blue.max(max_blue);
        }

        if max_red <= red_cubes && max_green <= green_cubes && max_blue <= blue_cubes {
            result += n + 1;
        }
    }

    result
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Subset {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug, PartialEq, Clone)]
struct Game {
    subsets: Vec<Subset>,
}

fn parse_games(i: &str) -> IResult<&str, Vec<Game>> {
    let (i, games) = separated_list1(multispace1, parse_game)(i)?;

    Ok((i, games))
}

fn parse_game(i: &str) -> IResult<&str, Game> {
    let (i, (_, _, _, subsets)) = tuple((
        tag("Game "),
        digit1,
        tag(": "),
        separated_list1(tuple((char(';'), space1)), parse_subset),
    ))(i)?;

    Ok((i, Game { subsets }))
}

fn parse_subset(i: &str) -> IResult<&str, Subset> {
    let (i, colors) = separated_list1(tuple((char(','), space1)), parse_1color)(i)?;
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    for (n, c) in colors {
        match c {
            "red" => red = n,
            "green" => green = n,
            "blue" => blue = n,
            _ => (),
        }
    }
    Ok((i, Subset { red, green, blue }))
}

fn parse_1color(i: &str) -> IResult<&str, (u32, &str)> {
    let (i, (d, _, c)) = tuple((
        map_res(digit1, |d: &str| d.parse()),
        space1,
        alt((tag("red"), tag("green"), tag("blue"))),
    ))(i)?;
    Ok((i, (d, c)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_input() {
        let input = include_str!("./example1.txt");
        let result = part1(input);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_parse_subset() {
        let input = "1 red, 2 green, 6 blue";
        let (_, result) = parse_subset(input).unwrap();
        let expected = Subset {
            red: 1,
            green: 2,
            blue: 6,
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_1color() {
        let input = "1 red";
        let (_, result) = parse_1color(input).unwrap();
        let expected = (1, "red");

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_game() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let (_, result) = parse_game(input).unwrap();
        let expected = Game {
            subsets: vec![
                Subset {
                    red: 4,
                    green: 0,
                    blue: 3,
                },
                Subset {
                    red: 1,
                    green: 2,
                    blue: 6,
                },
                Subset {
                    red: 0,
                    green: 2,
                    blue: 0,
                },
            ],
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_games() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let (remainder, result) = parse_games(input).unwrap();
        let expected = vec![
            Game {
                subsets: vec![
                    Subset {
                        red: 4,
                        green: 0,
                        blue: 3,
                    },
                    Subset {
                        red: 1,
                        green: 2,
                        blue: 6,
                    },
                    Subset {
                        red: 0,
                        green: 2,
                        blue: 0,
                    },
                ],
            },
            Game {
                subsets: vec![
                    Subset {
                        red: 4,
                        green: 0,
                        blue: 3,
                    },
                    Subset {
                        red: 1,
                        green: 2,
                        blue: 6,
                    },
                    Subset {
                        red: 0,
                        green: 2,
                        blue: 0,
                    },
                ],
            },
        ];
        dbg!(remainder);

        assert_eq!(result, expected);
    }
}
