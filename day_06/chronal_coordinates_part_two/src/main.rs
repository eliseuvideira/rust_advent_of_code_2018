use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const TOTAL_DISTANCE_THRESOLD: i32 = 10_000;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().filter_map(|line| line.ok()).collect();

    let coordinates = parse_lines(&lines);
    let (max_x, min_x, max_y, min_y) = get_bounds(&coordinates).ok_or("No bounds")?;
    let grid = create_grid((max_x, min_x, max_y, min_y));
    let mut result_grid: Vec<Grid> = vec![];
    let mut infinites: HashSet<char> = HashSet::new();

    for item in grid.iter() {
        let mut min_distance = i32::MAX;
        let mut min_distance_coordinates: Vec<Coordinate> = vec![];

        let mut total_distance_to_all_coordinates = 0;

        for &coordinate in coordinates.iter() {
            let distance = calc_distance(&item.point, &coordinate.point);

            total_distance_to_all_coordinates += distance;

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
                closest_coordinates: Some(min_distance_coordinates),
                total_distance_to_all_coordinates: Some(total_distance_to_all_coordinates),
                ..item.clone()
            });
        } else if min_distance_coordinates.len() == 1 {
            let coordinate = min_distance_coordinates.get(0).ok_or("No coordinate")?;

            result_grid.push(Grid {
                closest_coordinates: Some(vec![coordinate.clone()]),
                total_distance_to_all_coordinates: Some(total_distance_to_all_coordinates),
                ..item.clone()
            });
            if is_infinite(&item.point, (max_x, min_x, max_y, min_y)) {
                infinites.insert(coordinate.id);
            }
        } else {
            result_grid.push(item.clone());
        }
    }

    let mut total_area = 0;
    for item in result_grid.iter() {
        // if let Some(coordinates) = &item.closest_coordinates {
        //     if coordinates
        //         .iter()
        //         .any(|coordinate| infinites.contains(&coordinate.id))
        //     {
        //         continue;
        //     }
        // }

        if let Some(total_distance) = item.total_distance_to_all_coordinates {
            if total_distance < TOTAL_DISTANCE_THRESOLD {
                total_area += 1;
            }
        }
    }

    println!("{}", total_area);

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

#[derive(Debug, Clone)]
struct Grid {
    closest_coordinates: Option<Vec<Coordinate>>,
    point: Point,
    total_distance_to_all_coordinates: Option<i32>,
}

fn create_grid((max_x, min_x, max_y, min_y): (i32, i32, i32, i32)) -> Vec<Grid> {
    let width = (max_x - min_x + 1) as usize;
    let height = (max_y - min_y + 1) as usize;
    let mut grid: Vec<Grid> = Vec::with_capacity(width * height);

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            grid.push(Grid {
                closest_coordinates: None,
                point: Point { x, y },
                total_distance_to_all_coordinates: None,
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
