use rayon::prelude::*;
use std::collections::HashMap;
use std::ops::Range;
use std::str::FromStr;

advent_of_code::solution!(5);

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub enum MapType {
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}
impl MapType {
    fn all() -> &'static [Self] {
        &[
            Self::SeedToSoil,
            Self::SoilToFertilizer,
            Self::FertilizerToWater,
            Self::WaterToLight,
            Self::LightToTemperature,
            Self::TemperatureToHumidity,
            Self::HumidityToLocation,
        ]
    }
}
impl FromStr for MapType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "seed-to-soil" => Ok(Self::SeedToSoil),
            "soil-to-fertilizer" => Ok(Self::SoilToFertilizer),
            "fertilizer-to-water" => Ok(Self::FertilizerToWater),
            "water-to-light" => Ok(Self::WaterToLight),
            "light-to-temperature" => Ok(Self::LightToTemperature),
            "temperature-to-humidity" => Ok(Self::TemperatureToHumidity),
            "humidity-to-location" => Ok(Self::HumidityToLocation),
            _ => Err(()),
        }
    }
}
type MapLookup = HashMap<MapType, SectionMap>;

#[derive(Debug)]
struct SectionMap {
    _name: String,
    maps: Vec<SeedMap>,
}
impl SectionMap {
    fn map(&self, value: u64) -> u64 {
        let result = self
            .maps
            .iter()
            .filter_map(|m| m.map(value))
            .next()
            .unwrap_or(value);
        result
    }
}
#[derive(Debug)]
struct SeedMap {
    source: Range<u64>,
    offset: i64,
}
impl SeedMap {
    fn parse(line: &str) -> Self {
        let mut nums = line.split(' ').map(|n| n.parse::<u64>().unwrap());
        let dest_start = nums.next().unwrap();
        let source_start = nums.next().unwrap();
        let range_len = nums.next().unwrap();
        let offset = source_start as i64 - dest_start as i64;
        Self {
            source: source_start..(source_start + range_len),
            offset,
        }
    }
    fn map(&self, value: u64) -> Option<u64> {
        if !self.source.contains(&value) {
            return None;
        }
        Some((value as i64 - self.offset) as u64)
    }
}

fn parse_seeds(line: &str) -> Vec<u64> {
    (&line[6..])
        .trim()
        .split(' ')
        .map(|n| n.parse::<u64>().unwrap())
        .collect()
}
fn parse_section(section: &str) -> (MapType, SectionMap) {
    let mut lines = section.lines();
    let name = lines.next().unwrap();
    let name_end = name.find(' ').unwrap();
    let section_name = &name[0..name_end];
    // println!("Section name: {section_name}");
    let maps = lines.map(|l| SeedMap::parse(l)).collect();
    let map_type = MapType::from_str(section_name).unwrap();
    let section = SectionMap {
        _name: section_name.to_string(),
        maps,
    };
    // println!("Section: {section:?}");
    (map_type, section)
}
fn sequence(start: u64, seq: &[MapType], lookup: &MapLookup) -> u64 {
    seq.iter().fold(start, |val, map_type| {
        let map = lookup.get(map_type).unwrap();
        let r = map.map(val);
        // println!("{map_type:?} {val} => {r}");
        r
    })
}
struct Data {
    seeds: Vec<u64>,
    maps: MapLookup,
}
impl Data {
    fn parse(input: &str) -> Self {
        let mut sections = input.trim_end().split("\n\n");
        let seeds = parse_seeds(sections.next().unwrap());
        let mut maps = MapLookup::new();
        for section in sections {
            let (map_type, section) = parse_section(section);
            maps.insert(map_type, section);
        }
        Self { seeds, maps }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    // answer: 240320250
    let data = Data::parse(input);
    let lowest = data
        .seeds
        .into_iter()
        .map(|s| sequence(s, MapType::all(), &data.maps))
        .min()
        .unwrap();
    Some(lowest)
}

pub fn part_two(input: &str) -> Option<u64> {
    // answer: 28580589
    let data = Data::parse(input);
    let seeds: Vec<u64> = data
        .seeds
        .chunks(2)
        .flat_map(|chunk| (chunk[0]..(chunk[0] + chunk[1])).collect::<Vec<u64>>())
        .collect();
    let lowest = seeds
        .par_iter()
        .map(|s| sequence(*s, MapType::all(), &data.maps))
        .min()
        .unwrap();
    Some(lowest)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
