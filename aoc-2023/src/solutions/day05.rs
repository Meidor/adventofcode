use std::{ops::Range, str::FromStr};

use color_eyre::eyre::Result;
use itertools::Itertools;

#[derive(Debug)]
struct Almanac {
    seed_to_soil: AlmanacMap,
    soil_to_fertilizer: AlmanacMap,
    fertilizer_to_water: AlmanacMap,
    water_to_light: AlmanacMap,
    light_to_temperature: AlmanacMap,
    temperature_to_humidity: AlmanacMap,
    humidity_to_location: AlmanacMap,
}

impl Almanac {
    pub fn seed_to_location(&self, seed: usize) -> usize {
        let soil = self.seed_to_soil.convert(seed);
        let fertilizer = self.soil_to_fertilizer.convert(soil);
        let water = self.fertilizer_to_water.convert(fertilizer);
        let light = self.water_to_light.convert(water);
        let temperature = self.light_to_temperature.convert(light);
        let humidity = self.temperature_to_humidity.convert(temperature);
        self.humidity_to_location.convert(humidity)
    }
}

impl FromStr for Almanac {
    type Err = color_eyre::eyre::Error;

    fn from_str(s: &str) -> Result<Self> {
        let parts: Vec<&str> = s.split("\n\n").collect();
        let seed_to_soil: AlmanacMap = parts[1].parse()?;
        let soil_to_fertilizer: AlmanacMap = parts[2].parse()?;
        let fertilizer_to_water: AlmanacMap = parts[3].parse()?;
        let water_to_light: AlmanacMap = parts[4].parse()?;
        let light_to_temperature: AlmanacMap = parts[5].parse()?;
        let temperature_to_humidity: AlmanacMap = parts[6].parse()?;
        let humidity_to_location: AlmanacMap = parts[7].parse()?;
        Ok(Self {
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        })
    }
}

#[derive(Debug)]
struct AlmanacMap {
    ranges: Vec<AlmanacRange>,
}

impl FromStr for AlmanacMap {
    type Err = color_eyre::eyre::Error;

    fn from_str(s: &str) -> Result<Self> {
        let ranges = s
            .lines()
            .skip(1)
            .map(|line| line.parse())
            .collect::<Result<Vec<AlmanacRange>>>()?;
        Ok(Self { ranges })
    }
}

impl AlmanacMap {
    fn convert(&self, input: usize) -> usize {
        for range in &self.ranges {
            let conversion = range.convert(input);
            if conversion != input {
                return conversion;
            }
        }
        input
    }

    fn convert_range(&self, input: &Range<usize>) -> Vec<Range<usize>> {
        for range in &self.ranges {
            if let Some(converted_range) = range.convert_range(input) {
                return converted_range;
            }
        }
        vec![input.clone()]
    }
}

#[derive(Debug)]
struct AlmanacRange {
    source_start: usize,
    destination_start: usize,
    length: usize,
}

impl FromStr for AlmanacRange {
    type Err = color_eyre::eyre::Error;

    fn from_str(s: &str) -> Result<Self> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        let destination_start = parts[0].parse()?;
        let source_start = parts[1].parse()?;
        let length = parts[2].parse()?;
        Ok(Self {
            source_start,
            destination_start,
            length,
        })
    }
}

impl AlmanacRange {
    fn convert(&self, input: usize) -> usize {
        let source_start = self.source_start;
        if !self.source_range().contains(&input) {
            input
        } else {
            let destination_start = self.destination_start;
            let offset = input - source_start;
            destination_start + offset
        }
    }

    fn convert_range(&self, input: &Range<usize>) -> Option<Vec<Range<usize>>> {
        let source_range = self.source_range();

        let x1 = input.start;
        let x2 = input.end;
        let y1 = source_range.start;
        let y2 = source_range.end;

        if x1 >= y1 && x2 <= y2 {
            let range = self.convert(x1)..self.convert(x2);
            return Some(vec![range]);
        }

        if x1 <= y1 && x2 <= y2 {
            let range = x1..y1;
            let converted_range = self.convert(y1)..self.convert(x2);
            return Some(vec![range, converted_range]);
        }

        if x1 >= y1 && x2 >= y2 {
            let range = y2..x2;
            let converted_range = self.convert(x1)..self.convert(y2);
            return Some(vec![range, converted_range]);
        }

        None
    }

    fn source_range(&self) -> Range<usize> {
        self.source_start..(self.source_start + self.length)
    }
}

#[tracing::instrument]
pub fn part_one(input: &str) -> Result<String> {
    let seeds: Vec<usize> = input
        .lines()
        .next()
        .unwrap()
        .replace("seeds: ", "")
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let almanac: Almanac = input.parse()?;
    let min_location: usize = seeds
        .iter()
        .map(|s| almanac.seed_to_location(*s))
        .min()
        .unwrap();
    Ok(min_location.to_string())
}

#[tracing::instrument]
pub fn part_two(input: &str) -> Result<String> {
    let seed_ranges: Vec<Range<usize>> = input
        .lines()
        .next()
        .unwrap()
        .replace("seeds: ", "")
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .chunks(2)
        .into_iter()
        .map(|mut chunk| {
            let start: usize = chunk.next().unwrap();
            let end: usize = start + chunk.next().unwrap();
            start..end
        })
        .collect();
    let almanac: Almanac = input.parse()?;
    let converted: Vec<Range<usize>> = seed_ranges
        .iter()
        .flat_map(|r| almanac.seed_to_soil.convert_range(r))
        .collect();
    let min_location = 0;
    Ok(min_location.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_input() -> &'static str {
        "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"
    }

    #[test]
    fn test_seed_to_soil() -> Result<()> {
        let almanac: Almanac = test_input().parse()?;
        assert_eq!(almanac.seed_to_soil.convert(98), 50);
        assert_eq!(almanac.seed_to_soil.convert(99), 51);
        Ok(())
    }

    #[test]
    fn test_part_one() -> Result<()> {
        let expected = "35";
        let actual = part_one(&test_input())?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let expected = "46";
        let actual = part_two(&test_input())?;
        assert_eq!(expected, actual);
        Ok(())
    }
}
