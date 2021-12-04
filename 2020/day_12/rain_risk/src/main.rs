use std::env;
use std::fs;

struct Ship {
    bearing: i32,
    x: i32,
    y: i32,
    waypoint_x: i32,
    waypoint_y: i32,
}

impl Ship {
    fn drive_part1(&mut self, instruction: &(char, i32)) {
        match instruction.0 {
            'N' => self.y += instruction.1,
            'S' => self.y -= instruction.1,
            'E' => self.x += instruction.1,
            'W' => self.x -= instruction.1,
            'L' => self.bearing -= instruction.1,
            'R' => self.bearing += instruction.1,
            'F' => {
                // Only consider 4 directions here
                match self.bearing {
                    0 => self.y += instruction.1,
                    90 => self.x += instruction.1,
                    180 => self.y -= instruction.1,
                    270 => self.x -= instruction.1,
                    _ => panic!("Bearing not divisible by 90."),
                }
            }
            _ => panic!("Invalid instruction!"),
        };

        // Ensure bearing is always [0, 360)
        self.bearing %= 360;
        while self.bearing < 0 {
            self.bearing += 360;
        }
    }

    fn drive_part2(&mut self, instruction: &(char, i32)) {
        match instruction.0 {
            'N' => self.waypoint_y += instruction.1,
            'S' => self.waypoint_y -= instruction.1,
            'E' => self.waypoint_x += instruction.1,
            'W' => self.waypoint_x -= instruction.1,
            'L' | 'R' => {
                let mut waypoint_bearing = instruction.1;
                let mut operation = instruction.0;
                waypoint_bearing %= 360;
                if waypoint_bearing < 0 {
                    if operation == 'L' {
                        operation = 'R';
                    } else {
                        operation = 'L';
                    }
                    waypoint_bearing = waypoint_bearing.abs();
                }

                let tmp_waypoint_x = self.waypoint_x;
                let tmp_waypoint_y = self.waypoint_y;
                if operation == 'L' {
                    match waypoint_bearing {
                        0 => {}
                        90 => {
                            self.waypoint_y = self.waypoint_x;
                            self.waypoint_x = -tmp_waypoint_y;
                        }
                        180 => {
                            self.waypoint_y = -self.waypoint_y;
                            self.waypoint_x = -self.waypoint_x;
                        }
                        270 => {
                            self.waypoint_x = self.waypoint_y;
                            self.waypoint_y = -tmp_waypoint_x;
                        }
                        _ => panic!("Angle not divisible by 90."),
                    }
                } else {
                    match waypoint_bearing {
                        0 => {}
                        90 => {
                            self.waypoint_y = -self.waypoint_x;
                            self.waypoint_x = tmp_waypoint_y;
                        }
                        180 => {
                            self.waypoint_y = -self.waypoint_y;
                            self.waypoint_x = -self.waypoint_x;
                        }
                        270 => {
                            self.waypoint_x = -self.waypoint_y;
                            self.waypoint_y = tmp_waypoint_x;
                        }
                        _ => panic!("Angle not divisible by 90."),
                    }
                }
            }
            'F' => {
                let mut move_count = 0;
                while move_count < instruction.1 {
                    self.x += self.waypoint_x;
                    self.y += self.waypoint_y;
                    move_count += 1;
                }
            }
            _ => panic!("Invalid instruction!"),
        };
    }

    fn execute_instructions(&mut self, instructions: &[(char, i32)], mode: bool) {
        for instruction in instructions {
            if mode {
                // mode true = Part 2, false = Part 1
                self.drive_part2(instruction);
            } else {
                self.drive_part1(instruction);
            }
        }
    }
}

fn parse_file(file_path: &str) -> Vec<(char, i32)> {
    let file_content = fs::read_to_string(file_path).expect("Error reading file.");
    let mut instructions = Vec::new();

    for line in file_content.lines() {
        let operation: Vec<char> = line.chars().collect();
        let operation = operation[0];
        let value: i32 = (line[1..]).parse().unwrap();
        instructions.push((operation, value));
    }
    instructions
}

fn manhattan_distance(x: i32, y: i32) -> i32 {
    x.abs() + y.abs()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide input file path.");
        std::process::exit(1);
    }

    let instructions = parse_file(&args[1]);

    let mut ship_1 = Ship {
        bearing: 90, // Facing east
        x: 0,
        y: 0,
        waypoint_x: 0,
        waypoint_y: 0,
    };
    let mut ship_2 = Ship {
        waypoint_x: 10,
        waypoint_y: 1,
        ..ship_1
    };

    ship_1.execute_instructions(&instructions, false);
    println!(
        "Ship location (Part 1) :\n\tBearing: {}\n\tCoordinates: {}, {}",
        ship_1.bearing, ship_1.x, ship_1.y
    );
    println!(
        "Ship Manhattan distance (Part 1): {}",
        manhattan_distance(ship_1.x, ship_1.y)
    );

    println!();

    ship_2.execute_instructions(&instructions, true);
    println!(
        "Ship location (Part 2) :\n\tBearing: {}\n\tCoordinates: {}, {}",
        ship_2.bearing, ship_2.x, ship_2.y
    );
    println!(
        "Ship Manhattan distance (Part 2): {}",
        manhattan_distance(ship_2.x, ship_2.y)
    );
}
