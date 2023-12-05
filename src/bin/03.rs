use std::collections::HashMap;
use std::fmt;

advent_of_code::solution!(3);

enum Cardinal {
    NorthWest,
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
}
impl Cardinal {
    pub fn all() -> &'static [Self] {
        &[
            Self::NorthWest,
            Self::North,
            Self::NorthEast,
            Self::East,
            Self::SouthEast,
            Self::South,
            Self::SouthWest,
            Self::West,
        ]
    }
}
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Element {
    Part(u32),
    Char(char),
    Empty,
}
impl Element {
    pub fn new(c: char) -> Self {
        if c == '.' {
            Self::Empty
        } else {
            Self::Char(c)
        }
    }
    pub fn is_digit(&self) -> bool {
        match self {
            Self::Char(c) => c.is_ascii_digit(),
            Self::Part(_) => false,
            Self::Empty => false,
        }
    }
    pub fn is_symbol(&self) -> bool {
        let valid = &['#', '*', '+', '$'];
        match self {
            Self::Char(c) => !c.is_alphanumeric() || valid.contains(c),
            Self::Part(_) => false,
            Self::Empty => false,
        }
    }
    pub fn is_spacer(&self) -> bool {
        match self {
            Self::Char(_c) => false,
            Self::Part(_) => false,
            Self::Empty => true,
        }
    }
    pub fn is_part(&self) -> bool {
        matches!(self, Self::Part(_))
    }
    pub fn digit(&self) -> Option<char> {
        match self {
            Self::Char(c) if self.is_digit() => Some(*c),
            _ => None,
        }
    }
    pub fn part(&self) -> Option<u32> {
        match self {
            Self::Part(part) => Some(*part),
            _ => None,
        }
    }
}
struct Engine {
    map: HashMap<(isize, isize), Element>,
    rows: isize,
    cols: isize,
}
impl Engine {
    pub fn parse(input: &str) -> Self {
        let mut map = HashMap::new();
        for (row, line) in input.trim_end().lines().enumerate() {
            for (col, char) in line.chars().enumerate() {
                map.insert((row as isize, col as isize), Element::new(char));
            }
        }
        let (rows, cols) = map
            .keys()
            .fold((0isize, 0isize), |(total_rows, total_cols), (row, col)| {
                (total_rows.max(*row), total_cols.max(*col))
            });
        let mut engine = Engine {
            map,
            rows: rows + 1,
            cols: cols + 1,
        };
        engine.update_numbers();
        engine
    }
    fn update_numbers(&mut self) {
        for row in 0..self.rows {
            let mut locations = Vec::with_capacity(3);
            let mut current_num = Vec::with_capacity(3);
            for col in 0..self.cols {
                {
                    let val = self.map.get(&(row, col)).unwrap();
                    if let Some(c) = val.digit() {
                        current_num.push(c);
                        locations.push((row, col));
                    } else {
                        if !current_num.is_empty() {
                            let num = current_num
                                .into_iter()
                                .collect::<String>()
                                .parse::<u32>()
                                .unwrap();
                            current_num = Vec::with_capacity(3);
                            for loc in locations.into_iter() {
                                let el = self.map.get_mut(&loc).unwrap();
                                *el = Element::Part(num)
                            }
                            locations = Vec::with_capacity(3);
                        }
                    }
                }
            }
            if !current_num.is_empty() {
                let num = current_num
                    .into_iter()
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap();
                for loc in locations.into_iter() {
                    let el = self.map.get_mut(&loc).unwrap();
                    *el = Element::Part(num)
                }
            }
        }
    }
    pub fn has_symbol(&self, location: (isize, isize)) -> bool {
        let adjs = self.adjacent(location);
        let symbols: Vec<Element> = adjs.into_iter().filter(|s| s.is_symbol()).collect();
        // println!("Symbols: {symbols:?}");
        !symbols.is_empty()
    }
    pub fn adjacent(&self, location: (isize, isize)) -> Vec<Element> {
        Cardinal::all()
            .iter()
            .filter_map(|c| self.adjacent_coordinate(location, c))
            .filter(|s| !s.is_spacer())
            .collect()
    }
    fn adjacent_coordinate(
        &self,
        location: (isize, isize),
        cardinal: &Cardinal,
    ) -> Option<Element> {
        let offset: (isize, isize) = match cardinal {
            Cardinal::NorthWest => (-1, -1), // -1, -1
            Cardinal::North => (-1, 0),      // -1, 0
            Cardinal::NorthEast => (-1, 1),  // -1, 1
            Cardinal::East => (0, 1),        // 0, 1
            Cardinal::SouthEast => (1, 1),   // 1, 1
            Cardinal::South => (1, 0),       // 1, 0
            Cardinal::SouthWest => (1, -1),  // 1, -1
            Cardinal::West => (0, -1),       // 0, -1
        };
        let result = (location.0 + offset.0, location.1 + offset.1);
        if result.0 < 0 || result.0 > self.rows || result.1 < 0 || result.1 > self.cols {
            return None;
        }
        self.map.get(&result).map(|s| s.to_owned())
    }
}
impl fmt::Debug for Engine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{} x {}", self.rows, self.cols)?;
        for row in 0..self.rows {
            write!(f, "{}: ", row)?;
            for col in 0..self.cols {
                let val = self.map.get(&(row, col)).unwrap();
                write!(f, "{val:?} ")?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let engine = Engine::parse(input);
    println!("Engine: {:?}", engine);
    // let value = engine.map.get(&(0, 2)).unwrap();
    // let check = engine.adjacent((0, 2));
    // let has_symbol = engine.has_symbol((0, 2));
    // println!("{value:?} neighbors: {check:?} - {has_symbol}");
    // TODO: make iterator?
    let mut valid_parts = Vec::new();
    // let mut possible_parts = Vec::new();
    for row in 0..engine.rows {
        let mut current_part = None;
        let mut added_part = false;
        for col in 0..engine.cols {
            let loc = (row, col);
            let el = engine.map.get(&loc).unwrap();
            if el.is_part() {
                println!("Checking around part {el:?}");
            }
            if el.is_part() && current_part.as_ref() != Some(el) {
                added_part = false;
                if engine.has_symbol(loc) && el.is_part() {
                    valid_parts.push(*el);
                    added_part = true;
                }
                current_part = Some(*el);
            } else if engine.has_symbol(loc) && el.is_part() && !added_part {
                valid_parts.push(*el);
                added_part = true;
            }
        }
    }
    println!("Parts: {valid_parts:?}");
    Some(valid_parts.into_iter().filter_map(|p| p.part()).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
