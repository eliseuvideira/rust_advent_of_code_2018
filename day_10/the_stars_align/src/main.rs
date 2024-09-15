use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").expect("Failed to open input.txt");
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .collect();

    let mut points = parse_lines(&lines);
    let mut prev_area = u64::MAX;

    loop {
        let new_points = run_seconds(&points);

        let area = calc_area(&new_points);

        if area > prev_area {
            break;
        }

        points = new_points;
        prev_area = area;
    }

    draw_points(&points);
}

#[derive(Debug, Clone, Copy)]
struct Point {
    position: (i32, i32),
    velocity: (i32, i32),
}

fn parse_lines(lines: &Vec<String>) -> Vec<Point> {
    lines.iter().map(|line| parse_line(line)).collect()
}

fn parse_line(line: &str) -> Point {
    let pos_start = line.find('<').unwrap();
    let pos_end = line.find('>').unwrap();
    let pos_comma = line.find(',').unwrap();

    let pos_x_str = line[(pos_start + 1)..pos_comma].trim();
    let pos_y_str = line[(pos_comma + 1)..pos_end].trim();

    let position: (i32, i32) = (pos_x_str.parse().unwrap(), pos_y_str.parse().unwrap());

    let line_vel = &line[(pos_end + 1)..];
    let vel_start = line_vel.find('<').unwrap();
    let vel_end = line_vel.find('>').unwrap();
    let vel_comma = line_vel.find(',').unwrap();

    let vel_x_str = line_vel[(vel_start + 1)..vel_comma].trim();
    let vel_y_str = line_vel[(vel_comma + 1)..vel_end].trim();

    let velocity: (i32, i32) = (vel_x_str.parse().unwrap(), vel_y_str.parse().unwrap());

    Point { position, velocity }
}

fn run_second(point: &Point) -> Point {
    Point {
        position: (
            point.position.0 + point.velocity.0,
            point.position.1 + point.velocity.1,
        ),
        ..point.clone()
    }
}

fn run_seconds(points: &Vec<Point>) -> Vec<Point> {
    points.iter().map(run_second).collect()
}

fn calc_area(points: &Vec<Point>) -> u64 {
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;
    for point in points.iter() {
        if point.position.0 < min_x {
            min_x = point.position.0;
        }
        if point.position.0 > max_x {
            max_x = point.position.0;
        }
        if point.position.1 < min_y {
            min_y = point.position.1;
        }
        if point.position.1 > max_y {
            max_y = point.position.1;
        }
    }

    let width = (max_x - min_x) as u64;
    let height = (max_y - min_y) as u64;

    width * height
}

fn draw_points(points: &Vec<Point>) -> () {
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;
    let mut points_set: HashSet<(i32, i32)> = HashSet::with_capacity(points.len());
    for point in points.iter() {
        points_set.insert(point.position.clone());
        if point.position.0 < min_x {
            min_x = point.position.0;
        }
        if point.position.0 > max_x {
            max_x = point.position.0;
        }
        if point.position.1 < min_y {
            min_y = point.position.1;
        }
        if point.position.1 > max_y {
            max_y = point.position.1;
        }
    }

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if points_set.contains(&(x, y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}
