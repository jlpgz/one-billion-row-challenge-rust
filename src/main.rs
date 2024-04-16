use std::fmt::{Display, Formatter};
use std::{env, fs};

// Ahash is a faster hashing algorithm. It provides its own AHashMap wrapper.
// See https://github.com/tkaitchuck/ahash
use ahash::AHashMap;
use rayon::prelude::*;

struct Average {
    // Number of items of the average.
    number: f32,
    // Average value.
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
    // Every time we push a new value the average is updated.
    fn push(&mut self, n: f32) {
        self.value = (self.value * self.number + n) / (self.number + 1.0);
        self.number += 1.0;
    }

    // We can merge two Average structs. The one calling the method updates itself
    // adding the values from the second one.
    fn merge(&mut self, rhs: Self) {
        self.value =
            (self.value * self.number + rhs.value * rhs.number) / (self.number + rhs.number);
        self.number += rhs.number;
    }
}

fn main() {
    // Read the arguments passed to the application. We expect just one, which contains
    // the path of the data file.
    let args: Vec<String> = env::args().collect();
    let path = args.get(1).expect("No arguments provided");

    // Read the whole content of the file into memory.
    let file = fs::read_to_string(path).expect("Error reading the file");

    let values: AHashMap<String, Average> = file
        // Use a parallel iterator over the lines.
        .par_lines()
        // Extract data from every line, it generates (station, measurement) tuples.
        .map(extract_data)
        // Parallel folding the tuples. Rayon decides how many batches are processed in parallel.
        // The result contains several aggregated tuples. See rayon::fold documentation.
        .fold(
            AHashMap::<String, Average>::default,
            |mut acc, (station, measurement)| {
                acc.entry(station).or_default().push(measurement);
                acc
            },
        )
        // Reducing the result of the previous parallel folding. As opposed to fold, reduce
        // produces one unique aggregated tuple.
        .reduce(AHashMap::<String, Average>::new, |mut acc, e| {
            for (station, average) in e {
                acc.entry(station).or_default().merge(average)
            }
            acc
        });

    // We cannot order a HashMap, so we generate a Vec containing (station, average) tuples.
    let mut vec: Vec<(&String, &Average)> = values.par_iter().collect();
    // And we order it by station name.
    vec.sort_by(|a, b| a.0.cmp(b.0));
    // Finally print to screen the results.
    for (station, average) in vec.iter() {
        println!("{}: {}", station, average);
    }
}

// Extract data from a line and parse the measurement into a f32.
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
