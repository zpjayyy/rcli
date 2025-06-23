use std::{collections::HashMap, fs::File, iter::zip};

use anyhow::Result;

pub fn process_csv(input: &str, output: &str) -> Result<()> {
    let result = read_csv(input).unwrap();
    write_json(&result, output).unwrap();
    Ok(())
}

pub fn read_csv(file_path: &str) -> Result<Vec<HashMap<String, String>>> {
    let mut reader = csv::Reader::from_path(file_path)?;
    let headers = reader.headers()?.clone();

    let mut result = Vec::new();

    for record in reader.records() {
        let record = record?;
        let map: HashMap<String, String> = zip(headers.iter(), record.iter())
            .map(|(h, v)| (h.to_string(), v.to_string()))
            .collect();
        result.push(map);
    }
    println!("{:?}", result);
    Ok(result)
}

pub fn write_json(data: &Vec<HashMap<String, String>>, file_path: &str) -> Result<()> {
    let file = File::create(file_path)?;
    serde_json::to_writer_pretty(file, data)?;
    Ok(())
}
