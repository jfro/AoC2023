use std::collections::BTreeSet;

advent_of_code::solution!(4);

#[derive(Debug, Clone)]
struct Card {
    num: u32,
    winning: BTreeSet<u32>,
    numbers: BTreeSet<u32>,
    copies: u32,
}
impl Card {
    pub fn parse(line: &str) -> Self {
        let mut data = line.split(':');
        let num = data
            .next()
            .unwrap()
            .split(' ')
            .filter(|p| !p.is_empty())
            .skip(1)
            .next()
            .unwrap()
            .parse::<u32>()
            .unwrap();

        let mut numbers = data.next().unwrap().split(" | ");
        let winning = parse_numbers(numbers.next().unwrap());
        let numbers = parse_numbers(numbers.next().unwrap());
        Self {
            num,
            winning,
            numbers,
            copies: 1,
        }
    }
    pub fn points(&self) -> u32 {
        let matches = self.wins() as u32;
        if matches == 0 {
            return 0;
        }
        2u32.pow(matches - 1)
    }
    pub fn wins(&self) -> usize {
        self.winning.intersection(&self.numbers).count()
    }
}
fn parse_numbers(nums: &str) -> BTreeSet<u32> {
    nums.trim()
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u32>().unwrap())
        .collect()
}
pub fn part_one(input: &str) -> Option<u32> {
    let sum = input
        .trim_end()
        .lines()
        .map(|l| {
            let card = Card::parse(l);
            card.points()
        })
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut cards: Vec<Card> = input.trim_end().lines().map(|l| Card::parse(l)).collect();
    for i in 0..cards.len() {
        let (num, copies, wins) = {
            let card = &cards[i];
            let wins = card.wins();
            (card.num, card.copies, wins)
        };
        let new_cards = cards.iter_mut().skip(num as _).take(wins);
        for copy in new_cards {
            copy.copies += copies * 1;
        }
    }
    Some(cards.iter().map(|c| c.copies).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
