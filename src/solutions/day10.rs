use super::final_answer;
use super::input_raw;

const DAY: u8 = 10;

async fn input(example: bool) -> Vec<Vec<char>> {
    let raw = input_raw(DAY, example).await;
    let lines: Vec<String> = raw
        .lines()
        .map(|item| item.to_owned())
        .filter(|item| item.len() > 0)
        .collect();

    lines.iter().map(|item| item.chars().collect()).collect()
}

const NS: char = '|';
const EW: char = '-';
const NE: char = 'L';
const NW: char = 'J';
const SW: char = '7';
const SE: char = 'F';

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
struct Point {
    x: usize,
    y: usize,
}

enum Dir {
    North,
    South,
    East,
    West,

    None,
}

struct Traveler {
    came_from: Dir,
    pos: Point,
}
impl Traveler {
    fn step(&mut self, dir: Dir) {
        match dir {
            Dir::North => {
                self.pos.y -= 1;
                self.came_from = Dir::South;
                // println!("Stepped N");
            }
            Dir::South => {
                self.pos.y += 1;
                self.came_from = Dir::North;
                // println!("Stepped S");
            }
            Dir::East => {
                self.pos.x += 1;
                self.came_from = Dir::West;
                // println!("Stepped E");
            }
            Dir::West => {
                self.pos.x -= 1;
                self.came_from = Dir::East;
                // println!("Stepped W");
            }
            Dir::None => {}
        }
    }
    fn take_next_step(&mut self, char_at: char) {
        match char_at {
            NE => match self.came_from {
                Dir::North | Dir::None => self.step(Dir::East),
                Dir::East => self.step(Dir::North),
                _ => {}
            },
            NS => match self.came_from {
                Dir::North | Dir::None => self.step(Dir::South),
                Dir::South => self.step(Dir::North),
                _ => {}
            },
            NW => match self.came_from {
                Dir::North | Dir::None => self.step(Dir::West),
                Dir::West => self.step(Dir::North),
                _ => {}
            },
            SE => match self.came_from {
                Dir::South | Dir::None => self.step(Dir::East),
                Dir::East => self.step(Dir::South),
                _ => {}
            },
            SW => match self.came_from {
                Dir::South | Dir::None => self.step(Dir::West),
                Dir::West => self.step(Dir::South),
                _ => {}
            },
            EW => match self.came_from {
                Dir::East | Dir::None => self.step(Dir::West),
                Dir::West => self.step(Dir::East),
                _ => {}
            },
            _ => {}
        }
    }
}

fn get_start(input: &Vec<Vec<char>>) -> Point {
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if input[y][x] == 'S' {
                return Point { x, y };
            }
        }
    }
    Point { x: 0, y: 0 }
}

fn replace_start_with_valid_tile(input: &mut Vec<Vec<char>>, pos: Point) {
    println!("TEST: {:?}", pos);
    let (mut north, mut east, mut south, mut west) = (false, false, false, false);
    // test tile north
    if pos.y > 0 {
        if [NS, SW, SE].contains(&input[pos.y - 1][pos.x]) {
            north = true;
        }
    }
    // test tile south
    if pos.y < input.len() - 1 {
        if [NS, NW, NE].contains(&input[pos.y + 1][pos.x]) {
            south = true;
        }
    }
    // test tile east
    if pos.x > 0 {
        if [NW, SW, EW].contains(&input[pos.y][pos.x + 1]) {
            east = true;
        }
    }
    // test tile west
    if pos.x < input[0].len() - 1 {
        if [NE, SE, EW].contains(&input[pos.y][pos.x - 1]) {
            west = true;
        }
    }
    if north && south {
        input[pos.y][pos.x] = NS;
        println!("Starting tile is now {}", NS);
    } else if north && east {
        input[pos.y][pos.x] = NE;
        println!("Starting tile is now {}", NE);
    } else if north && west {
        input[pos.y][pos.x] = NW;
        println!("Starting tile is now {}", NW);
    } else if south && east {
        input[pos.y][pos.x] = SE;
        println!("Starting tile is now {}", SE);
    } else if south && west {
        input[pos.y][pos.x] = SW;
        println!("Starting tile is now {}", NW);
    } else if east && west {
        input[pos.y][pos.x] = EW;
        println!("Starting tile is now {}", EW);
    } else {
        panic!("Failed to find start!");
    }
}

pub async fn d10s1(submit: bool, example: bool) {
    let mut input = input(example).await;
    let start = get_start(&input);
    replace_start_with_valid_tile(&mut input, start);

    let mut traveler = Traveler {
        came_from: Dir::None,
        pos: start,
    };

    let mut steps = 0;
    while steps == 0 || traveler.pos != start {
        traveler.take_next_step(input[traveler.pos.y][traveler.pos.x]);
        steps += 1;
    }

    final_answer(steps / 2, submit, DAY, 1).await;
}

// =============================================================================

#[derive(Clone, Copy, PartialEq, Eq)]
enum Pixel {
    Empty,
    TraveledPipe,
    UntouchedPipe,
    Flooded,
}

fn generate_pixel_map(input: &Vec<Vec<char>>) -> Vec<Vec<Pixel>> {
    let mut pixel_map: Vec<Vec<Pixel>> = Vec::with_capacity(input.len() * 3);
    for _y in 0..input.len() {
        pixel_map.push(vec![Pixel::Empty; input[0].len() * 3]);
        pixel_map.push(vec![Pixel::Empty; input[0].len() * 3]);
        pixel_map.push(vec![Pixel::Empty; input[0].len() * 3]);
    }
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            match input[y][x] {
                NS => {
                    pixel_map[y * 3][x * 3 + 1] = Pixel::UntouchedPipe;
                    pixel_map[y * 3 + 1][x * 3 + 1] = Pixel::UntouchedPipe;
                    pixel_map[y * 3 + 2][x * 3 + 1] = Pixel::UntouchedPipe;
                }
                NE => {
                    pixel_map[y * 3][x * 3 + 1] = Pixel::UntouchedPipe;
                    pixel_map[y * 3 + 1][x * 3 + 1] = Pixel::UntouchedPipe;
                    pixel_map[y * 3 + 1][x * 3 + 2] = Pixel::UntouchedPipe;
                }
                NW => {
                    pixel_map[y * 3][x * 3 + 1] = Pixel::UntouchedPipe;
                    pixel_map[y * 3 + 1][x * 3 + 1] = Pixel::UntouchedPipe;
                    pixel_map[y * 3 + 1][x * 3] = Pixel::UntouchedPipe;
                }
                SE => {
                    pixel_map[y * 3 + 2][x * 3 + 1] = Pixel::UntouchedPipe;
                    pixel_map[y * 3 + 1][x * 3 + 1] = Pixel::UntouchedPipe;
                    pixel_map[y * 3 + 1][x * 3 + 2] = Pixel::UntouchedPipe;
                }
                SW => {
                    pixel_map[y * 3 + 2][x * 3 + 1] = Pixel::UntouchedPipe;
                    pixel_map[y * 3 + 1][x * 3 + 1] = Pixel::UntouchedPipe;
                    pixel_map[y * 3 + 1][x * 3] = Pixel::UntouchedPipe;
                }
                EW => {
                    pixel_map[y * 3 + 1][x * 3 + 2] = Pixel::UntouchedPipe;
                    pixel_map[y * 3 + 1][x * 3 + 1] = Pixel::UntouchedPipe;
                    pixel_map[y * 3 + 1][x * 3] = Pixel::UntouchedPipe;
                }
                _ => {}
            }
        }
    }

    pixel_map
}

fn travel_pixel_map(pixel_map: &mut Vec<Vec<Pixel>>, input: &Vec<Vec<char>>, pos: Point) {
    match input[pos.y][pos.x] {
        NS => {
            pixel_map[pos.y * 3][pos.x * 3 + 1] = Pixel::TraveledPipe;
            pixel_map[pos.y * 3 + 1][pos.x * 3 + 1] = Pixel::TraveledPipe;
            pixel_map[pos.y * 3 + 2][pos.x * 3 + 1] = Pixel::TraveledPipe;
        }
        NE => {
            pixel_map[pos.y * 3][pos.x * 3 + 1] = Pixel::TraveledPipe;
            pixel_map[pos.y * 3 + 1][pos.x * 3 + 1] = Pixel::TraveledPipe;
            pixel_map[pos.y * 3 + 1][pos.x * 3 + 2] = Pixel::TraveledPipe;
        }
        NW => {
            pixel_map[pos.y * 3][pos.x * 3 + 1] = Pixel::TraveledPipe;
            pixel_map[pos.y * 3 + 1][pos.x * 3 + 1] = Pixel::TraveledPipe;
            pixel_map[pos.y * 3 + 1][pos.x * 3] = Pixel::TraveledPipe;
        }
        SE => {
            pixel_map[pos.y * 3 + 2][pos.x * 3 + 1] = Pixel::TraveledPipe;
            pixel_map[pos.y * 3 + 1][pos.x * 3 + 1] = Pixel::TraveledPipe;
            pixel_map[pos.y * 3 + 1][pos.x * 3 + 2] = Pixel::TraveledPipe;
        }
        SW => {
            pixel_map[pos.y * 3 + 2][pos.x * 3 + 1] = Pixel::TraveledPipe;
            pixel_map[pos.y * 3 + 1][pos.x * 3 + 1] = Pixel::TraveledPipe;
            pixel_map[pos.y * 3 + 1][pos.x * 3] = Pixel::TraveledPipe;
        }
        EW => {
            pixel_map[pos.y * 3 + 1][pos.x * 3 + 2] = Pixel::TraveledPipe;
            pixel_map[pos.y * 3 + 1][pos.x * 3 + 1] = Pixel::TraveledPipe;
            pixel_map[pos.y * 3 + 1][pos.x * 3] = Pixel::TraveledPipe;
        }
        _ => {}
    }
}

pub async fn d10s2(submit: bool, example: bool) {
    let mut input = input(example).await;
    let start = get_start(&input);
    replace_start_with_valid_tile(&mut input, start);
    let mut pixel_map = generate_pixel_map(&input);

    let mut traveler = Traveler {
        came_from: Dir::None,
        pos: start,
    };

    let mut steps = 0;
    while steps == 0 || traveler.pos != start {
        traveler.take_next_step(input[traveler.pos.y][traveler.pos.x]);
        travel_pixel_map(&mut pixel_map, &input, traveler.pos);
        steps += 1;
    }
    // debug_pixels(&pixel_map);
    flood_pixel_map(&mut pixel_map);
    // debug_pixels(&pixel_map);

    let flooded_tiles = calc_flooded_tiles(&pixel_map);

    final_answer(flooded_tiles, submit, DAY, 2).await;
}

fn calc_flooded_tiles(pixel_map: &Vec<Vec<Pixel>>) -> usize {
    let mut ans = 0;

    for y in 0..pixel_map.len() / 3 {
        for x in 0..pixel_map[0].len() / 3 {
            if pixel_map[y * 3 + 1][x * 3 + 1] == Pixel::Flooded {
                ans += 1;
            }
        }
    }

    ans
}

fn flood_pixel_map(pixel_map: &mut Vec<Vec<Pixel>>) {
    let mut first_traveled = Point { x: 0, y: 0 };
    'find: for y in 0..pixel_map.len() {
        for x in 0..pixel_map[0].len() {
            match pixel_map[y][x] {
                Pixel::TraveledPipe => {
                    first_traveled = Point { x: x + 1, y: y + 1 };
                    break 'find;
                }
                _ => {}
            }
        }
    }

    recursive_flood(pixel_map, first_traveled);
}

fn recursive_flood(pixel_map: &mut Vec<Vec<Pixel>>, pos: Point) {
    match pixel_map[pos.y][pos.x] {
        Pixel::Empty | Pixel::UntouchedPipe => {
            pixel_map[pos.y][pos.x] = Pixel::Flooded;
            // flood north
            if pos.y > 0 {
                match pixel_map[pos.y - 1][pos.x] {
                    Pixel::Empty | Pixel::UntouchedPipe => {
                        recursive_flood(
                            pixel_map,
                            Point {
                                y: pos.y - 1,
                                x: pos.x,
                            },
                        );
                    }
                    _ => {}
                }
            }
            // flood south
            if pos.y < pixel_map.len() - 1 {
                match pixel_map[pos.y + 1][pos.x] {
                    Pixel::Empty | Pixel::UntouchedPipe => {
                        recursive_flood(
                            pixel_map,
                            Point {
                                y: pos.y + 1,
                                x: pos.x,
                            },
                        );
                    }
                    _ => {}
                }
            }
            // flood east
            if pos.x < pixel_map[0].len() - 1 {
                match pixel_map[pos.y][pos.x + 1] {
                    Pixel::Empty | Pixel::UntouchedPipe => {
                        recursive_flood(
                            pixel_map,
                            Point {
                                y: pos.y,
                                x: pos.x + 1,
                            },
                        );
                    }
                    _ => {}
                }
            }
            // flood west
            if pos.x > 0 {
                match pixel_map[pos.y][pos.x - 1] {
                    Pixel::Empty | Pixel::UntouchedPipe => {
                        recursive_flood(
                            pixel_map,
                            Point {
                                y: pos.y,
                                x: pos.x - 1,
                            },
                        );
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }
}

#[allow(dead_code)]
fn debug_pixels(pixel_map: &Vec<Vec<Pixel>>) {
    for y in 0..pixel_map.len() {
        for x in 0..pixel_map[0].len() {
            match pixel_map[y][x] {
                Pixel::Empty => print!(" "),
                Pixel::TraveledPipe => print!("X"),
                Pixel::UntouchedPipe => print!("#"),
                Pixel::Flooded => print!("I"),
            }
        }
        println!("");
    }
}
