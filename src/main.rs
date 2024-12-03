use std::cmp::Ordering;

// Define an Edge struct
#[derive(Debug, Clone)]
struct Edge {
    weight: i32,
    start: usize,
    end: usize,
}

// Implement sorting for Edge
impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight.cmp(&other.weight)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}

impl Eq for Edge {}

// Union-Find structure
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        UnionFind {
            parent: (0..size).collect(),
            rank: vec![0; size],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]); // Path compression
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false; // Cycle detected
        }

        // Union by rank
        if self.rank[root_x] > self.rank[root_y] {
            self.parent[root_y] = root_x;
        } else if self.rank[root_x] < self.rank[root_y] {
            self.parent[root_x] = root_y;
        } else {
            self.parent[root_y] = root_x;
            self.rank[root_x] += 1;
        }
        true
    }
}

// Kruskal's algorithm
fn kruskal(n: usize, mut edges: Vec<Edge>) -> (Vec<Edge>, i32) {
    edges.sort(); // Sort edges by weight
    let mut uf = UnionFind::new(n);
    let mut mst = Vec::new();
    let mut total_weight = 0;

    for edge in edges {
        if uf.union(edge.start, edge.end) {
            mst.push(edge.clone());
            total_weight += edge.weight;
        }
    }

    (mst, total_weight)
}

fn main() {
    // Example usage
    let edges = vec![
        Edge { weight: 1, start: 0, end: 1 },
        Edge { weight: 4, start: 0, end: 2 },
        Edge { weight: 3, start: 1, end: 2 },
        Edge { weight: 2, start: 1, end: 3 },
        Edge { weight: 5, start: 2, end: 3 },
    ];

    let (mst, total_weight) = kruskal(4, edges);

    println!("Edges in MST: {:?}", mst);
    println!("Total weight of MST: {}", total_weight);
}
