use std::env;
use std::fs;

struct Bus {
    id: i64,
    offset: i64,
}

impl Bus {
    fn new(id: i64, offset: i64) -> Bus {
        Bus { id, offset }
    }

    fn is_at_port(&self, timestamp: i64) -> bool {
        if (timestamp + self.offset) % self.id == 0 {
            return true;
        }
        false
    }

    fn highest_id(buses: &[Bus]) -> &Bus {
        let mut current_high = &buses[0];
        for bus in buses.iter() {
            if bus.id > current_high.id {
                current_high = bus;
            }
        }
        current_high
    }
}
impl std::fmt::Debug for Bus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(ID: {}, Offset: {})", self.id, self.offset)
    }
}

fn parse_part2(ids: &[String]) -> Vec<Bus> {
    let mut buses = Vec::new();
    for (index, id) in ids.iter().enumerate() {
        if id != "x" {
            let id = id.parse().unwrap();
            buses.push(Bus::new(id, index as i64));
        }
    }
    buses
}

fn run_part2(start_timestamp: i64, ids: &[String]) -> i64 {
    let buses = parse_part2(ids);
    let highest_id_bus = Bus::highest_id(&buses);
    let start_timestamp =
        start_timestamp + (highest_id_bus.id - (start_timestamp % highest_id_bus.id));
    let mut timestamp = start_timestamp - highest_id_bus.offset;
    let timestamp_gap = highest_id_bus.id;
    loop {
        let mut all_at_station = true;
        for bus in buses.iter() {
            if !bus.is_at_port(timestamp) {
                all_at_station = false;
                break;
            }
        }
        if all_at_station {
            break;
        }
        timestamp += timestamp_gap;
    }
    timestamp
}

fn solve_part1(timestamp: u64, ids: &[u64]) -> (u64, u64) {
    let mut timestamp = timestamp;
    loop {
        for id in ids.iter() {
            if timestamp % id == 0 {
                return (timestamp, *id);
            }
        }
        timestamp += 1;
    }
}

fn parse_file(file_path: &str) -> (u64, Vec<String>) {
    let mut ids = Vec::new();
    let file_content = fs::read_to_string(file_path).expect("Error reading input file.");
    let mut lines: Vec<String> = Vec::new();
    for line in file_content.lines() {
        lines.push(String::from(line));
    }
    let timestamp = lines[0].parse().unwrap();
    for num in lines[1].split(',') {
        ids.push(String::from(num));
    }
    (timestamp, ids)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide input file path.");
        std::process::exit(1);
    }

    let (timestamp, ids) = parse_file(&args[1]);

    println!("Part 1:");
    let mut valid_id = Vec::new();
    for id in ids.iter() {
        if *id == "x" {
            continue;
        }
        valid_id.push(id.parse().unwrap());
    }
    let (part1_time, part1_id) = solve_part1(timestamp, &valid_id);
    println!("Current time         : {}", timestamp);
    println!("Next bus arrival time: {}", part1_time);
    println!("Next bus ID          : {}", part1_id);
    println!(
        "Product              : {}",
        part1_id * (part1_time - timestamp)
    );
    println!();
    println!("Part 2:");
    println!(
        "Earliest timestamp   : {}",
        run_part2(100000000000000, &ids)
    );
}
