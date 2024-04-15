use std::collections::HashMap;
use std::fs;
use std::hash::BuildHasherDefault;

use rayon::prelude::*;
use twox_hash::XxHash;

fn main() {
    let file = fs::read_to_string("measurements_1b.txt").expect("Error reading the file");

    let values: HashMap<String, Vec<f32>> = file
        .par_lines()
        .fold(HashMap::<String, Vec<f32>>::new, |mut acc, line| {
            if !line.is_empty() {
                let (station, measurement) = extract_data(line);
                acc.entry(station).or_default().push(measurement);
                acc
            } else {
                HashMap::new()
            }
        })
        .reduce(HashMap::<String, Vec<f32>>::new, |mut acc, map| {
            for (station, measurements) in map {
                acc.entry(station).or_default().extend(measurements);
            }
            acc
        });
    let averages: HashMap<String, f32> = values
        .par_iter()
        .map(|(k, v)| {
            let total: f32 = v.iter().sum();
            (k.clone(), total / v.len() as f32)
        })
        .collect();

    let mut vec: Vec<(&String, &f32)> = averages.par_iter().collect();
    vec.sort_by(|a, b| a.0.cmp(b.0));
    for (station, average) in vec.iter() {
        println!("{}: {}", station, average);
    }
}

fn extract_data(line: &str) -> (String, f32) {
    let parts: Vec<&str> = line.split(';').collect();
    match parts[..] {
        [station, measurement] => {
            let station = station.to_string();
            let measurement = measurement
                .parse::<f32>()
                .expect("Unable to parse measurement");
            (station, measurement)
        }
        _ => panic!("Expected a line with two parts"),
    }
}

#[cfg(test)]
mod tests {
    use crate::extract_data;

    #[test]
    fn extract_line() {
        let line = "Kelilalina;-96.9";
        let (station, measurement) = extract_data(line);

        assert_eq!(station, "Kelilalina");
        assert_eq!(measurement, -96.9);
    }
}
