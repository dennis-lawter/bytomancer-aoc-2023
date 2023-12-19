use regex::Regex;

use super::final_answer;
use super::input_raw;

const DAY: u8 = 18;

async fn input(example: bool) -> Vec<String> {
    let raw = input_raw(DAY, example).await;
    let lines = raw
        .lines()
        .map(|item| item.to_owned())
        .filter(|item| item.len() > 0)
        .collect();

    lines
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}
impl Direction {
    fn from_str(input: &str) -> Self {
        match input {
            "U" | "3" => Self::North,
            "R" | "0" => Self::East,
            "D" | "1" => Self::South,
            "L" | "2" => Self::West,
            bad_input => panic!("Not a valid direction string: {}", bad_input),
        }
    }
}

struct DigPlan {
    dir: Direction,
    dist: u32,
    color_dist: u32,
    color_dir: Direction,
}
impl DigPlan {
    fn from_str(input: &str) -> Self {
        let regex = Regex::new(r#"^(.) (\d+) \(#(.+)(\d)\)$"#).unwrap();
        let captures = regex.captures(input).unwrap();
        let dir = Direction::from_str(captures.get(1).unwrap().as_str());
        let distance = str::parse::<u32>(captures.get(2).unwrap().as_str()).unwrap();
        let color_dist = u32::from_str_radix(captures.get(3).unwrap().as_str(), 16).unwrap();
        let color_dir = Direction::from_str(captures.get(4).unwrap().as_str());

        Self {
            dir,
            dist: distance,
            color_dist,
            color_dir,
        }
    }
}

// fn debug_grid(grid: &Vec<Vec<bool>>) {
//     for y in 0..grid.len() {
//         for x in 0..grid[0].len() {
//             if grid[y][x] {
//                 print!("#");
//             } else {
//                 print!(".");
//             }
//         }
//         println!();
//     }
// }

// fn find_filled(grid: &Vec<Vec<bool>>) -> usize {
//     let mut count = 0;

//     for y in 0..grid.len() {
//         for x in 0..grid[0].len() {
//             if grid[y][x] {
//                 count += 1;
//             }
//         }
//     }

//     count
// }

// fn flood_grid(grid: &mut Vec<Vec<bool>>, flood_at: (usize, usize)) {
//     let y = flood_at.1;
//     let x = flood_at.0;
//     grid[y][x] = true;
//     if y > 0 {
//         if grid[y - 1][x] == false {
//             flood_grid(grid, (x, y - 1));
//         }
//     }
//     if y < grid.len() - 1 {
//         if grid[y + 1][x] == false {
//             flood_grid(grid, (x, y + 1));
//         }
//     }
//     if x > 0 {
//         if grid[y][x - 1] == false {
//             flood_grid(grid, (x - 1, y));
//         }
//     }
//     if x < grid[0].len() - 1 {
//         if grid[y][x + 1] == false {
//             flood_grid(grid, (x + 1, y));
//         }
//     }

//     // println!();
//     // debug_grid(grid.as_ref());
//     // println!();
// }

#[derive(Clone, Debug)]
struct Point {
    x: i64,
    y: i64,
}
impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

struct Polygon {
    data: Vec<Point>,
}
impl Polygon {
    fn new(data: Vec<Point>) -> Self {
        let mut data = data;
        if !Self::is_clockwise(&data) {
            data.reverse();
        }
        Self { data }
    }

    fn is_clockwise(data: &Vec<Point>) -> bool {
        // find the top-most Y value
        let mut smallest_y = data[0].y;
        for i in 0..data.len() {
            if data[i].y < smallest_y {
                smallest_y = data[i].y;
            }
        }

        // get all indexes that reference a point on the "top line"
        let mut indexes_on_the_top_line = vec![];
        for i in 0..data.len() {
            if data[i].y == smallest_y {
                indexes_on_the_top_line.push(i);
            }
        }

        // of all the "top line" points, find the left-most one
        let mut left_most_top_line_point_index = indexes_on_the_top_line[0];
        for i in indexes_on_the_top_line {
            if data[i].x < data[left_most_top_line_point_index].x {
                left_most_top_line_point_index = i;
            }
        }

        // wrapping i+1
        let next_i = if left_most_top_line_point_index == data.len() - 1 {
            0
        } else {
            left_most_top_line_point_index + 1
        };

        // this might be conjecture but I think this has to be true?

        // if the point after the point on the top line furthest left is also
        // on the top line (same y), then the polygon must be in CW order
        data[left_most_top_line_point_index].y == data[next_i].y
    }

    fn cw_i(&self, i: usize) -> usize {
        if i == self.data.len() - 1 {
            0
        } else {
            i + 1
        }
    }

    fn ccw_i(&self, i: usize) -> usize {
        if i == 0 {
            self.data.len() - 1
        } else {
            i - 1
        }
    }

    fn slice_to_calculate_area(&mut self) -> usize {
        let mut area = 0;
        // for debugging to stop after so many iterations
        // let mut futility = 100;
        while self.data.len() > 4 {
            println!("\n\n========================================");
            println!("Data: {:?}", self.data);
            println!("Data len: {}", self.data.len());
            println!("Area: {}", area);

            // futility -= 1;
            // if futility == 0 {
            //     panic!("FUTILE");
            // }

            // let bounds = (self.min_xs(), self.min_ys(), self.max_xs(), self.max_ys());
            // println!(
            //     "Bounds: ({}, {})\t({}, {})",
            //     bounds.0 .0, bounds.1 .0, bounds.2 .0, bounds.3 .0
            // );

            // count the nubs on the N, E, S, and W sides
            // let north_nubs = self.count_north_nubs();
            // let east_nubs = self.count_east_nubs();
            // let south_nubs = self.count_south_nubs();
            // let west_nubs = self.count_west_nubs();
            // println!(
            //     "Nub counts: {}N\t{}E\t{}S\t{}W",
            //     north_nubs, east_nubs, south_nubs, west_nubs
            // );
            let north_faces = self.count_faces_moving_east();
            let east_faces = self.count_faces_moving_south();
            let south_faces = self.count_faces_moving_west();
            let west_faces = self.count_faces_moving_north();
            println!(
                "Face counts: {} N\t{} E\t{} S\t{} W",
                north_faces, east_faces, south_faces, west_faces
            );

            // let slice_count: usize;
            // whichever side has the most nubs gets a slice taken off
            let slice_from_dir = if north_faces >= east_faces
                && north_faces >= south_faces
                && north_faces >= west_faces
            {
                // slice_count = north_faces / 2;
                Direction::North
            } else if east_faces >= north_faces
                && east_faces >= south_faces
                && east_faces >= west_faces
            {
                // slice_count = east_faces / 2;
                Direction::East
            } else if south_faces >= north_faces
                && south_faces >= east_faces
                && south_faces >= west_faces
            {
                // slice_count = south_faces / 2;
                Direction::South
            } else {
                // slice_count = west_faces / 2;
                Direction::West
            };

            let slice_count = 1;

            area += self.perform_slices(slice_from_dir, slice_count);
        }
        assert_eq!(self.data.len(), 4);
        let final_rect_area = Self::calc_rectangle_area(&self.data[0], &self.data[2]);
        println!("\n========================================");
        println!("Data: {:?}", self.data);
        println!("Data len: {}", self.data.len());
        println!("Area: {}", area);
        println!("Final rect area: {}", final_rect_area);
        area += final_rect_area;
        area
    }

    fn count_faces_moving_north(&self) -> usize {
        let mut count = 0;
        let last_i = self.data.len() - 1;
        if self.data[last_i].y > self.data[0].y {
            count += 1;
        }
        for i in 1..self.data.len() {
            if self.data[i - 1].y > self.data[i].y {
                count += 1;
            }
        }

        count
    }

    fn count_faces_moving_east(&self) -> usize {
        let mut count = 0;
        let last_i = self.data.len() - 1;
        if self.data[last_i].x < self.data[0].x {
            // println!(
            //     "EAST FACE {}: {:?}\t{:?}",
            //     0, self.data[last_i], self.data[0]
            // );
            count += 1;
        }
        for i in 1..self.data.len() {
            if self.data[i - 1].x < self.data[i].x {
                // println!(
                //     "EAST FACE {}: {:?}\t{:?}",
                //     i,
                //     self.data[i - 1],
                //     self.data[i]
                // );
                count += 1;
            }
        }

        count
    }

    fn count_faces_moving_south(&self) -> usize {
        let mut count = 0;
        let last_i = self.data.len() - 1;
        if self.data[last_i].y < self.data[0].y {
            count += 1;
        }
        for i in 1..self.data.len() {
            if self.data[i - 1].y < self.data[i].y {
                count += 1;
            }
        }

        count
    }

    fn count_faces_moving_west(&self) -> usize {
        let mut count = 0;
        let last_i = self.data.len() - 1;
        if self.data[last_i].x > self.data[0].x {
            count += 1;
        }
        for i in 1..self.data.len() {
            if self.data[i - 1].x > self.data[i].x {
                count += 1;
            }
        }

        count
    }

    // fn count_north_nubs(&self) -> usize {
    //     let mut count = 0;
    //     let (min_y, _) = self.min_ys();
    //     for i in 0..self.data.len() {
    //         if self.data[i].y == min_y {
    //             count += 1;
    //         }
    //     }
    //     count
    // }

    // fn count_east_nubs(&self) -> usize {
    //     let mut count = 0;
    //     let (max_x, _) = self.max_xs();
    //     for i in 0..self.data.len() {
    //         if self.data[i].x == max_x {
    //             count += 1;
    //         }
    //     }
    //     count
    // }

    // fn count_south_nubs(&self) -> usize {
    //     let mut count = 0;
    //     let (max_y, _) = self.max_ys();
    //     for i in 0..self.data.len() {
    //         if self.data[i].y == max_y {
    //             count += 1;
    //         }
    //     }
    //     count
    // }

    // fn count_west_nubs(&self) -> usize {
    //     let mut count = 0;
    //     let (min_x, _) = self.min_xs();
    //     for i in 0..self.data.len() {
    //         if self.data[i].x == min_x {
    //             count += 1;
    //         }
    //     }
    //     count
    // }

    fn min_xs(&self) -> (i64, i64) {
        let mut min_x = i64::MAX;
        let mut second_min_x = i64::MAX;
        for i in 0..self.data.len() {
            if self.data[i].x <= min_x {
                min_x = self.data[i].x;
            }
        }
        for i in 0..self.data.len() {
            if self.data[i].x != min_x {
                if self.data[i].x <= second_min_x {
                    second_min_x = self.data[i].x;
                }
            }
        }
        (min_x, second_min_x)
    }

    fn max_xs(&self) -> (i64, i64) {
        let mut max_x = i64::MIN;
        let mut second_max_x = i64::MIN;
        for i in 0..self.data.len() {
            if self.data[i].x >= max_x {
                max_x = self.data[i].x;
            }
        }
        for i in 0..self.data.len() {
            if self.data[i].x != max_x {
                if self.data[i].x >= second_max_x {
                    second_max_x = self.data[i].x;
                }
            }
        }
        (max_x, second_max_x)
    }

    fn min_ys(&self) -> (i64, i64) {
        let mut min_y = i64::MAX;
        let mut second_min_y = i64::MAX;
        for i in 0..self.data.len() {
            if self.data[i].y <= min_y {
                min_y = self.data[i].y;
            }
        }
        for i in 0..self.data.len() {
            if self.data[i].y != min_y {
                if self.data[i].y <= second_min_y {
                    second_min_y = self.data[i].y;
                }
            }
        }
        (min_y, second_min_y)
    }

    fn max_ys(&self) -> (i64, i64) {
        let mut max_y = i64::MIN;
        let mut second_max_y = i64::MIN;
        for i in 0..self.data.len() {
            if self.data[i].y >= max_y {
                max_y = self.data[i].y;
            }
        }
        for i in 0..self.data.len() {
            if self.data[i].y != max_y {
                if self.data[i].y >= second_max_y {
                    second_max_y = self.data[i].y;
                }
            }
        }
        (max_y, second_max_y)
    }

    fn perform_slices(&mut self, slice_from_dir: Direction, count: usize) -> usize {
        let mut accum = 0;
        for _ in 0..count {
            accum += match slice_from_dir {
                Direction::North => self.perform_north_slice(),
                Direction::East => self.perform_east_slice(),
                Direction::South => self.perform_south_slice(),
                Direction::West => self.perform_west_slice(),
            };
        }
        accum
    }

    fn calc_rectangle_area(p0: &Point, p1: &Point) -> usize {
        ((p1.x - p0.x).abs() * (p1.y - p0.y).abs()) as usize
    }

    fn perform_north_slice(&mut self) -> usize {
        println!("North slice");
        let (min_y, second_min_y) = self.min_ys();

        // get all indexes that reference a point on the "top line"
        let mut indexes_on_the_top_line = vec![];
        for i in 0..self.data.len() {
            if self.data[i].y == min_y {
                indexes_on_the_top_line.push(i);
            }
        }

        // get the left most point
        let mut top_left_corner_i = indexes_on_the_top_line[0];
        for i in indexes_on_the_top_line {
            if self.data[i].x < self.data[top_left_corner_i].x {
                top_left_corner_i = i;
            }
        }

        let bottom_left_leg_i = self.ccw_i(top_left_corner_i);
        let top_right_corner_i = self.cw_i(top_left_corner_i);
        let bottom_right_leg_i = self.cw_i(top_right_corner_i);

        let bottom_left_leg = self.data[bottom_left_leg_i].clone();
        let top_left_corner = self.data[top_left_corner_i].clone();
        let top_right_corner = self.data[top_right_corner_i].clone();
        let bottom_right_leg = self.data[bottom_right_leg_i].clone();

        println!("Bounds: {}\t{}", min_y, second_min_y);

        println!(
            "Corners: {:?}\t{:?}\t{:?}\t{:?}",
            bottom_left_leg, top_left_corner, top_right_corner, bottom_right_leg
        );

        let mut remove_is = vec![];

        if bottom_left_leg.y == second_min_y && bottom_right_leg.y == second_min_y {
            // perfect square lump removal
            remove_is.push(top_left_corner_i);
            remove_is.push(bottom_left_leg_i);
            remove_is.push(top_right_corner_i);
            remove_is.push(bottom_right_leg_i);
        } else {
            if bottom_left_leg.y != second_min_y {
                let bottom_left_cut_point = Point::new(bottom_left_leg.x, second_min_y);
                self.data[top_left_corner_i] = bottom_left_cut_point;
            } else {
                remove_is.push(top_left_corner_i);
                remove_is.push(bottom_left_leg_i);
            }

            if bottom_right_leg.y != second_min_y {
                let bottom_right_cut_point = Point::new(bottom_right_leg.x, second_min_y);
                self.data[top_right_corner_i] = bottom_right_cut_point;
            } else {
                remove_is.push(top_right_corner_i);
                remove_is.push(bottom_right_leg_i);
            }
        }

        self.remove_indexes(remove_is);

        let width = top_right_corner.x - top_left_corner.x;
        let height = second_min_y - min_y;

        println!("Gained {} area", (width.abs() * height.abs()) as usize);

        (width.abs() * height.abs()) as usize
    }

    fn perform_east_slice(&mut self) -> usize {
        println!("East slice");
        let (max_x, second_max_x) = self.max_xs();

        // get all indexes that reference a point on the "right line"
        let mut indexes_on_the_right_line = vec![];
        for i in 0..self.data.len() {
            if self.data[i].x == max_x {
                indexes_on_the_right_line.push(i);
            }
        }

        // get the top most point
        let mut top_right_corner_i = indexes_on_the_right_line[0];
        for i in indexes_on_the_right_line {
            if self.data[i].y < self.data[top_right_corner_i].y {
                top_right_corner_i = i;
            }
        }

        let top_left_leg_i = self.ccw_i(top_right_corner_i);
        let bottom_right_corner_i = self.cw_i(top_right_corner_i);
        let bottom_left_leg_i = self.cw_i(bottom_right_corner_i);

        let top_right_corner = self.data[top_right_corner_i].clone();
        let bottom_right_corner = self.data[bottom_right_corner_i].clone();
        let bottom_left_leg = self.data[bottom_left_leg_i].clone();
        let top_left_leg = self.data[top_left_leg_i].clone();

        println!("Bounds: {}\t{}", max_x, second_max_x);

        println!(
            "Corners: {:?}\t{:?}\t{:?}\t{:?}",
            top_left_leg, top_right_corner, bottom_right_corner, bottom_left_leg
        );

        let mut remove_is = vec![];

        if top_left_leg.x == second_max_x && bottom_left_leg.x == second_max_x {
            remove_is.push(top_right_corner_i);
            remove_is.push(top_left_leg_i);
            remove_is.push(bottom_right_corner_i);
            remove_is.push(bottom_left_leg_i);
        } else {
            if top_left_leg.x != second_max_x {
                let top_left_cut_point = Point::new(second_max_x, top_left_leg.y);
                // move the top right corner left
                self.data[top_right_corner_i] = top_left_cut_point;
            } else {
                remove_is.push(top_right_corner_i);
                remove_is.push(top_left_leg_i);
            }

            if bottom_left_leg.x != second_max_x {
                let bottom_left_cut_point = Point::new(second_max_x, bottom_left_leg.y);
                // move the bottom right corner left
                self.data[bottom_right_corner_i] = bottom_left_cut_point;
            } else {
                remove_is.push(bottom_right_corner_i);
                remove_is.push(bottom_left_leg_i);
            }
        }

        self.remove_indexes(remove_is);

        let width = max_x - second_max_x;
        let height = bottom_right_corner.y - top_right_corner.y;

        println!("Gained {} area", (width.abs() * height.abs()) as usize);

        (width.abs() * height.abs()) as usize
    }

    fn perform_south_slice(&mut self) -> usize {
        println!("South slice");
        let (max_y, second_max_y) = self.max_ys();

        // get all indexes that reference a point on the "bottom line"
        let mut indexes_on_the_bottom_line = vec![];
        for i in 0..self.data.len() {
            if self.data[i].y == max_y {
                indexes_on_the_bottom_line.push(i);
            }
        }

        // get the right most point
        let mut bottom_right_corner_i = indexes_on_the_bottom_line[0];
        for i in indexes_on_the_bottom_line {
            if self.data[i].x > self.data[bottom_right_corner_i].x {
                bottom_right_corner_i = i;
            }
        }

        let top_right_leg_i = self.ccw_i(bottom_right_corner_i);
        let bottom_left_corner_i = self.cw_i(bottom_right_corner_i);
        let top_left_leg_i = self.cw_i(bottom_left_corner_i);

        let top_right_leg = self.data[top_right_leg_i].clone();
        let bottom_right_corner = self.data[bottom_right_corner_i].clone();
        let bottom_left_corner = self.data[bottom_left_corner_i].clone();
        let top_left_leg = self.data[top_left_leg_i].clone();

        println!("Bounds: {}\t{}", max_y, second_max_y);

        println!(
            "Corners: {:?}\t{:?}\t{:?}\t{:?}",
            top_right_leg, bottom_right_corner, bottom_left_corner, top_left_leg
        );

        let mut remove_is = vec![];

        if top_right_leg.y == second_max_y && top_left_leg.y == second_max_y {
            remove_is.push(bottom_right_corner_i);
            remove_is.push(top_right_leg_i);
            remove_is.push(bottom_left_corner_i);
            remove_is.push(top_left_leg_i);
        } else {
            if top_right_leg.y != second_max_y {
                let bottom_left_cut_point = Point::new(top_right_leg.x, second_max_y);
                self.data[bottom_right_corner_i] = bottom_left_cut_point;
            } else {
                remove_is.push(bottom_right_corner_i);
                remove_is.push(top_right_leg_i);
            }

            if top_left_leg.y != second_max_y {
                let bottom_right_cut_point = Point::new(top_left_leg.x, second_max_y);
                self.data[bottom_left_corner_i] = bottom_right_cut_point;
            } else {
                remove_is.push(bottom_left_corner_i);
                remove_is.push(top_left_leg_i);
            }
        }

        self.remove_indexes(remove_is);

        let width = bottom_left_corner.x - bottom_right_corner.x;
        let height = max_y - second_max_y;

        println!("Gained {} area", (width.abs() * height.abs()) as usize);

        (width.abs() * height.abs()) as usize
    }

    fn perform_west_slice(&mut self) -> usize {
        println!("West slice");
        let (min_x, second_min_x) = self.min_xs();

        // get all indexes that reference a point on the "left line"
        let mut indexes_on_the_left_line = vec![];
        for i in 0..self.data.len() {
            if self.data[i].x == min_x {
                indexes_on_the_left_line.push(i);
            }
        }

        // get the bottom most point
        let mut bottom_left_corner_i = indexes_on_the_left_line[0];
        for i in indexes_on_the_left_line {
            if self.data[i].y > self.data[bottom_left_corner_i].y {
                bottom_left_corner_i = i;
            }
        }

        let top_left_corner_i = self.cw_i(bottom_left_corner_i);
        let top_right_leg_i = self.cw_i(top_left_corner_i);
        let bottom_right_leg_i = self.ccw_i(bottom_left_corner_i);
        let bottom_left_corner = self.data[bottom_left_corner_i].clone();
        let top_left_corner = self.data[top_left_corner_i].clone();
        let bottom_right_leg = self.data[bottom_right_leg_i].clone();
        let top_right_leg = self.data[top_right_leg_i].clone();

        println!("Bounds: {}\t{}", min_x, second_min_x);

        println!(
            "Corners: {:?}\t{:?}\t{:?}\t{:?}",
            bottom_right_leg, bottom_left_corner, top_left_corner, top_right_leg
        );

        let mut remove_is = vec![];

        if top_right_leg.x == second_min_x && bottom_right_leg.x == second_min_x {
            remove_is.push(top_left_corner_i);
            remove_is.push(top_right_leg_i);
            remove_is.push(bottom_left_corner_i);
            remove_is.push(bottom_right_leg_i);
        } else {
            if top_right_leg.x != second_min_x {
                let top_right_cut_point = Point::new(second_min_x, top_right_leg.y);
                // move the top left corner to the right
                self.data[top_left_corner_i] = top_right_cut_point;
            } else {
                remove_is.push(top_left_corner_i);
                remove_is.push(top_right_leg_i);
            }

            if bottom_right_leg.x != second_min_x {
                let bottom_right_cut_point = Point::new(second_min_x, bottom_right_leg.y);
                // move the bottom left corner to the right
                self.data[bottom_left_corner_i] = bottom_right_cut_point;
            } else {
                remove_is.push(bottom_left_corner_i);
                remove_is.push(bottom_right_leg_i);
            }
        }

        self.remove_indexes(remove_is);

        let width = second_min_x - min_x;
        let height = bottom_left_corner.y - top_left_corner.y;

        println!("Gained {} area", (width.abs() * height.abs()) as usize);

        (width.abs() * height.abs()) as usize
    }

    fn remove_indexes(&mut self, remove_is: Vec<usize>) {
        let mut remove_is = remove_is;
        remove_is.sort();
        remove_is.reverse();
        for i in remove_is {
            self.data.remove(i);
        }
    }
}

// enum Corners {
//     TopLeft,
//     TopRight,
//     BotRight,
//     BotLeft,
// }

#[derive(Clone, PartialEq, Eq)]
enum TurnDirection {
    Left,
    Right,
}
enum HandDirection {
    Inside,
    Outside,
}
impl HandDirection {
    fn inverse(&self) -> Self {
        match self {
            HandDirection::Inside => HandDirection::Outside,
            HandDirection::Outside => HandDirection::Inside,
        }
    }
}

// =============================================================================
// ENTRY POINTS
// =============================================================================

pub async fn d18s1(submit: bool, example: bool) {
    let lines = input(example).await;
    let mut points: Vec<Point> = Vec::with_capacity(lines.len());
    let mut cursor = Point::new(0, 0);
    let mut plans: Vec<DigPlan> = vec![];
    // let mut perimeter = 0;
    // let mut last_dir: Option<Direction> = None;
    for line in lines {
        let plan = DigPlan::from_str(line.as_str());
        if plans.len() > 0 {
            let last_index = plans.len() - 1;
            let last_plan = &mut plans[last_index];
            if plan.dir == last_plan.dir {
                last_plan.dist += plan.dist;
            } else {
                plans.push(plan);
            }
        } else {
            plans.push(plan);
        }
        // last_dir = Some(plan.dir.clone());
    }

    for plan in plans.iter() {
        // perimeter += plan.dist;
        match plan.dir {
            Direction::North => cursor.y -= plan.dist as i64,
            Direction::East => cursor.x += plan.dist as i64,
            Direction::South => cursor.y += plan.dist as i64,
            Direction::West => cursor.x -= plan.dist as i64,
        }
        points.push(cursor.clone());
    }
    if !Polygon::is_clockwise(&points) {
        points.reverse();
        plans.reverse();
    }

    // now we re-trace the points to circumscribe
    let temp_poly = Polygon::new(points);
    let mut circumscribed_points = vec![];
    let mut indexes_on_top_line = vec![];

    // find all points on the top line
    let (min_y, _) = temp_poly.min_ys();
    for i in 0..temp_poly.data.len() {
        if temp_poly.data[i].y == min_y {
            indexes_on_top_line.push(i);
        }
    }

    let mut left_most_point_i = indexes_on_top_line[0];
    for i in indexes_on_top_line {
        if temp_poly.data[i].x < temp_poly.data[left_most_point_i].x {
            left_most_point_i = i
        }
    }

    // so now we have the point at the top left!
    // we're actually going to start walking from the point after it though
    let end_i = left_most_point_i;
    let mut i = temp_poly.cw_i(left_most_point_i);
    // Since we are starting at a top-left corner,
    // advancing to the next turn,
    // and there are no points higher,
    // this position is guaranteed
    let mut hand_position = HandDirection::Outside;
    let mut turn_direction = TurnDirection::Right;
    let mut cursor = Point::new(0, 0);
    circumscribed_points.push(cursor.clone());
    while i != end_i {
        // println!("i {}", i);
        let prev_turn_direction = turn_direction.clone();
        let prev_point = &temp_poly.data[temp_poly.ccw_i(i)];
        let curr_point = &temp_poly.data[i];
        let next_point = &temp_poly.data[temp_poly.cw_i(i)];
        let dir_just_traveled = if prev_point.y > curr_point.y {
            Direction::North
        } else if prev_point.y < curr_point.y {
            Direction::South
        } else if prev_point.x > curr_point.x {
            Direction::West
        } else {
            Direction::East
        };
        let dir_to_next_point = if next_point.y > curr_point.y {
            Direction::South
        } else if next_point.y < curr_point.y {
            Direction::North
        } else if next_point.x > curr_point.x {
            Direction::East
        } else {
            Direction::West
        };

        match dir_just_traveled {
            Direction::North => match dir_to_next_point {
                Direction::North | Direction::South => panic!("Invalid turn"),
                Direction::East => turn_direction = TurnDirection::Right,
                Direction::West => turn_direction = TurnDirection::Left,
            },
            Direction::East => match dir_to_next_point {
                Direction::East | Direction::West => panic!("Invalid turn"),
                Direction::North => turn_direction = TurnDirection::Left,
                Direction::South => turn_direction = TurnDirection::Right,
            },
            Direction::South => match dir_to_next_point {
                Direction::South | Direction::North => panic!("Invalid turn"),
                Direction::East => turn_direction = TurnDirection::Left,
                Direction::West => turn_direction = TurnDirection::Right,
            },
            Direction::West => match dir_to_next_point {
                Direction::East | Direction::West => panic!("Invalid turn"),
                Direction::North => turn_direction = TurnDirection::Right,
                Direction::South => turn_direction = TurnDirection::Left,
            },
        }

        let mut distance_offset: i64 = 0;

        if turn_direction == prev_turn_direction {
            match hand_position {
                HandDirection::Inside => distance_offset = -1,
                HandDirection::Outside => distance_offset = 1,
            }
        } else {
            hand_position = hand_position.inverse();
        }

        match plans[i].dir {
            Direction::North => cursor.y -= plans[i].dist as i64 + distance_offset,
            Direction::East => cursor.x += plans[i].dist as i64 + distance_offset,
            Direction::South => cursor.y += plans[i].dist as i64 + distance_offset,
            Direction::West => cursor.x -= plans[i].dist as i64 + distance_offset,
        }
        circumscribed_points.push(cursor.clone());

        i = temp_poly.cw_i(i);
    }

    println!("ORIGINAL POINTS:\n{:?}\n", temp_poly.data);

    let mut poly = Polygon::new(circumscribed_points);

    println!("CIRCUMSCRIBED POINTS:\n{:?}\n", poly.data);
    let answer = poly.slice_to_calculate_area();
    final_answer(answer, submit, DAY, 1).await;
}

pub async fn d18s2(submit: bool, example: bool) {
    let lines = input(example).await;
    let mut points: Vec<Point> = Vec::with_capacity(lines.len());
    let mut cursor = Point::new(0, 0);
    let mut plans: Vec<DigPlan> = vec![];
    // let mut perimeter = 0;
    // let mut last_dir: Option<Direction> = None;
    for line in lines {
        let plan = DigPlan::from_str(line.as_str());
        if plans.len() > 0 {
            let last_index = plans.len() - 1;
            let last_plan = &mut plans[last_index];
            if plan.color_dir == last_plan.color_dir {
                last_plan.color_dist += plan.color_dist;
            } else {
                plans.push(plan);
            }
        } else {
            plans.push(plan);
        }
        // last_dir = Some(plan.color_dir.clone());
    }

    for plan in plans.iter() {
        // perimeter += plan.color_dist;
        match plan.color_dir {
            Direction::North => cursor.y -= plan.color_dist as i64,
            Direction::East => cursor.x += plan.color_dist as i64,
            Direction::South => cursor.y += plan.color_dist as i64,
            Direction::West => cursor.x -= plan.color_dist as i64,
        }
        points.push(cursor.clone());
    }
    if !Polygon::is_clockwise(&points) {
        points.reverse();
        plans.reverse();
    }

    // now we re-trace the points to circumscribe
    let temp_poly = Polygon::new(points);
    let mut circumscribed_points = vec![];
    let mut indexes_on_top_line = vec![];

    // find all points on the top line
    let (min_y, _) = temp_poly.min_ys();
    for i in 0..temp_poly.data.len() {
        if temp_poly.data[i].y == min_y {
            indexes_on_top_line.push(i);
        }
    }

    let mut left_most_point_i = indexes_on_top_line[0];
    for i in indexes_on_top_line {
        if temp_poly.data[i].x < temp_poly.data[left_most_point_i].x {
            left_most_point_i = i
        }
    }

    // so now we have the point at the top left!
    // we're actually going to start walking from the point after it though
    let end_i = left_most_point_i;
    let mut i = temp_poly.cw_i(left_most_point_i);
    // Since we are starting at a top-left corner,
    // advancing to the next turn,
    // and there are no points higher,
    // this position is guaranteed
    let mut hand_position = HandDirection::Outside;
    let mut turn_direction = TurnDirection::Right;
    let mut cursor = Point::new(0, 0);
    circumscribed_points.push(cursor.clone());
    while i != end_i {
        // println!("i {}", i);
        let prev_turn_direction = turn_direction.clone();
        let prev_point = &temp_poly.data[temp_poly.ccw_i(i)];
        let curr_point = &temp_poly.data[i];
        let next_point = &temp_poly.data[temp_poly.cw_i(i)];
        let dir_just_traveled = if prev_point.y > curr_point.y {
            Direction::North
        } else if prev_point.y < curr_point.y {
            Direction::South
        } else if prev_point.x > curr_point.x {
            Direction::West
        } else {
            Direction::East
        };
        let dir_to_next_point = if next_point.y > curr_point.y {
            Direction::South
        } else if next_point.y < curr_point.y {
            Direction::North
        } else if next_point.x > curr_point.x {
            Direction::East
        } else {
            Direction::West
        };

        match dir_just_traveled {
            Direction::North => match dir_to_next_point {
                Direction::North | Direction::South => panic!("Invalid turn"),
                Direction::East => turn_direction = TurnDirection::Right,
                Direction::West => turn_direction = TurnDirection::Left,
            },
            Direction::East => match dir_to_next_point {
                Direction::East | Direction::West => panic!("Invalid turn"),
                Direction::North => turn_direction = TurnDirection::Left,
                Direction::South => turn_direction = TurnDirection::Right,
            },
            Direction::South => match dir_to_next_point {
                Direction::South | Direction::North => panic!("Invalid turn"),
                Direction::East => turn_direction = TurnDirection::Left,
                Direction::West => turn_direction = TurnDirection::Right,
            },
            Direction::West => match dir_to_next_point {
                Direction::East | Direction::West => panic!("Invalid turn"),
                Direction::North => turn_direction = TurnDirection::Right,
                Direction::South => turn_direction = TurnDirection::Left,
            },
        }

        let mut color_distance_offset: i64 = 0;

        if turn_direction == prev_turn_direction {
            match hand_position {
                HandDirection::Inside => color_distance_offset = -1,
                HandDirection::Outside => color_distance_offset = 1,
            }
        } else {
            hand_position = hand_position.inverse();
        }

        match plans[i].color_dir {
            Direction::North => cursor.y -= plans[i].color_dist as i64 + color_distance_offset,
            Direction::East => cursor.x += plans[i].color_dist as i64 + color_distance_offset,
            Direction::South => cursor.y += plans[i].color_dist as i64 + color_distance_offset,
            Direction::West => cursor.x -= plans[i].color_dist as i64 + color_distance_offset,
        }
        circumscribed_points.push(cursor.clone());

        i = temp_poly.cw_i(i);
    }

    println!("ORIGINAL POINTS:\n{:?}\n", temp_poly.data);

    let mut poly = Polygon::new(circumscribed_points);

    println!("CIRCUMSCRIBED POINTS:\n{:?}\n", poly.data);
    let answer = poly.slice_to_calculate_area();
    final_answer(answer, submit, DAY, 2).await;
}
