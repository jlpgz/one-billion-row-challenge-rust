use std::collections::HashMap;
use std::fs;
use std::sync::{Arc, Mutex};

use rayon::prelude::*;

fn main() {
    let shared_values: Arc<Mutex<HashMap<String, Vec<f32>>>> = Arc::new(Mutex::new(HashMap::new()));

    let file = fs::read_to_string("measurements_1m.txt").expect("Error reading the file");

    for line in file.split("\n") {
        if line != "" {
            let (station, measurement) = extract_data(line);
            let mut values = shared_values.lock().unwrap();
            match values.get_mut(&station) {
                Some(array) => array.push(measurement),
                None => _ = values.insert(station, vec![measurement]),
            }
        }
    }
    let averages: HashMap<String, f32> = shared_values
        .lock()
        .unwrap()
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
    let parts: Vec<&str> = line.split(";").collect();
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
