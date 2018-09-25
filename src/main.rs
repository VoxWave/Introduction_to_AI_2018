#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;

fn main() {
    let stops = load_stops();
    let mut queue = VecDeque::new();
    let from = "1250429".to_string();
    let to = "1121480".to_string();
//    println!("From:");
//    let mut from = String::new();
//    io::stdin()
//        .read_line(&mut from)
//        .expect("Failed to read line");
//    println!("To:");
//    let mut to = String::new();
//    from = from.trim().to_string();
//    to = to.trim().to_string();
//    io::stdin().read_line(&mut to).expect("Failed to read line");
    queue.push_front((None, from));

    let mut visited = HashMap::new();
    while let Some((parent, stop_code)) = queue.pop_back() {
        if stop_code == to {
            visited.insert(stop_code.clone(), parent);
            trace_back_and_print(stop_code, visited, stops);
            break;
        } else {
            if visited.contains_key(&stop_code) {
                continue;
            }
            let stop = stops.get(&stop_code).unwrap();
            visited.insert(stop_code, parent);
            for neighbour in stop.neighbors.keys() {
                queue.push_front((Some(stop.code.clone()), neighbour.clone()));
            }
        }
    }
}

fn trace_back_and_print(
    destination: String,
    visited: HashMap<String, Option<String>>,
    stops: HashMap<String, Stop>,
) {
    let mut stop = stops.get(&destination).unwrap();
    let mut route = String::new();
    route.push_str(&format!("{}({})[DESTINATION]", stop.code, stop.name));
    while let Some(Some(parent_id)) = visited.get(&stop.code) {
        route.push_str(" -> ");
        stop = stops.get(parent_id).unwrap();
        route.push_str(&format!("{}({})", stop.code, stop.name));
    }
    route.push_str("[START]");
    println!("{}", route);
}

fn load_stops() -> HashMap<String, Stop> {
    let file_path = Path::new("./resource/graph.json");
    let file = File::open(file_path).unwrap();

    let mut data = String::new();
    let mut buf_reader = BufReader::new(file);
    buf_reader.read_to_string(&mut data).unwrap();

    let stops: Vec<Stop> = serde_json::from_str(&data).unwrap();
    let mut stop_map = HashMap::new();
    for stop in stops {
        stop_map.insert(stop.code.clone(), stop);
    }
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
