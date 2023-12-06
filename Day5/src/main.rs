// use std::{fs, u64::MAX};

// fn main() {
//     let data = fs::read_to_string(
//         "C:\\Users\\jcz21\\Documents\\RustProjects\\AdventOfCode2023\\Day5\\src\\input.txt",
//     )
//     .expect("Unable to read file");
//     let mut seed_to_soil: Vec<(u64, u64, u64)> = vec![];
//     let mut soil_to_fertilizer: Vec<(u64, u64, u64)> = vec![];
//     let mut fertilizer_to_water: Vec<(u64, u64, u64)> = vec![];
//     let mut water_to_light: Vec<(u64, u64, u64)> = vec![];
//     let mut light_to_temperature: Vec<(u64, u64, u64)> = vec![];
//     let mut temperature_to_humidity: Vec<(u64, u64, u64)> = vec![];
//     let mut humidity_to_location: Vec<(u64, u64, u64)> = vec![];

//     let mut seeds: Vec<&str> = vec![];

//     let mut cur_map = &mut seed_to_soil;
//     for line in data.lines() {
//         if line.contains("seeds:") {
//             seeds = line.split_ascii_whitespace().collect::<Vec<&str>>();
//             seeds.remove(0);
//         } else if line.contains("soil-to") {
//             cur_map = &mut soil_to_fertilizer;
//         } else if line.contains("fertilizer-to") {
//             cur_map = &mut fertilizer_to_water;
//         } else if line.contains("water-to") {
//             cur_map = &mut water_to_light;
//         } else if line.contains("light-to") {
//             cur_map = &mut light_to_temperature;
//         } else if line.contains("temperature-to") {
//             cur_map = &mut temperature_to_humidity;
//         } else if line.contains("humidity-to") {
//             cur_map = &mut humidity_to_location;
//         } else {
//             let vals: Vec<&str> = line.split_ascii_whitespace().collect();
//             if vals.len() > 2 {
//                 cur_map.push((
//                     vals[0].parse::<u64>().unwrap(),
//                     vals[1].parse::<u64>().unwrap(),
//                     vals[2].parse::<u64>().unwrap(),
//                 ));
//             }
//         }
//     }
//     let mut min = MAX;
//     for seed in seeds {
//         let soil = convert(seed.parse::<u64>().unwrap(), seed_to_soil.clone());
//         let fert = convert(soil, soil_to_fertilizer.clone());
//         let water = convert(fert, fertilizer_to_water.clone());
//         let light = convert(water, water_to_light.clone());
//         let temp = convert(light, light_to_temperature.clone());
//         let humidity = convert(temp, temperature_to_humidity.clone());
//         let location = convert(humidity, humidity_to_location.clone());
//         println!("{}", water);
//         if location < min {
//             min = location;
//         }
//     }

//     println!("{}", min);
// }

// fn convert(seed: u64, mapping: Vec<(u64, u64, u64)>) -> u64 {
//     for line in mapping {
//         if seed >= line.1 && seed < line.1 + line.2 {
//             return line.0 + (seed - line.1)
//         }
//     }
//     seed
// }

use std::{fs, u64::MAX};

fn main() {
    let data = fs::read_to_string(
        "C:\\Users\\jcz21\\Documents\\RustProjects\\AdventOfCode2023\\Day5\\src\\input.txt",
    )
    .expect("Unable to read file");
    let mut seed_to_soil: Vec<(u64, u64, u64)> = vec![];
    let mut soil_to_fertilizer: Vec<(u64, u64, u64)> = vec![];
    let mut fertilizer_to_water: Vec<(u64, u64, u64)> = vec![];
    let mut water_to_light: Vec<(u64, u64, u64)> = vec![];
    let mut light_to_temperature: Vec<(u64, u64, u64)> = vec![];
    let mut temperature_to_humidity: Vec<(u64, u64, u64)> = vec![];
    let mut humidity_to_location: Vec<(u64, u64, u64)> = vec![];

    let mut seeds: Vec<(u64, u64)> = vec![];

    let mut cur_map = &mut seed_to_soil;
    for line in data.lines() {
        if line.contains("seeds:") {
            let mut temp = line
                .split_ascii_whitespace()
                .filter(|x| !x.contains("se"))
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            println!("{:?}", temp);
            seeds = temp.chunks(2).map(|x| (x[0], x[1])).collect();
        } else if line.contains("soil-to") {
            cur_map = &mut soil_to_fertilizer;
        } else if line.contains("fertilizer-to") {
            cur_map = &mut fertilizer_to_water;
        } else if line.contains("water-to") {
            cur_map = &mut water_to_light;
        } else if line.contains("light-to") {
            cur_map = &mut light_to_temperature;
        } else if line.contains("temperature-to") {
            cur_map = &mut temperature_to_humidity;
        } else if line.contains("humidity-to") {
            cur_map = &mut humidity_to_location;
        } else {
            let vals: Vec<&str> = line.split_ascii_whitespace().collect();
            if vals.len() > 2 {
                cur_map.push((
                    vals[0].parse::<u64>().unwrap(),
                    vals[1].parse::<u64>().unwrap(),
                    vals[2].parse::<u64>().unwrap(),
                ));
            }
        }
    }
    for i in 0..1082128224 {
        let humidity = convert(i, humidity_to_location.clone());
        let temp = convert(humidity, temperature_to_humidity.clone());
        let light = convert(temp, light_to_temperature.clone());
        let water = convert(light, water_to_light.clone());
        let fert = convert(water, fertilizer_to_water.clone());
        let soil = convert(fert, soil_to_fertilizer.clone());
        let poop = convert(soil, seed_to_soil.clone());

        for s in &seeds {
            if poop >= s.0 && poop < s.1 + s.0 {
                println!("{}", i);
                return;
            }
        }
    }
}

fn convert(seed: u64, mapping: Vec<(u64, u64, u64)>) -> u64 {
    for line in mapping {
        if seed >= line.0 && seed < line.0 + line.2 {
            return line.1 + (seed - line.0);
        }
    }
    seed
}
