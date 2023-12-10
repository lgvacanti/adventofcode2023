fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> i32 {
    let mut result = 0;

    let height = dbg!(input.lines().count());
    let width = dbg!(input.lines().next().unwrap().trim().len());

    let map = Map::new(
        width,
        height,
        input.chars().filter(|&c| !c.is_whitespace()).collect(),
    );

    //dbg!(&map.m);

    let mut numbers: Vec<LinearMapSection> = Vec::new();

    // Collect numbers

    for nrow in 0..map.height {
        let row = &map.m[nrow * map.width..nrow * map.width + map.width];
        //dbg!(row);

        let mut start: i32 = -1;
        let mut end: i32 = -1;
        for (i, c) in row.iter().enumerate() {
            // map index = nrow * width + i
            let mapindex = nrow * width + i;
            if c.is_numeric() && start == -1 {
                start = i32::try_from(mapindex).unwrap();
                end = i32::try_from(mapindex).unwrap();
            } else if mapindex % width == width - 1 && start != -1 {
                // end of row
                if c.is_numeric() {
                    end = i32::try_from(mapindex).unwrap();
                }
                numbers.push(LinearMapSection {
                    start: usize::try_from(start).unwrap(),
                    end: usize::try_from(end).unwrap(),
                });
                start = -1;
                end = -1;
            } else if c.is_numeric() && start != -1 {
                end = i32::try_from(mapindex).unwrap();
            } else if !c.is_numeric() && start != -1 {
                numbers.push(LinearMapSection {
                    start: usize::try_from(start).unwrap(),
                    end: usize::try_from(end).unwrap(),
                });
                start = -1;
                end = -1;
            }

            if mapindex < 6020 && mapindex > 5980 {
                dbg!(c);
                dbg!(start);
                dbg!(end);
            }
        }
    }

    dbg!(&numbers);

    // Check if numbers are part numbers
    for number in numbers {
        let mut is_partnumber = false;
        // get surroundings
        let borders = number.borders(&map);

        // check is surroundings contain a part
        for location in borders {
            if map.m[location] != '.' {
                is_partnumber = true;
            }
        }

        if is_partnumber {
            let x: i32 = String::from_iter(map.m[number.start..=number.end].iter())
                .parse()
                .unwrap();
            result += dbg!(x);
        }
    }

    result
}

#[derive(Debug)]
struct Map<T> {
    width: usize,
    height: usize,
    m: Vec<T>,
}

impl<T> Map<T>
where
    T: Clone + Copy,
{
    fn new(width: usize, height: usize, vec: Vec<T>) -> Map<T> {
        assert_eq!(vec.len(), width * height);
        Map {
            width,
            height,
            m: vec,
        }
    }

    fn get(self, x: usize, y: usize) -> T {
        self.m[y * self.width + x]
    }

    fn len(&self) -> usize {
        self.width * self.height
    }
}

#[derive(Debug, Clone, Copy)]
struct LinearMapSection {
    start: usize,
    end: usize,
}

impl LinearMapSection {
    fn new(start: usize, end: usize) -> LinearMapSection {
        assert! {start <= end}
        LinearMapSection { start, end }
    }

    fn fits_in(self, map: &Map<impl Clone + Copy>) -> bool {
        self.end < map.len()
    }

    /// returns the bordering positions, diagonal included
    fn borders(self, map: &Map<impl Clone + Copy>) -> Vec<usize> {
        let mut borders = Vec::new();

        let touches_left = self.start % map.width == 0;
        let touches_right = self.end % map.width == map.width - 1;
        let touches_top = self.start < map.width;
        let touches_bottom = self.start >= map.width * (map.height - 1);

        if !touches_left {
            borders.push(self.start - 1);
        }
        if !touches_right {
            borders.push(self.end + 1);
        }

        if !touches_top {
            borders.extend((self.start - map.width..=self.end - map.width).collect::<Vec<usize>>());
        }

        if !touches_bottom {
            borders.extend((self.start + map.width..=self.end + map.width).collect::<Vec<usize>>());
        }

        if !touches_left && !touches_top {
            borders.push(self.start - map.width - 1);
        }

        if !touches_right && !touches_top {
            borders.push(self.end - map.width + 1);
        }

        if !touches_left && !touches_bottom {
            borders.push(self.start + map.width - 1);
        }

        if !touches_right && !touches_bottom {
            borders.push(self.end + map.width + 1);
        }

        borders
    }

    fn print_contents(self, map: &Map<char>) -> String {
        String::from_iter(map.m[self.start..=self.end].iter())
            .parse()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_input() {
        let input = include_str!("./example.txt");
        let result = part1(input);
        assert_eq!(result, 4361);
    }

    #[test]
    fn test_706() {
        let input = include_str!("./input.txt");
        let section = LinearMapSection {
            start: 5998,
            end: 5999,
        };

        let height = dbg!(input.lines().count());
        let width = dbg!(input.lines().next().unwrap().trim().len());

        let map = Map::new(
            width,
            height,
            input.chars().filter(|&c| !c.is_whitespace()).collect(),
        );

        assert_eq!(section.print_contents(&map), "706");
    }
}
