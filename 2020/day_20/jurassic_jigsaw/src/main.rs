use std::{fs, env, collections::HashMap};

fn parse_file(file_path: &str) -> HashMap<u32, Vec<Vec<char>>> {
    let mut result: HashMap<u32, Vec<Vec<char>>> = HashMap::new();
    let file_content = fs::read_to_string(file_path).unwrap();
    let data: Vec<&str> = file_content.split("\n").collect();
    let mut tile_number = 0;
    for line in data.iter() {
        if *line == "" {
            continue;
        }

        if line.starts_with("Tile") {
            tile_number = line.strip_prefix("Tile ").unwrap().strip_suffix(":").unwrap().parse::<u32>().unwrap();
            println!("{}", tile_number);
            continue;
        }

        let mut tile = match result.get(&tile_number) {
            Some(v) => v.clone(),
            None => vec!(),
        };

        tile.push(line.chars().collect());
        result.insert(tile_number, tile);
    }
    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide input file.");
        std::process::exit(1);
    }
    let tiles = parse_file(&args[1]);
}
