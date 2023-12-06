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

    pub fn seed_range_to_location_ranges(&self, seed_range: &Range<usize>) -> Vec<Range<usize>> {
        self.seed_to_soil
            .convert_range(seed_range)
            .iter()
            .flat_map(|r| self.soil_to_fertilizer.convert_range(r))
            .flat_map(|r| self.fertilizer_to_water.convert_range(&r))
            .flat_map(|r| self.water_to_light.convert_range(&r))
            .flat_map(|r| self.light_to_temperature.convert_range(&r))
            .flat_map(|r| self.temperature_to_humidity.convert_range(&r))
            .flat_map(|r| self.humidity_to_location.convert_range(&r))
            .collect()
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

        // COMPLETELY WITHIN RANGE
        if x1 >= y1 && x2 < y2 {
            let range = self.convert(x1)..self.convert(x2);
            return Some(vec![range]);
        }

        // START WITHIN RANGE END OUTSIDE RANGE
        if x1 >= y1 && x2 >= y2 && x1 < y2 {
            let convert_range = self.convert(x1)..self.convert(y2);
            let remainder_range = y2..x2;
            return Some(vec![convert_range, remainder_range]);
        }

        // START BEFORE RANGE END WITHIN RANGE
        if x1 < y1 && x2 < y2 && x2 > y1 {
            let convert_range = self.convert(y1)..self.convert(x2);
            let remainder_range = x1..y1;
            return Some(vec![remainder_range, convert_range]);
        }

        // OUTSIDE OF RANGE
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
    let min_location = seed_ranges
        .iter()
        .flat_map(|r| almanac.seed_range_to_location_ranges(r))
        .map(|r| r.start)
        .min()
        .unwrap();
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
    fn test_convert_range() -> Result<()> {
        let almanac: Almanac = test_input().parse()?;
        let input1 = 79..93;
        let input2 = 55..68;

        let expected1 = 81..95;
        let expected2 = 57..70;

        let actual1 = almanac.seed_to_soil.convert_range(&input1);
        let actual2 = almanac.seed_to_soil.convert_range(&input2);
        assert!(actual1.len() == 1);
        assert!(actual2.len() == 1);
        assert_eq!(expected1, actual1[0]);
        assert_eq!(expected2, actual2[0]);
        Ok(())
    }

    #[test]
    fn test_edge_cases_conver_range() -> Result<()> {
        let almanac_range = AlmanacRange {
            source_start: 10,
            destination_start: 100,
            length: 10,
        };

        //start before range end within range
        let input1 = 5..15;
        // start in range end outside of range
        let input2 = 15..25;
        // perfect overlap with range 
        let input3 = 10..20;
        // start within range end within range
        let input4 = 12..18;

        let expected1 = vec![5..10, 100..105];
        let expected2 = vec![105..110, 20..25];
        let expected3 = vec![100..110];
        let expected4 = vec![102..108];

        let actual1 = almanac_range.convert_range(&input1);
        let actual2 = almanac_range.convert_range(&input2);
        let actual3 = almanac_range.convert_range(&input3);
        let actual4 = almanac_range.convert_range(&input4);

        assert_eq!(expected1, actual1.unwrap());
        assert_eq!(expected2, actual2.unwrap());
        assert_eq!(expected3, actual3.unwrap());
        assert_eq!(expected4, actual4.unwrap());

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
