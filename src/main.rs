use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs;
use std::ops::Add;

use rayon::prelude::*;

struct Average {
    number: f32,
    value: f32,
}

impl Display for Average {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Default for Average {
    fn default() -> Self {
        Self {
            number: 0.0,
            value: 0.0,
        }
    }
}

impl Average {
    fn push(&mut self, n: f32) {
        self.value = (self.value * self.number + n) / (self.number + 1.0);
        self.number += 1.0;
    }

    fn merge(&mut self, rhs: Self) {
        self.value =
            (self.value * self.number + rhs.value * rhs.number) / (self.number + rhs.number);
        self.number += rhs.number;
    }
}

fn main() {
    let file = fs::read_to_string("measurements_1m.txt").expect("Error reading the file");

    let values: HashMap<String, Average> = file
        .par_lines()
        .map(extract_data)
        .fold(
            HashMap::<String, Average>::default,
            |mut acc, (station, measurement)| {
                acc.entry(station).or_default().push(measurement);
                acc
            },
        )
        .reduce(HashMap::<String, Average>::new, |mut acc, e| {
            for (station, average) in e {
                acc.entry(station).or_default().merge(average)
            }
            acc
        });

    let mut vec: Vec<(&String, &Average)> = values.par_iter().collect();
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
    use crate::{extract_data, Average};

    #[test]
    fn extract_line() {
        let line = "Kelilalina;-96.9";
        let (station, measurement) = extract_data(line);

        assert_eq!(station, "Kelilalina");
        assert_eq!(measurement, -96.9);
    }

    #[test]
    fn test_average_push() {
        let mut avg = Average::default();

        avg.push(2.0);
        assert_eq!(avg.value, 2.0);

        avg.push(4.0);
        assert_eq!(avg.value, 3.0);

        avg.push(6.0);
        assert_eq!(avg.value, 4.0);
    }

    #[test]
    fn test_average_merge() {
        let mut avg1 = Average::default();
        avg1.push(2.0);
        avg1.push(4.0);

        let mut avg2 = Average::default();
        avg2.push(6.0);
        avg2.push(8.0);

        avg1.merge(avg2);
        assert_eq!(avg1.value, 5.0);
    }
}
