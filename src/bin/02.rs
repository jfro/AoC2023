advent_of_code::solution!(2);
use anyhow::{Error, Result};
use std::iter::Sum;
use std::ops::Add;

#[derive(Debug, Default, Copy, Clone)]
struct Set {
    red: u8,
    green: u8,
    blue: u8,
}
impl Add for Set {
    type Output = Set;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}
impl Sum for Set {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::default(), |sum, s| sum + s)
    }
}
#[derive(Debug)]
struct Game {
    num: u8,
    sets: Vec<Set>,
}
impl Game {
    pub fn is_valid(&self, max: Set) -> bool {
        for set in self.sets.iter() {
            if set.green > max.green || set.red > max.red || set.blue > max.blue {
                return false;
            }
        }
        true
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    let max = Set {
        red: 12,
        green: 13,
        blue: 14,
    };
    let lines = input.trim_end().split('\n');
    let games = lines
        .map(|l| parse_line(l).unwrap())
        .filter(|g| g.is_valid(max))
        .map(|g| g.num as u32)
        .sum();
    Some(games)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.trim_end().split('\n');
    let sum = lines
        .map(|l| parse_line(l).unwrap())
        .map(|game| {
            let max_red = game.sets.iter().map(|s| s.red).max().unwrap();
            let max_green = game.sets.iter().map(|s| s.green).max().unwrap();
            let max_blue = game.sets.iter().map(|s| s.blue).max().unwrap();
            let power = max_red as u32 * max_green as u32 * max_blue as u32;
            power
        })
        .sum();
    Some(sum)
}

fn parse_line(line: &str) -> Result<Game> {
    let mut pieces = line.split(':');
    let num: u8 = pieces
        .next()
        .ok_or(Error::msg("Invalid data on : split"))?
        .split(' ')
        .skip(1)
        .next()
        .ok_or(Error::msg("Invalid data ' ' split"))?
        .parse()?;
    let game_details = pieces.next().ok_or(Error::msg("Invalid data"))?;
    let sets = parse_details(game_details);
    Ok(Game { num, sets })
}

fn parse_details(details: &str) -> Vec<Set> {
    details.split(';').map(|s| parse_set(s)).collect()
}
fn parse_set(set: &str) -> Set {
    let elements = set.split(',').map(|s| s.trim()).map(|s| {
        let mut set = Set::default();
        let mut pieces = s.split(' ');
        let num = pieces.next().unwrap().parse::<u8>().unwrap();
        match pieces.next().unwrap() {
            "red" => set.red += num,
            "green" => set.green += num,
            "blue" => set.blue += num,
            s => panic!("Invalid color: {s}"),
        }
        set
    });
    elements.sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
