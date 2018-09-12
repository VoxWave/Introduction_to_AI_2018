#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use std::path::Path;
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::BufReader;
use std::collections::HashMap;
use std::collections::VecDeque;


fn main() {
    let stops = load_stops();
    let visited = HashMap::new();
    let mut queue = VecDeque::new();
    println!("From:");
    let mut from = String::new();
    io::stdin().read_line(&mut from).expect("Failed to read line");
    println!("To:");
    let mut to = String::new();
    io::stdin().read_line(&mut to).expect("Failed to read line");
    queue.push_front((from, from));

    let mut visited = HashMap::new();
    while let Some((parent, stop)) = queue.pop_back() {
        if stop == to {
            trace_back_and_print(stop, parent, visited, stops);
            break;
        } else {
            if visited.contains_key(stop) {
                continue;
            }
            visited.insert(stop, parent);
            let stop = stops.get(&stop).unwrap();
            for neighbour in stop.neighbors.keys() {
                queue.push_front((stop.code, *neighbour));
            }
        }
    }
}

fn trace_back_and_print(destination: String, parent: String, visited: HashMap<String, String>, stops: HashMap<String, Stop>) {
    let stop = stops.get(&destination).unwrap();
    let mut parent = stops.get(&parent).unwrap();
    let mut route = String::new();
    route.push_str(&format!("{}({})[DESTINATION]", stop.code, stop.address));
    route.push_str(" -> ");
    route.push_str(&format!("{}({})", parent.code, parent.address));
    while let Some(parent_id) = visited.get(&parent.code) {
        route.push_str(" -> ");
        let parent = stops.get(&parent_id).unwrap();
        route.push_str(&format!("{}({})", parent.code, parent.address));
    }
    route.push_str("[START]");
    println!("{}", route);
}

fn load_stops() -> HashMap<String, Stop> {
    let file_path = Path::new("./resource/network.json");
    let file = File::open(file_path).unwrap();

    let mut data = String::new();
    let mut buf_reader = BufReader::new(file);
    buf_reader.read_to_string(&mut data).unwrap();

    let stops: Vec<Stop> = serde_json::from_str(&data).unwrap();
    let mut stop_map = HashMap::new();
    for stop in stops {
        stop_map.insert(stop.code.clone(), stop);
    };
    stop_map
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