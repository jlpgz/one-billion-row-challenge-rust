use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() {
    let mut values: HashMap<String, Vec<f32>> = HashMap::new();

    let file = File::open("measurements.txt").expect("Error opening data file");
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(line) => {
                let (station, measurement) = extract_data(line);
                match values.get_mut(&station) {
                    Some(array) => array.push(measurement),
                    None => _ = values.insert(station, vec![measurement])
                }
            }
            Err(error) => println!("{}", format!("Error reading line: {}", error.to_string())),
        }
    }
    let averages: HashMap<String, f32> = values.iter().map(|(k, v)| {
        let total: f32 = v.iter().sum();
        (k.clone(), total / v.len() as f32)
    }).collect();

    let mut vec: Vec<(&String, &f32)> = averages.iter().collect();
    vec.sort_by(|a, b| a.0.cmp(b.0));
    for (station, average) in averages.iter() {
        println!("{}: {}", station, average);
    }
}


fn extract_data(line: String) -> (String, f32) {
    let parts: Vec<&str> = line.split(";").collect();
    match parts[..] {
        [station, measurement] => {
            let station = station.to_string();
            let measurement = measurement.parse::<f32>().expect("Unable to parse measurement");
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
        let line = "Kelilalina;-96.9".to_string();
        let (station, measurement) = extract_data(line);

        assert_eq!(station, "Kelilalina");
        assert_eq!(measurement, -96.9);
    }
}