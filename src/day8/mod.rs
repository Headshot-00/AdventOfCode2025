use crate::adv_errors::UpdateError;
use itertools::iproduct;
use rayon::prelude::*;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::BufRead;

/// Represents a 3D point with integer coordinates
#[derive(Copy, Clone, Debug)]
pub struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    #[inline]
    fn dist2(&self, other: &Point) -> i64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        dx * dx + dy * dy + dz * dz
    }
}

#[derive(Debug)]
pub struct Edge {
    dist2: i64,
    a: usize,
    b: usize,
}

impl Eq for Edge {}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.dist2 == other.dist2
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.dist2.cmp(&other.dist2)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Reads points from a buffered input, expects lines in "x,y,z" format.
/// Skips empty lines and returns an error for invalid input.
pub fn read_points<R: BufRead>(reader: R) -> Result<Vec<Point>, UpdateError> {
    let points: Vec<Point> = reader
        .lines()
        .map(|line| {
            let line = line.map_err(|e| UpdateError::Io(e))?;
            let line = line.trim();
            if line.is_empty() {
                return Err(UpdateError::EmptyInput); // we’ll filter empty later
            }
            let parts: Vec<&str> = line.split(',').collect();

            if parts.len() != 3 {
                return Err(UpdateError::InvalidInput(format!(
                    "\"{}\" does not contain exactly three elements separated by comma!",
                    line
                )));
            }

            let x = parts[0].trim().parse::<i64>().map_err(|_| {
                UpdateError::InvalidInput(format!(
                    "\"{}\" Could not be parsed as an integer!",
                    parts[0]
                ))
            })?;
            let y = parts[1].trim().parse::<i64>().map_err(|_| {
                UpdateError::InvalidInput(format!(
                    "\"{}\" Could not be parsed as an integer!",
                    parts[1]
                ))
            })?;
            let z = parts[2].trim().parse::<i64>().map_err(|_| {
                UpdateError::InvalidInput(format!(
                    "\"{}\" Could not be parsed as an integer!",
                    parts[2]
                ))
            })?;
            Ok(Point { x, y, z })
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

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        let mut v = x;

        // Climb up to find the root
        while self.parent[v] != v {
            v = self.parent[v];
        }
        let root = v;

        // Path compression on the way back
        let mut v = x;
        while self.parent[v] != v {
            let next = self.parent[v];
            self.parent[v] = root;
            v = next;
        }

        root
    }

    fn union(&mut self, a: usize, b: usize) {
        let mut ra = self.find(a);
        let mut rb = self.find(b);

        if ra == rb {
            return;
        }

        if self.size[ra] < self.size[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }

        self.parent[rb] = ra;
        self.size[ra] += self.size[rb];
    }
}

/// Solve computes the solution to both parts 1 and 2
/// This is basically kruskal's algorithm to find the minimum spanning tree
pub fn solve<R: BufRead>(reader: R, cluster_mult_num: usize) -> Result<(i64, i64), UpdateError> {
    let points = read_points(reader)?;
    let n = points.len();
    if n < 2 {
        return Err(UpdateError::InvalidInput(
            "Input does not contain at least two points!".into(),
        ));
    }

    // Compute bounding box
    let mut min_x = i64::MAX;
    let mut max_x = i64::MIN;
    let mut min_y = i64::MAX;
    let mut max_y = i64::MIN;
    let mut min_z = i64::MAX;
    let mut max_z = i64::MIN;

    for p in &points {
        min_x = min_x.min(p.x);
        max_x = max_x.max(p.x);
        min_y = min_y.min(p.y);
        max_y = max_y.max(p.y);
        min_z = min_z.min(p.z);
        max_z = max_z.max(p.z);
    }

    let n_f64 = n as f64;
    let span_x = max_x - min_x;
    let span_y = max_y - min_y;
    let span_z = max_z - min_z;

    // Simple heuristic for cell size
    let cell_size = ((span_x.max(span_y).max(span_z)) as f64 / n_f64.cbrt()).ceil() as i64 + 1;

    // Hash points into grid
    let mut grid: HashMap<(i64, i64, i64), Vec<usize>> = HashMap::new();
    for (i, p) in points.iter().enumerate() {
        let cx = (p.x - min_x) / cell_size;
        let cy = (p.y - min_y) / cell_size;
        let cz = (p.z - min_z) / cell_size;
        grid.entry((cx, cy, cz)).or_default().push(i);
    }

    let pts = &points;

    // Generate edges using neighbor cells only
    let mut edges: Vec<Edge> = (0..n)
        .into_par_iter()
        .flat_map_iter(|i| {
            let p = pts[i];
            let cx = (p.x - min_x) / cell_size;
            let cy = (p.y - min_y) / cell_size;
            let cz = (p.z - min_z) / cell_size;

            let grid_ref = &grid;

            iproduct!(-1..=1, -1..=1, -1..=1)
                .flat_map(move |(dx, dy, dz)| {
                    grid_ref
                        .get(&(cx + dx, cy + dy, cz + dz))
                        .into_iter()
                        .flat_map(|indices| indices.iter())
                })
                .filter(move |&&j| i < j)
                .map(move |&j| Edge {
                    dist2: p.dist2(&pts[j]),
                    a: i,
                    b: j,
                })
        })
        .collect();

    // Sort edges for Kruskal
    edges.par_sort_unstable_by_key(|e| e.dist2);

    // First phase: cluster_mult_num edges
    let k = edges.len().min(cluster_mult_num);
    let mut uf = UnionFind::new(n);
    for e in &edges[..k] {
        uf.union(e.a, e.b);
    }

    // Compute cluster sizes
    let mut counts = vec![0usize; n];
    for i in 0..n {
        counts[uf.find(i)] += 1;
    }

    let mut sizes: Vec<usize> = counts.into_iter().filter(|&c| c > 0).collect();
    sizes.par_sort_unstable_by(|a, b| b.cmp(a));
    if sizes.len() < 3 {
        return Err(UpdateError::InvalidInput(
            "Fewer than three clusters exist after connecting edges!".into(),
        ));
    }
    let cluster_product = sizes[0] as i64 * sizes[1] as i64 * sizes[2] as i64;

    // Full MST to get last edge
    let mut last_edge: Option<(usize, usize)> = None;
    for e in &edges {
        let ra = uf.find(e.a);
        let rb = uf.find(e.b);
        if ra != rb {
            uf.union(ra, rb);
            last_edge = Some((e.a, e.b));
        }
    }

    let (i, j) = last_edge.ok_or_else(|| UpdateError::InvalidInput("No MST edge found!".into()))?;
    let p1 = points[i];
    let p2 = points[j];
    let x_product = p1.x * p2.x;

    Ok((cluster_product, x_product))
}
