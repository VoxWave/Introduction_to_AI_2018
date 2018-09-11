#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::io::BufReader;
use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let file_path = Path::new("./resource/network.json");
    let file = File::open(file_path).unwrap();
    let mut data = String::new();
    let mut buf_reader = BufReader::new(file);
    buf_reader.read_to_string(&mut data).unwrap();
    let stops: Vec<Stop> = serde_json::from_str(&data).unwrap();
    let mut stop_map = HashMap::new();
    for stop in stops {
        stop_map.insert(stop.code.clone(), stop);
    }
    println!("{:?}", stop_map)
}

#[derive(Debug, Serialize, Deserialize)]
struct Stop {
    code: String,
    address: String,
    name: String,
    x: i32,
    y: i32,
    neighbors: HashMap<String, Vec<String>>,
}