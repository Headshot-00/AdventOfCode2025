use crate::adv_errors::UpdateError;
use rayon::prelude::*;
use std::collections::HashMap;
use std::io::BufRead;

/// Represents a 2D point with integer coordinates
#[derive(Copy, Clone, Debug)]
pub struct Point {
    x: u32,
    y: u32,
}

/// Represents an axis-aligned edge
#[derive(Copy, Clone, Debug)]
pub struct AAEdge {
    start: u32,
    end: u32,
    height: u32,
}

impl AAEdge {
    fn new(u1: u32, u2: u32, height: u32) -> Self {
        AAEdge {
            start: u1.min(u2),
            end: u1.max(u2),
            height,
        }
    }
}

/// Represents an axis-aligned rectangle, defined by two opposing points
#[derive(PartialEq, Eq)]
pub struct AARect {
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
}

impl AARect {
    fn new(p1: &Point, p2: &Point) -> Self {
        AARect {
            x1: p1.x.min(p2.x),
            x2: p1.x.max(p2.x),
            y1: p1.y.min(p2.y),
            y2: p1.y.max(p2.y),
        }
    }

    fn grid_area(&self) -> u64 {
        let xd = self.x2 - self.x1 + 1;
        let yd = self.y2 - self.y1 + 1;
        let area: u64 = xd as u64 * yd as u64;
        area
    }
}

/// Reads points from a buffered input, expects lines in "x,y" format.
/// Skips empty lines and returns an error for invalid input.
fn read_points<R: BufRead>(reader: R) -> Result<Vec<Point>, UpdateError> {
    let points: Vec<Point> = reader
        .lines()
        .map(|line| {
            let line = line.map_err(|e| UpdateError::Io(e))?;
            let line = line.trim();
            if line.is_empty() {
                return Err(UpdateError::EmptyInput); // we’ll filter empty later
            }
            let (x_str, y_str) = line
                .split_once(',')
                .ok_or(UpdateError::InvalidInput(format!(
                    "Line could not be split on comma! {}",
                    line
                )))?;
            let x = x_str.trim().parse::<u32>().map_err(|_| {
                UpdateError::InvalidInput(format!(
                    "\"{}\" could not be parsed as an integer!",
                    x_str
                ))
            })?;
            let y = y_str.trim().parse::<u32>().map_err(|_| {
                UpdateError::InvalidInput(format!(
                    "\"{}\" could not be parsed as an integer!",
                    y_str
                ))
            })?;
            Ok(Point { x, y })
        })
        .filter_map(|res| match res {
            Ok(p) => Some(Ok(p)),                 // valid point, keep it
            Err(UpdateError::EmptyInput) => None, // skip blank lines
            Err(e) => Some(Err(e)),               // propagate other errors
        })
        .collect::<Result<_, _>>()?;

    if points.is_empty() {
        return Err(UpdateError::EmptyInput); // error if no valid points
    }

    Ok(points)
}

/// Coordinate compression: maps original x/y coordinates to small indices for grid usage.
/// Returns unique x/y lists and maps from original coordinate -> compressed index.
fn compress(points: &[Point]) -> (Vec<u32>, Vec<u32>, HashMap<u32, usize>, HashMap<u32, usize>) {
    let mut xs: Vec<u32> = points.iter().map(|p| p.x).collect();
    let mut ys: Vec<u32> = points.iter().map(|p| p.y).collect();

    xs.par_sort_unstable();
    xs.dedup();
    ys.par_sort_unstable();
    ys.dedup();

    // create mapping from original coordinate to compressed grid index
    let x_map = xs.iter().enumerate().map(|(i, &x)| (x, i)).collect();
    let y_map = ys.iter().enumerate().map(|(i, &y)| (y, i)).collect();

    (xs, ys, x_map, y_map)
}

fn get_rects(points: &[Point]) -> Vec<AARect> {
    points
        .iter()
        .enumerate()
        .flat_map(|(i, p1)| points.iter().skip(i + 1).map(move |p2| AARect::new(p1, p2)))
        .collect()
}

/// Extracts axis-aligned edges from a polygon and separates them into horizontal and vertical edges
fn extract_edges(
    points: &[Point],
    x_map: &HashMap<u32, usize>,
    y_map: &HashMap<u32, usize>,
) -> Result<(Vec<AAEdge>, Vec<AAEdge>), UpdateError> {
    let mut horizontal = Vec::new();
    let mut vertical = Vec::new();

    if points.is_empty() {
        return Err(UpdateError::EmptyInput);
    }

    let n = points.len();
    for i in 0..n {
        let p1 = points[i];
        let p2 = points[(i + 1) % n]; // wrap around for last edge
        if p1.x == p2.x {
            // vertical edge
            let x = *x_map
                .get(&p1.x)
                .ok_or_else(|| UpdateError::InvalidInput(format!("x coord {} not found", p1.x)))?
                as u32;
            let start = *y_map
                .get(&p1.y)
                .ok_or_else(|| UpdateError::InvalidInput(format!("y coord {} not found", p1.y)))?
                as u32;
            let end = *y_map
                .get(&p2.y)
                .ok_or_else(|| UpdateError::InvalidInput(format!("y coord {} not found", p2.y)))?
                as u32;
            vertical.push(AAEdge::new(start, end, x));
        } else if p1.y == p2.y {
            // horizontal edge
            let y = *y_map
                .get(&p1.y)
                .ok_or_else(|| UpdateError::InvalidInput(format!("y coord {} not found", p1.y)))?
                as u32;
            let start = *x_map
                .get(&p1.x)
                .ok_or_else(|| UpdateError::InvalidInput(format!("x coord {} not found", p1.x)))?
                as u32;
            let end = *x_map
                .get(&p2.x)
                .ok_or_else(|| UpdateError::InvalidInput(format!("x coord {} not found", p2.x)))?
                as u32;
            horizontal.push(AAEdge::new(start, end, y));
        } else {
            // ignore non-axis-aligned edges
            return Err(UpdateError::InvalidInput(format!(
                "Non-axis-aligned edge detected: {:?} -> {:?}",
                p1, p2
            )));
        }
    }

    Ok((horizontal, vertical))
}

/// Find a rectangle inside the polygon by checking if any edge crosses into the rectangle
/// Technically this is not correct because a rectangle could be considered to be "inside"
/// a convex polygon based on this criteria even if it is actually in a "pocket".
fn is_rect_inside(
    rect: &AARect,
    h_edges: &[AAEdge],
    v_edges: &[AAEdge],
    x_map: &HashMap<u32, usize>,
    y_map: &HashMap<u32, usize>,
) -> bool {
    let x1 = *x_map.get(&rect.x1).unwrap_or(&0) as u32;
    let x2 = *x_map.get(&rect.x2).unwrap_or(&0) as u32;
    let y1 = *y_map.get(&rect.y1).unwrap_or(&0) as u32;
    let y2 = *y_map.get(&rect.y2).unwrap_or(&0) as u32;
    for edge in h_edges {
        // Horizontal edges
        if edge.height > y1 && edge.height < y2 {
            if edge.end > x1 && edge.start < x2 {
                // Edge crosses rectangle horizontally
                return false;
            }
        }
    }
    for edge in v_edges {
        // Vertical edges
        if edge.height > x1 && edge.height < x2 {
            if edge.end > y1 && edge.start < y2 {
                // Edge crosses rectangle vertically
                return false;
            }
        }
    }
    true
}

/// Helper function to find the largest rectangle fully inside a polygon
pub fn solve<R: BufRead>(reader: R) -> Result<(u64, u64), UpdateError> {
    let points = read_points(reader)?;
    let (_uniq_x, _uniq_y, x_map, y_map) = compress(&points);

    let (horizontal_edges, vertical_edges) = extract_edges(&points, &x_map, &y_map)?;

    let mut rects = get_rects(&points);
    // Sort rectangles by
    rects.par_sort_unstable_by(|a, b| b.grid_area().cmp(&a.grid_area()));
    rects.dedup();

    let largest_inside = rects
        .iter()
        .find(|r| is_rect_inside(r, &horizontal_edges, &vertical_edges, &x_map, &y_map))
        .map(|r| r.grid_area())
        .ok_or_else(|| {
            UpdateError::InvalidInput("No rectangle found fully inside polygon".into())
        })?;

    // first is the largest rectangle of all, second is the largest rectangle inside polygon
    Ok((rects[0].grid_area(), largest_inside))
}
