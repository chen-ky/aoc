use std::collections::HashMap;
use std::{env, fs, process};

#[derive(Debug)]
struct ConwayCubes {
    cubes: HashMap<Coordinate, State>,
    check_limit: Coordinate,
    dimension: usize,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Coordinate {
    x: isize,
    y: isize,
    z: isize,
    w: isize,
}

#[derive(Debug, Clone)]
enum State {
    Active,
    Inactive,
}

fn execute_n_cycles(mut conway_cubes: ConwayCubes, mut n: usize) -> ConwayCubes {
    while n > 0 {
        conway_cubes = cycle(conway_cubes);
        n -= 1;
    }
    conway_cubes
}

fn cycle(conway_cubes: ConwayCubes) -> ConwayCubes {
    let mut new_cubes = HashMap::new();
    let mut new_limit = Coordinate {
        x: 0,
        y: 0,
        z: 0,
        w: 0,
    };

    for check_w in -conway_cubes.check_limit.w..conway_cubes.check_limit.w + 1 {
        for check_z in -conway_cubes.check_limit.z..conway_cubes.check_limit.z + 1 {
            for check_y in -conway_cubes.check_limit.y..conway_cubes.check_limit.y + 1 {
                for check_x in -conway_cubes.check_limit.x..conway_cubes.check_limit.x + 1 {
                    let check_coordinate = Coordinate {
                        x: check_x,
                        y: check_y,
                        z: check_z,
                        w: check_w,
                    };
                    let mut active_counter = 0;
                    for cube in get_surrounding(&conway_cubes, &check_coordinate) {
                        match cube {
                            State::Active => active_counter += 1,
                            State::Inactive => {}
                        }
                    }
                    match get_cube(&conway_cubes, &check_coordinate) {
                        State::Active => {
                            if active_counter == 2 || active_counter == 3 {
                                new_cubes.insert(check_coordinate, State::Active);
                            } else {
                                continue;
                            }
                        }
                        State::Inactive => {
                            if active_counter == 3 {
                                new_cubes.insert(check_coordinate, State::Active);
                            } else {
                                continue;
                            }
                        }
                    }
                    // If new cube not active, will not reach this branch
                    if check_x.abs() + 1 > new_limit.x {
                        new_limit.x = check_x.abs() + 1;
                    }
                    if check_y.abs() + 1 > new_limit.y {
                        new_limit.y = check_y.abs() + 1;
                    }
                    if check_z.abs() + 1 > new_limit.z {
                        new_limit.z = check_z.abs() + 1;
                    }
                    if check_w.abs() + 1 > new_limit.w {
                        new_limit.w = check_w.abs() + 1;
                    }
                }
            }
        }
    }
    ConwayCubes {
        cubes: new_cubes,
        check_limit: new_limit,
        dimension: conway_cubes.dimension,
    }
}

fn get_cube(conway_cubes: &ConwayCubes, coordinate: &Coordinate) -> State {
    match conway_cubes.cubes.get_key_value(coordinate) {
        Some(v) => v.1.clone(),
        None => State::Inactive,
    }
}

fn get_surrounding(conway_cubes: &ConwayCubes, coordinate: &Coordinate) -> Vec<State> {
    let mut result = vec![];
    let mut check_coordinate = coordinate.clone();
    let mut w_range = -1..2;
    if conway_cubes.dimension == 3 {
        w_range = 0..1;
    }
    for w_offset in w_range {
        check_coordinate.w += w_offset;
        for z_offset in -1..2 {
            check_coordinate.z += z_offset;
            for y_offset in -1..2 {
                check_coordinate.y += y_offset;
                for x_offset in -1..2 {
                    check_coordinate.x += x_offset;
                    if !conway_cubes.cubes.contains_key(&check_coordinate) {
                        result.push(State::Inactive);
                    } else if check_coordinate == *coordinate {
                    } else {
                        result.push(get_cube(&conway_cubes, &check_coordinate).clone());
                    }
                    check_coordinate.x = coordinate.x;
                }
                check_coordinate.y = coordinate.y;
            }
            check_coordinate.z = coordinate.z;
        }
        check_coordinate.w = coordinate.w;
    }
    result
}

fn parse_layer(layer: &str, dimension: usize) -> ConwayCubes {
    let mut conway_cubes = ConwayCubes {
        cubes: HashMap::new(),
        check_limit: Coordinate {
            x: 0,
            y: 0,
            z: 0,
            w: 0,
        },
        dimension,
    };
    let mut y = 0;
    let z = 0;
    let w = 0;
    for line in layer.lines() {
        let mut x = 0;
        for character in line.chars() {
            conway_cubes.cubes.insert(
                Coordinate { x, y, z, w },
                match character {
                    '.' => State::Inactive,
                    '#' => State::Active,
                    _ => panic!("Invalid character."),
                },
            );
            if x.abs() + 1 > conway_cubes.check_limit.x {
                conway_cubes.check_limit.x = x.abs() + 1;
            }
            x += 1;
        }
        if y.abs() + 1 > conway_cubes.check_limit.y {
            conway_cubes.check_limit.y = y.abs() + 1;
        }
        y += 1;
    }
    if z.abs() + 1 > conway_cubes.check_limit.z {
        conway_cubes.check_limit.z = z.abs() + 1;
    }
    if w.abs() + 1 > conway_cubes.check_limit.w {
        conway_cubes.check_limit.w = w.abs() + 1;
    }
    conway_cubes
}

fn count_active_cubes(conway_cubes: &ConwayCubes) -> u128 {
    let mut counter = 0;
    for cube in conway_cubes.cubes.values() {
        match cube {
            State::Active => counter += 1,
            State::Inactive => {}
        }
    }
    counter
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide input file path.");
        process::exit(1);
    }

    let file_content = fs::read_to_string(&args[1]).expect("Error reading input file.");
    let mut energy_cubes = parse_layer(&file_content, 3);

    energy_cubes = execute_n_cycles(energy_cubes, 6);

    println!(
        "Total active cubes (Part 1): {}",
        count_active_cubes(&energy_cubes)
    );

    let mut energy_cubes = parse_layer(&file_content, 4);

    energy_cubes = execute_n_cycles(energy_cubes, 6);

    println!(
        "Total active cubes (Part 2): {}",
        count_active_cubes(&energy_cubes)
    );
}
