advent_of_code::solution!(6);

// Charge time: 1mm per 1ms
#[derive(Debug, Copy, Clone)]
struct Time(u32); // ms
#[derive(Debug, Copy, Clone)]
struct Speed(u32); // mm per ms
#[derive(Debug, Copy, Clone)]
struct Distance(u32); // mm
impl Time {
    pub fn charge_to_speed(&self) -> Speed {
        // since it's 1mm per 1ms of charge, speed is same as time, just diff units
        Speed(self.0)
    }
}
impl Speed {
    pub fn distance_for_speed(&self, time: Time) -> Distance {
        Distance(time.0 * self.0)
    }
}
#[derive(Debug)]
struct Race {
    /// Farthest distance a boat went in this race
    record: Distance,
    /// Total time of race
    time: Time,
}
impl Race {
    pub fn new(record: u32, time: u32) -> Self {
        Race {
            record: Distance(record),
            time: Time(time),
        }
    }
    // pub fn possible_charge_times(&self) -> Vec<Time> {
    //     let time_range = 1..(self.time.0 - 1); // all but 0 and max time
    //     time_range.map(|t| Time(t)).collect()
    // }
    pub fn attempts(&self) -> Vec<RaceAttempt> {
        let time_range = 1..self.time.0; // all but 0 and max time
        time_range
            .map(|t| {
                let charge = Time(t);
                let distance = Time(self.time.0 - t);
                RaceAttempt { charge, distance }
            })
            .collect()
    }
}
/// Describes an attempt split between how long charge & how long for travel
#[derive(Debug)]
struct RaceAttempt {
    /// Time button held for charging
    charge: Time,
    /// Time left spent traveling
    distance: Time,
}
impl RaceAttempt {
    pub fn distance(&self) -> Distance {
        let speed = self.charge.charge_to_speed();
        speed.distance_for_speed(self.distance)
    }
}

fn parse_numbers(line: &str) -> Vec<u32> {
    let colon = line.find(':').unwrap();
    (&line[colon..])
        .trim()
        .split(' ')
        .filter_map(|n| n.parse::<u32>().ok())
        .collect()
}
fn parse(input: &str) -> Vec<Race> {
    let mut lines = input.lines();
    let time = parse_numbers(lines.next().unwrap());
    let distance = parse_numbers(lines.next().unwrap());
    time.into_iter()
        .zip(distance.into_iter())
        .map(|(t, d)| Race::new(d, t))
        .collect()
}
pub fn part_one(input: &str) -> Option<u32> {
    let races = parse(input);
    println!("Races: {races:?}");
    let mut total = 1;
    for race in races {
        let wins = race
            .attempts()
            .into_iter()
            .map(|a| a.distance())
            .filter(|a| a.0 > race.record.0)
            .count();
        println!("Wins: {wins}");
        total *= wins;
    }
    Some(total as _)
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
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
