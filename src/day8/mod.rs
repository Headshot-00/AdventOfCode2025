use crate::adv_errors::UpdateError;
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

    let pts = &points;

    let mut edges: Vec<Edge> = (0..n)
        .into_par_iter()
        .flat_map_iter(|i| {
            let pi = pts[i];
            (i + 1..n).map(move |j| Edge {
                dist2: pi.dist2(&pts[j]),
                a: i,
                b: j,
            })
        })
        .collect();

    edges.par_sort_unstable_by_key(|e| e.dist2);

    // Compute product of the three largest clusters after cluster_mult_num connections
    let k = edges.len().min(cluster_mult_num);
    let mut uf = UnionFind::new(n);
    for e in &edges[..k] {
        uf.union(e.a, e.b);
    }

    // compute sizes of connected components after 1000 edges
    let mut counts: HashMap<usize, usize> = HashMap::new();
    for i in 0..n {
        let root = uf.find(i);
        *counts.entry(root).or_insert(0) += 1;
    }

    let mut sizes: Vec<usize> = counts.values().copied().collect();
    sizes.sort_unstable_by(|a, b| b.cmp(a));
    if sizes.len() < 3 {
        return Err(UpdateError::InvalidInput(
            "Fewer than three clusters exist after connecting 1000 pairs!".into(),
        ));
    }
    let cluster_product = sizes[0] as i64 * sizes[1] as i64 * sizes[2] as i64;

    // Do full minimum spanning tree then get the final edge
    let mut last_edge: Option<(usize, usize)> = None;
    for e in &edges {
        let ra = uf.find(e.a);
        let rb = uf.find(e.b);
        if ra != rb {
            uf.union(ra, rb);
            last_edge = Some((e.a, e.b));
        }
    }

    let (i, j) = match last_edge {
        Some(edge) => edge, // bind (i, j) here
        None => return Err(UpdateError::InvalidInput("Could not find any edge!".into())), // return early if no edge
    };

    let p1 = points[i];
    let p2 = points[j];

    let x_product = p1.x * p2.x;

    Ok((cluster_product, x_product))
}
