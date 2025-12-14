use crate::adv_errors::UpdateError;
use std::collections::HashMap;
use std::io::BufRead;

/// Represents a 2D point with integer coordinates
#[derive(Copy, Clone, Debug)]
pub struct Point {
    x: i64,
    y: i64,
}

/// Reads points from a buffered input, expects lines in "x,y" format.
/// Skips empty lines and returns an error for invalid input.
pub fn read_points<R: BufRead>(reader: R) -> Result<Vec<Point>, UpdateError> {
    let points: Vec<Point> = reader
        .lines()
        .map(|line| {
            let line = line.map_err(|_| UpdateError::InvalidInput)?;
            let line = line.trim();
            if line.is_empty() {
                return Err(UpdateError::EmptyInput); // we’ll filter empty later
            }
            let (x_str, y_str) = line.split_once(',').ok_or(UpdateError::InvalidInput)?;
            let x = x_str
                .trim()
                .parse::<i64>()
                .map_err(|_| UpdateError::InvalidInput)?;
            let y = y_str
                .trim()
                .parse::<i64>()
                .map_err(|_| UpdateError::InvalidInput)?;
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

/// Brute-force approach to find largest rectangle defined by any two points.
/// Computes area = (|x1 - x2| + 1) * (|y1 - y2| + 1) for each point pairing.
pub fn find_biggest_rectangle_simple(points: &[Point]) -> i64 {
    points
        .iter()
        .enumerate()
        .flat_map(|(i, p1)| {
            points
                .iter()
                .skip(i + 1)
                .map(move |p2| ((p1.x - p2.x).abs() + 1) * ((p1.y - p2.y).abs() + 1))
        })
        .max()
        .unwrap_or(0) // returns 0 if there are no points
}

/// Coordinate compression: maps original x/y coordinates to small indices for grid usage.
/// Returns unique x/y lists and maps from original coordinate -> compressed index.
fn compress(points: &[Point]) -> (Vec<i64>, Vec<i64>, HashMap<i64, usize>, HashMap<i64, usize>) {
    let mut xs: Vec<i64> = points.iter().map(|p| p.x).collect();
    let mut ys: Vec<i64> = points.iter().map(|p| p.y).collect();

    xs.sort();
    xs.dedup();
    ys.sort();
    ys.dedup();

    // create mapping from original coordinate to compressed grid index
    let x_map = xs.iter().enumerate().map(|(i, &x)| (x, i)).collect();
    let y_map = ys.iter().enumerate().map(|(i, &y)| (y, i)).collect();

    (xs, ys, x_map, y_map)
}

/// Creates an empty grid filled with '.' for given width and height
fn make_grid(w: usize, h: usize) -> Vec<Vec<char>> {
    vec![vec!['.'; w]; h]
}

/// Rasterizes polygon edges onto the grid using axis-aligned segments
fn rasterize(grid: &mut [Vec<char>], pts: &[(usize, usize)]) {
    let n = pts.len();
    for i in 0..n {
        let (x1, y1) = pts[i];
        let (x2, y2) = pts[(i + 1) % n]; // wrap-around to close polygon

        if x1 == x2 {
            // vertical
            let (a, b) = if y1 <= y2 { (y1, y2) } else { (y2, y1) };
            for y in a..=b {
                grid[y][x1] = '#';
            }
        } else if y1 == y2 {
            // horizontal
            let (a, b) = if x1 <= x2 { (x1, x2) } else { (x2, x1) };
            for x in a..=b {
                grid[y1][x] = '#';
            }
        }
    }
}

/// Flood-fill algorithm to mark all empty cells connected to a start point
/// Marks visited empty cells with 'X'
fn flood_fill(grid: &mut [Vec<char>], start: (usize, usize)) {
    let mut stack = vec![start];
    let dirs = [(0i64, 1), (0, -1), (1, 0), (-1, 0)];

    while let Some((x, y)) = stack.pop() {
        if grid[y][x] != '.' {
            continue; // already filled or edge
        }
        grid[y][x] = 'X'; // mark as visited

        for (dx, dy) in dirs {
            let nx = x as i64 + dx;
            let ny = y as i64 + dy;
            if nx >= 0 && ny >= 0 {
                let (nx, ny) = (nx as usize, ny as usize);
                if ny < grid.len() && nx < grid[0].len() {
                    if grid[ny][nx] == '.' {
                        stack.push((nx, ny));
                    }
                }
            }
        }
    }
}

/// Finds a point inside the polygon using the ray-casting method
/// A point is inside if the horizontal ray to the left crosses an odd number of edges
fn find_inside(grid: &[Vec<char>]) -> (usize, usize) {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] != '.' {
                continue;
            }

            let mut hits = 0;
            let mut prev = '.';

            for i in (0..=x).rev() {
                let cur = grid[y][i];
                if cur != prev {
                    hits += 1;
                }
                prev = cur;
            }

            if hits % 2 == 1 {
                return (x, y); // found an inside point
            }
        }
    }
    panic!("no inside point found"); // should never happen
}

/// Checks if the rectangle defined by points a and b is fully enclosed by the polygon
fn is_enclosed(
    a: Point,
    b: Point,
    grid: &[Vec<char>],
    x_map: &HashMap<i64, usize>,
    y_map: &HashMap<i64, usize>,
) -> bool {
    let (x1, x2) = {
        let xa = x_map[&a.x];
        let xb = x_map[&b.x];
        if xa <= xb { (xa, xb) } else { (xb, xa) }
    };

    let (y1, y2) = {
        let ya = y_map[&a.y];
        let yb = y_map[&b.y];
        if ya <= yb { (ya, yb) } else { (yb, ya) }
    };

    // Check top and bottom edges of rectangle
    for x in x1..=x2 {
        if grid[y1][x] == '.' || grid[y2][x] == '.' {
            return false;
        }
    }

    // Check left and right edges of rectangle
    for y in y1..=y2 {
        if grid[y][x1] == '.' || grid[y][x2] == '.' {
            return false;
        }
    }

    true
}

/// Brute-force search for largest rectangle fully enclosed by polygon
fn biggest_rectangle(
    points: &[Point],
    grid: &[Vec<char>],
    x_map: &HashMap<i64, usize>,
    y_map: &HashMap<i64, usize>,
) -> i64 {
    let mut max = 0;

    for i in 0..points.len() {
        for j in i + 1..points.len() {
            if is_enclosed(points[i], points[j], grid, x_map, y_map) {
                let area = ((points[i].x - points[j].x).abs() + 1)
                    * ((points[i].y - points[j].y).abs() + 1);
                max = max.max(area);
            }
        }
    }
    max
}

/// Helper function to find the largest rectangle fully inside a polygon
pub fn find_biggest_rectangle_polygon(points: Vec<Point>) -> i64 {
    // Step 1: compress coordinates for efficient grid representation
    let (uniq_x, uniq_y, x_map, y_map) = compress(&points);

    // Step 2: create empty grid
    let mut grid = make_grid(uniq_x.len(), uniq_y.len());

    // Step 3: map points to compressed grid and mark vertices
    let z_points: Vec<(usize, usize)> = points
        .iter()
        .map(|p| {
            let x = x_map[&p.x];
            let y = y_map[&p.y];
            grid[y][x] = '#';
            (x, y)
        })
        .collect();

    // Step 4: rasterize polygon edges on grid
    rasterize(&mut grid, &z_points);

    // Step 5: flood-fill from an inside point to mark inside/outside regions
    let inside_point = find_inside(&grid);
    flood_fill(&mut grid, inside_point);

    // Step 6: brute-force largest rectangle fully enclosed by polygon
    biggest_rectangle(&points, &grid, &x_map, &y_map)
}
