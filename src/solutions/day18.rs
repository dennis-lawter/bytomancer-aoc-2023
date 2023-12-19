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

    // fn min_xs(&self) -> (i64, i64) {
    //     let mut min_x = i64::MAX;
    //     let mut second_min_x = i64::MAX;
    //     for i in 0..self.data.len() {
    //         if self.data[i].x <= min_x {
    //             min_x = self.data[i].x;
    //         }
    //     }
    //     for i in 0..self.data.len() {
    //         if self.data[i].x != min_x {
    //             if self.data[i].x <= second_min_x {
    //                 second_min_x = self.data[i].x;
    //             }
    //         }
    //     }
    //     (min_x, second_min_x)
    // }

    // fn max_xs(&self) -> (i64, i64) {
    //     let mut max_x = i64::MIN;
    //     let mut second_max_x = i64::MIN;
    //     for i in 0..self.data.len() {
    //         if self.data[i].x >= max_x {
    //             max_x = self.data[i].x;
    //         }
    //     }
    //     for i in 0..self.data.len() {
    //         if self.data[i].x != max_x {
    //             if self.data[i].x >= second_max_x {
    //                 second_max_x = self.data[i].x;
    //             }
    //         }
    //     }
    //     (max_x, second_max_x)
    // }

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

    // fn max_ys(&self) -> (i64, i64) {
    //     let mut max_y = i64::MIN;
    //     let mut second_max_y = i64::MIN;
    //     for i in 0..self.data.len() {
    //         if self.data[i].y >= max_y {
    //             max_y = self.data[i].y;
    //         }
    //     }
    //     for i in 0..self.data.len() {
    //         if self.data[i].y != max_y {
    //             if self.data[i].y >= second_max_y {
    //                 second_max_y = self.data[i].y;
    //             }
    //         }
    //     }
    //     (max_y, second_max_y)
    // }

    fn scale(&mut self, factor: i64) {
        for i in 0..self.data.len() {
            self.data[i].x *= factor;
            self.data[i].y *= factor;
        }
    }

    fn get_all_known_xs(&self) -> Vec<i64> {
        let mut output = vec![];

        for i in 0..self.data.len() {
            if !output.contains(&self.data[i].x) {
                output.push(self.data[i].x);
            }
        }

        output.sort();

        output
    }

    fn get_all_known_ys(&self) -> Vec<i64> {
        let mut output = vec![];

        for i in 0..self.data.len() {
            if !output.contains(&self.data[i].y) {
                output.push(self.data[i].y);
            }
        }

        output.sort();

        output
    }

    fn is_point_on_perimeter(&self, point: &Point) -> bool {
        let i = 0;
        let last_i = self.data.len() - 1;
        let first_segment = (&self.data[last_i], &self.data[i]);
        if Self::is_point_on_segment(first_segment, point) {
            return true;
        }
        for i in 1..self.data.len() {
            let segment = (&self.data[i - 1], &self.data[i]);
            if Self::is_point_on_segment(segment, point) {
                return true;
            }
        }

        false
    }

    fn is_point_on_segment(segment: (&Point, &Point), point: &Point) -> bool {
        let (top_left_seg_point, bot_right_seg_point) =
            if segment.0.x < segment.1.x || segment.0.y < segment.1.y {
                (segment.0, segment.1)
            } else {
                (segment.1, segment.0)
            };
        if top_left_seg_point.x != bot_right_seg_point.x {
            point.y == top_left_seg_point.y
                && point.x > top_left_seg_point.x
                && point.x < bot_right_seg_point.x
        } else {
            point.x == top_left_seg_point.x
                && point.y > top_left_seg_point.y
                && point.y < bot_right_seg_point.y
        }
    }

    fn find_area(&mut self) -> i64 {
        // necessary to place all points on even numbers for this algorithm
        self.scale(2);

        let mut area = 0;

        let known_xs = self.get_all_known_xs();
        let known_ys = self.get_all_known_ys();

        for i in 0..known_ys.len() {
            let mut crossed_segments = 0;
            let test_y = known_ys[i] + 1;
            for j in 0..known_xs.len() {
                let test_x = known_xs[j];
                let test_point = Point::new(test_x, test_y);
                if self.is_point_on_perimeter(&test_point) {
                    crossed_segments += 1;
                }

                if crossed_segments % 2 == 1 {
                    // should never overflow in a closed polygon
                    let grid_area_width = known_ys[i + 1] - known_ys[i];
                    let grid_area_height = known_xs[j + 1] - known_xs[j];
                    area += grid_area_width * grid_area_height;
                }
            }
        }

        // adjust for scaling
        area / 4
    }
}

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
    }

    for plan in plans.iter() {
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

    let answer = poly.find_area();

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
    let answer = poly.find_area();
    final_answer(answer, submit, DAY, 2).await;
}
