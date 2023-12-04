advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let nums: u32 = input
        .trim_end()
        .split('\n')
        .map(|line| {
            let num = parse_line_num(line);
            // println!("Num: {num}");
            num
        })
        .sum();
    // println!("Nums: {nums:?}");
    Some(nums)
}
const NUM_WORDS: &[&str; 9] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
pub fn part_two(input: &str) -> Option<u32> {
    let nums: u32 = input
        .trim_end()
        .split('\n')
        .map(|line| {
            let num = parse_line_text(line);
            // println!("Num: {num}");
            num
        })
        .sum();
    // println!("Nums: {nums:?}");
    Some(nums)
}
struct NumMatch {
    start: usize,
    num: u8,
}
fn parse_line_text(line: &str) -> u32 {
    let mut finds = Vec::new();
    // find first digit if any
    if let Some(first_digit) = line
        .chars()
        .enumerate()
        .filter(|(_i, c)| c.is_ascii_digit())
        .next()
        .map(|(i, d)| NumMatch {
            start: i,
            num: d.to_string().parse::<u8>().unwrap(),
        })
    {
        finds.push(first_digit)
    }
    if let Some(last_digit) = line
        .chars()
        .enumerate()
        .filter(|(_i, c)| c.is_ascii_digit())
        .last()
        .map(|(i, d)| NumMatch {
            start: i,
            num: d.to_string().parse::<u8>().unwrap(),
        })
    {
        finds.push(last_digit)
    }
    // find first num word if any
    for (num, string) in NUM_WORDS.iter().enumerate() {
        if let Some(index) = line.find(string) {
            finds.push(NumMatch {
                start: index,
                num: (num + 1) as u8,
            });
        }
    }
    // find last num word if any
    for (num, string) in NUM_WORDS.iter().enumerate() {
        if let Some(index) = line.rfind(string) {
            finds.push(NumMatch {
                start: index,
                num: (num + 1) as u8,
            });
        }
    }
    let min = finds
        .iter()
        .min_by(|a1, a2| a1.start.cmp(&a2.start))
        .unwrap()
        .num;
    let max = finds
        .iter()
        .max_by(|a1, a2| a1.start.cmp(&a2.start))
        .unwrap()
        .num;
    format!("{min}{max}").parse().unwrap()
}
fn parse_line_num(line: &str) -> u32 {
    // println!("Line: {line}");
    let digits = line
        .chars()
        .filter(|d| d.is_ascii_digit())
        .collect::<Vec<char>>();
    let first = digits.first().unwrap();
    let last = digits.last().unwrap();
    let num = &[*first, *last].into_iter().collect::<String>();
    num.parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY, None));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file(
            "examples",
            DAY,
            Some(2),
        ));
        assert_eq!(result, Some(281));
    }
    #[test]
    fn test_translate() {
        let input = "onetwothreefourfivesixseveneightnine";
        let r = translate_numbers(input);
        assert_eq!(r.as_str(), "123456789");
    }
}
