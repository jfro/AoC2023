use std::collections::HashMap;
use std::ops::Range;
use std::str::FromStr;
advent_of_code::solution!(5);

pub struct Seed(pub u64);
// pub struct Soil(pub u32);
// pub struct Fertilizer(pub u32);
// pub struct Water(pub u32);
// pub struct Light(pub u32);
// pub struct Temperature(pub u32);
// pub struct Humidity(pub u32);
// pub struct Location(pub u32);

// pub type SeedToSoil = HashMap<Seed, Soil>;
// pub type SoilToFertilizer = HashMap<Soil, Fertilizer>;
// pub type FertilizerToWater = HashMap<Fertilizer, Water>;
// pub type WaterToLight = HashMap<Water, Light>;
// pub type LightToTemperature = HashMap<Light, Temperature>;
// pub type TemperatureToHumidity = HashMap<Temperature, Humidity>;
// pub type HumidityToLocation = HashMap<Humidity, Location>;

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
pub type MapLookup = HashMap<MapType, SectionMap>;

#[derive(Debug)]
struct SectionMap {
    name: String,
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
    destination: Range<u64>,
}
impl SeedMap {
    fn parse(line: &str) -> Self {
        let mut nums = line.split(' ').map(|n| n.parse::<u64>().unwrap());
        let dest_start = nums.next().unwrap();
        let source_start = nums.next().unwrap();
        let range_len = nums.next().unwrap();
        Self {
            source: source_start..(source_start + range_len),
            destination: dest_start..(dest_start + range_len),
        }
    }
    fn map(&self, value: u64) -> Option<u64> {
        let index = self.source.clone().position(|s| s == value)?;
        self.destination.clone().nth(index)
    }
}

fn parse_seeds(line: &str) -> Vec<Seed> {
    (&line[6..])
        .trim()
        .split(' ')
        .map(|n| Seed(n.parse::<u64>().unwrap()))
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
        name: section_name.to_string(),
        maps,
    };
    // println!("Section: {section:?}");
    (map_type, section)
}
fn sequence(start: u64, seq: &[MapType], lookup: &MapLookup) -> u64 {
    seq.iter().fold(start, |val, map_type| {
        let map = lookup.get(map_type).unwrap();
        let r = map.map(val);
        println!("{map_type:?} {val} => {r}");
        r
    })
}
pub fn part_one(input: &str) -> Option<u64> {
    let mut sections = input.trim_end().split("\n\n");
    let seeds = parse_seeds(sections.next().unwrap());
    let mut section_lookup = MapLookup::new();
    for section in sections {
        let (map_type, section) = parse_section(section);
        section_lookup.insert(map_type, section);
    }
    // let soil = section_lookup.get(&MapType::SeedToSoil).unwrap();
    // let r = soil.map(79);
    // println!("79 -> {r}");
    let lowest = seeds
        .into_iter()
        .map(|s| sequence(s.0, MapType::all(), &section_lookup))
        .min()
        .unwrap();
    // let result = sequence(79, MapType::all(), &section_lookup);
    // println!("Result: {result}");
    Some(lowest)
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
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
