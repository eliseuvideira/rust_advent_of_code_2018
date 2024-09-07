use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().filter_map(|line| line.ok()).collect();

    let coordinates = parse_lines(&lines);
    let (max_x, min_x, max_y, min_y) = get_bounds(&coordinates).ok_or("No bounds")?;
    let grid = create_grid((max_x, min_x, max_y, min_y));
    let mut result_grid: Vec<Grid> = vec![];
    let mut infinites: HashSet<char> = HashSet::new();

    for &item in grid.iter() {
        let mut min_distance = i32::MAX;
        let mut min_distance_coordinates: Vec<Coordinate> = vec![];

        for &coordinate in coordinates.iter() {
            let distance = calc_distance(&item.point, &coordinate.point);

            if distance < min_distance {
                min_distance = distance;
                min_distance_coordinates.clear();
                min_distance_coordinates.push(coordinate);
            } else if distance == min_distance {
                min_distance_coordinates.push(coordinate);
            }
        }

        if min_distance_coordinates.len() > 1 {
            result_grid.push(Grid {
                is_coordinate_root: false,
                closest_coordinate_id: '-',
                ..item
            });
        } else if min_distance_coordinates.len() == 1 {
            let coordinate = min_distance_coordinates.get(0).ok_or("No coordinate")?;

            result_grid.push(Grid {
                is_coordinate_root: min_distance == 0,
                closest_coordinate_id: coordinate.id,
                ..item
            });
            if is_infinite(&item.point, (max_x, min_x, max_y, min_y)) {
                infinites.insert(coordinate.id);
            }
        } else {
            result_grid.push(item);
        }
    }

    let mut counter: HashMap<char, i32> = HashMap::new();
    for &item in result_grid.iter() {
        if infinites.contains(&item.closest_coordinate_id) {
            continue;
        }

        match counter.get(&item.closest_coordinate_id) {
            Some(&count) => counter.insert(item.closest_coordinate_id, count + 1),
            None => counter.insert(item.closest_coordinate_id, 1),
        };
    }

    let mut max_count = 0;
    for (_, &count) in counter.iter() {
        if count > max_count {
            max_count = count;
        }
    }

    println!("{}", max_count);

    // result_grid.sort_by(|a, b| {
    //     format!("{:03}x{:03}", a.point.y, a.point.x)
    //         .cmp(&format!("{:03}x{:03}", b.point.y, b.point.x))
    // });
    // let mut current_y = 0;
    // for item in result_grid.iter() {
    //     if current_y != item.point.y {
    //         println!();
    //         current_y = item.point.y;
    //     }
    //     if item.is_coordinate_root {
    //         print!(":{}:", item.closest_coordinate_id);
    //     } else {
    //         print!(" {} ", item.closest_coordinate_id);
    //     }
    // }
    // println!();

    Ok(())
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy)]
struct Coordinate {
    id: char,
    point: Point,
}

fn parse_lines(lines: &Vec<String>) -> Vec<Coordinate> {
    lines
        .iter()
        .enumerate()
        .filter_map(|(index, line)| parse_line((index, line)))
        .collect()
}

fn parse_line((index, line): (usize, &str)) -> Option<Coordinate> {
    let comma_index = line.find(',')?;

    let x = line[..comma_index].trim().parse::<i32>().ok()?;
    let y = line[comma_index + 1..].trim().parse::<i32>().ok()?;

    Some(Coordinate {
        id: (b'.' + index as u8) as char,
        point: Point { x, y },
    })
}

fn get_bounds(coordinates: &Vec<Coordinate>) -> Option<(i32, i32, i32, i32)> {
    if coordinates.len() == 0 {
        return None;
    }

    let mut max_x = i32::MIN;
    let mut min_x = i32::MAX;
    let mut max_y = i32::MIN;
    let mut min_y = i32::MAX;

    for coordinate in coordinates.iter() {
        if max_x < coordinate.point.x {
            max_x = coordinate.point.x;
        }
        if min_x > coordinate.point.x {
            min_x = coordinate.point.x;
        }
        if max_y < coordinate.point.y {
            max_y = coordinate.point.y;
        }
        if min_y > coordinate.point.y {
            min_y = coordinate.point.y;
        }
    }

    Some((max_x, min_x, max_y, min_y))
}

#[derive(Debug, Clone, Copy)]
struct Grid {
    is_coordinate_root: bool,
    closest_coordinate_id: char,
    point: Point,
}

fn create_grid((max_x, min_x, max_y, min_y): (i32, i32, i32, i32)) -> Vec<Grid> {
    let width = (max_x - min_x + 1) as usize;
    let height = (max_y - min_y + 1) as usize;
    let mut grid: Vec<Grid> = Vec::with_capacity(width * height);

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            grid.push(Grid {
                is_coordinate_root: false,
                closest_coordinate_id: '-',
                point: Point { x, y },
            })
        }
    }

    grid
}

fn calc_distance(point1: &Point, point2: &Point) -> i32 {
    (point1.x - point2.x).abs() + (point1.y - point2.y).abs()
}

fn is_infinite(point: &Point, (max_x, min_x, max_y, min_y): (i32, i32, i32, i32)) -> bool {
    point.x == max_x || point.x == min_x || point.y == max_y || point.y == min_y
}
