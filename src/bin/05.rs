use std::collections::HashMap;

use crate::AlmanacMapType::{
    FertilizerToWater, HumidityToLocation, LightToTemperature, SeedToSoil, SoilToFertilizer,
    TemperatureToHumidity, WaterToLight,
};

advent_of_code::solution!(5);

#[derive(Hash, PartialEq, Eq, Debug)]
enum AlmanacMapType {
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

#[derive(Debug)]
struct AlmanacMap {
    map_type: AlmanacMapType,
    range_start: usize,
    source_range_start: usize,
    destination_range_start: usize,
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut seeds: Vec<usize> = Vec::new();
    let mut almanac_maps_by_type: HashMap<AlmanacMapType, Vec<AlmanacMap>> = HashMap::new();
    let mut lines: Vec<String> = input.lines().map(|l| l.to_string()).collect();
    let mut index = 0;
    let count = lines.len();
    while index < count {
        let line: &String = lines.get(index).unwrap();
        if line.starts_with("seeds") {
            line.split(":")
                .nth(1)
                .unwrap()
                .trim()
                .split(" ")
                .map(|s| s.trim())
                .map(|s| s.parse::<usize>().unwrap())
                .for_each(|s| seeds.push(s));
        } else if line.starts_with("seed-") {
            create_map_type(&mut almanac_maps_by_type, &mut lines, index, SeedToSoil);
        } else if line.starts_with("soil-") {
            create_map_type(
                &mut almanac_maps_by_type,
                &mut lines,
                index,
                SoilToFertilizer,
            );
        } else if line.starts_with("fertilizer-") {
            create_map_type(
                &mut almanac_maps_by_type,
                &mut lines,
                index,
                FertilizerToWater,
            );
        } else if line.starts_with("water-") {
            create_map_type(&mut almanac_maps_by_type, &mut lines, index, WaterToLight);
        } else if line.starts_with("light-") {
            create_map_type(
                &mut almanac_maps_by_type,
                &mut lines,
                index,
                LightToTemperature,
            );
        } else if line.starts_with("temperature-") {
            create_map_type(
                &mut almanac_maps_by_type,
                &mut lines,
                index,
                TemperatureToHumidity,
            );
        } else if line.starts_with("humidity-") {
            create_map_type(
                &mut almanac_maps_by_type,
                &mut lines,
                index,
                HumidityToLocation,
            );
        }
        index += 1;
    }
    let types: Vec<AlmanacMapType> = Vec::from([
        SeedToSoil,
        SoilToFertilizer,
        FertilizerToWater,
        WaterToLight,
        LightToTemperature,
        TemperatureToHumidity,
        HumidityToLocation,
    ]);
    let mut total: usize = usize::MAX;
    for i in 0..seeds.len() {
        let mut num: usize = *seeds.get(i).unwrap();
        for j in 0..types.len() {
            let lookup = types.get(j).unwrap();
            let almanac_maps = almanac_maps_by_type.get(&lookup).unwrap();
            for k in 0..almanac_maps.len() {
                let a = almanac_maps.get(k).unwrap();
                let s_start = a.source_range_start;
                let s_end = a.source_range_start + a.range_start;
                if num >= s_start && num <= s_end {
                    num = a.destination_range_start + (num - a.source_range_start);
                    break;
                }
            }
        }
        if num < total {
            total = num;
        }
    }
    Some(total)
}

fn create_map_type(
    almanac_maps_by_type: &mut HashMap<AlmanacMapType, Vec<AlmanacMap>>,
    lines: &mut Vec<String>,
    mut index: usize,
    almanac_map_type: AlmanacMapType,
) {
    let list = almanac_maps_by_type.entry(almanac_map_type).or_default();
    index += 1;
    let mut current_line = lines.get(index).unwrap();
    while !current_line.is_empty() {
        let ns: Vec<usize> = current_line
            .split(" ")
            .map(|s| s.trim())
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        let new_map = AlmanacMap {
            map_type: SeedToSoil,
            destination_range_start: *ns.get(0).unwrap(),
            source_range_start: *ns.get(1).unwrap(),
            range_start: *ns.get(2).unwrap(),
        };
        list.push(new_map);
        if index == lines.len() - 1 {
            break;
        }
        index += 1;
        current_line = lines.get(index).unwrap();
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut seeds_with_range: Vec<(usize, usize)> = Vec::new();
    let mut almanac_maps_by_type: HashMap<AlmanacMapType, Vec<AlmanacMap>> = HashMap::new();
    let mut lines: Vec<String> = input.lines().map(|l| l.to_string()).collect();
    let mut index = 0;
    let count = lines.len();
    while index < count {
        let line: &String = lines.get(index).unwrap();
        if line.starts_with("seeds") {
            let seed_numbers: Vec<usize> = line
                .split(":")
                .nth(1)
                .unwrap()
                .trim()
                .split(" ")
                .map(|s| s.trim())
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
            let mut i = 0;
            while i < seed_numbers.len() {
                seeds_with_range.push((
                    *seed_numbers.get(i).unwrap(),
                    *seed_numbers.get(i + 1).unwrap(),
                ));
                i += 2;
            }
        } else if line.starts_with("seed-") {
            create_map_type(&mut almanac_maps_by_type, &mut lines, index, SeedToSoil);
        } else if line.starts_with("soil-") {
            create_map_type(
                &mut almanac_maps_by_type,
                &mut lines,
                index,
                SoilToFertilizer,
            );
        } else if line.starts_with("fertilizer-") {
            create_map_type(
                &mut almanac_maps_by_type,
                &mut lines,
                index,
                FertilizerToWater,
            );
        } else if line.starts_with("water-") {
            create_map_type(&mut almanac_maps_by_type, &mut lines, index, WaterToLight);
        } else if line.starts_with("light-") {
            create_map_type(
                &mut almanac_maps_by_type,
                &mut lines,
                index,
                LightToTemperature,
            );
        } else if line.starts_with("temperature-") {
            create_map_type(
                &mut almanac_maps_by_type,
                &mut lines,
                index,
                TemperatureToHumidity,
            );
        } else if line.starts_with("humidity-") {
            create_map_type(
                &mut almanac_maps_by_type,
                &mut lines,
                index,
                HumidityToLocation,
            );
        }
        index += 1;
    }
    let types: Vec<AlmanacMapType> = Vec::from([
        SeedToSoil,
        SoilToFertilizer,
        FertilizerToWater,
        WaterToLight,
        LightToTemperature,
        TemperatureToHumidity,
        HumidityToLocation,
    ]);
    let mut total: usize = usize::MAX;

    for i in 0..seeds_with_range.len() {
        let (seed_start, seed_range) = *seeds_with_range.get(i).unwrap();
        for k in seed_start..(seed_start + seed_range) {
            let mut num = k;
            for j in 0..types.len() {
                let type_lookup = types.get(j).unwrap();
                let almanac_maps = almanac_maps_by_type.get(&type_lookup).unwrap();
                for k in 0..almanac_maps.len() {
                    let a = almanac_maps.get(k).unwrap();
                    let s_start = a.source_range_start;
                    let s_end = a.source_range_start + a.range_start;
                    if num >= s_start && num <= s_end {
                        num = a.destination_range_start + (num - a.source_range_start);
                        break;
                    }
                }
            }
            if num < total {
                total = num;
            }
        }
    }
    Some(total)
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
