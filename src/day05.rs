use crate::utils::read_lines;
use std::collections::HashMap;

#[derive(Debug)]
struct RangeMapping {
    destination_start: u64,
    source_start: u64,
    length: u64,
}
#[derive(Debug)]
struct Mapping {
    ranges: Vec<RangeMapping>,
}

impl Mapping {
    fn new() -> Self {
        Self { ranges: Vec::new() }
    }

    fn add_range(&mut self, destination_start: u64, source_start: u64, length: u64) {
        self.ranges.push(RangeMapping {destination_start, source_start, length});
    }

    fn lookup(&self, source: u64) -> u64 {
        for range in &self.ranges {
            if source >= range.source_start && source < range.source_start + range.length {
                return range.destination_start + (source - range.source_start);
            }
        }
        return source;
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum ConversionStep {
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

fn convert(seed: u64, mappings: &HashMap<ConversionStep, Mapping>) -> u64 {
    let mut value = seed;

    for step in &[
        ConversionStep::SeedToSoil,
        ConversionStep::SoilToFertilizer,
        ConversionStep::FertilizerToWater,
        ConversionStep::WaterToLight,
        ConversionStep::LightToTemperature,
        ConversionStep::TemperatureToHumidity,
        ConversionStep::HumidityToLocation,
    ] {
        if let Some(mapping) = mappings.get(step) {
            value = mapping.lookup(value);
        }
    }

    return value;
}

pub fn run_part1() {
    println!("Running day 05 part 1 solution.");

    let mut seeds = Vec::new();
    let mut mappings: HashMap<ConversionStep, Mapping> = HashMap::new();
    let mut current_step: Option<ConversionStep> = None;

    let filename = "input/day05.txt";

    match read_lines(filename) {
        Ok(lines) => {

            for line in lines {
                if line.is_empty() {
                    continue;
                }
                if line.starts_with("seeds:") {
                    seeds = line["seeds:".len()..]
                        .split_whitespace()
                        .map(|s| s.parse::<u64>().unwrap())
                        .collect();
                } else if line.ends_with("map:") {
                    println!("{}", &line[..line.len()  - " map:".len() ]);
                    current_step = Some(match &line[..line.len() - " map:".len() ] {
                        "seed-to-soil"              => ConversionStep::SeedToSoil,
                        "soil-to-fertilizer"        => ConversionStep::SoilToFertilizer,
                        "fertilizer-to-water"       => ConversionStep::FertilizerToWater,
                        "water-to-light"            => ConversionStep::WaterToLight,
                        "light-to-temperature"      => ConversionStep::LightToTemperature,
                        "temperature-to-humidity"   => ConversionStep::TemperatureToHumidity,
                        "humidity-to-location"      => ConversionStep::HumidityToLocation,
                        _                           => panic!("Unknown map type"),
                    });
                } else if let Some(step) = &current_step {
                    let mut parts = line.split_whitespace().map(|s| s.parse::<u64>().unwrap());
                    let destination_start = parts.next().unwrap();
                    let source_start = parts.next().unwrap();
                    let length = parts.next().unwrap();

                    let mapping = mappings.entry(step.clone()).or_insert_with(Mapping::new);
                    mapping.add_range(destination_start, source_start, length);
                }
            }
        }
        Err(e) => println!("Error: {}", e),
    }

    let mut minimum = convert(seeds[0], &mappings);

    for seed in seeds {
        let result = convert(seed, &mappings);
        if result < minimum {
            minimum = result;
        }
        println!("Seed {} maps to final location {}", seed, result);
    }

    println!("The minimum location is: {}", minimum);


}