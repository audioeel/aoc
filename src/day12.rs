use std::fs;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashSet;

struct Grid {
    values: Vec<i32>,
    m: usize,
    n: usize,
}

type NeighborFn = fn(&Grid, usize) -> Vec<usize>;

fn downhill_neighbors(grid: &Grid, p: usize) -> Vec<usize> {
    let m = grid.m;
    let n = grid.n;
    let values = &grid.values;

    let i = p / n;
    let j = p - i*n;

    let mut ns: Vec<usize> = vec![];
    if i >= 1 && values[(i-1)*n+j] - values[i*n+j] <= 1 {
        ns.push((i-1)*n+j);
    }
    if i + 1 < m && values[(i+1)*n+j] - values[i*n+j] <= 1 {
        ns.push((i+1)*n+j);
    }
    if j >= 1 && values[i*n+j-1] - values[i*n+j] <= 1 {
        ns.push(i*n+j-1);
    }
    if j + 1 < n && values[i*n+j+1] - values[i*n+j] <= 1 {
        ns.push(i*n+j+1);
    }

    ns
}

fn uphill_neighbors(grid: &Grid, p: usize) -> Vec<usize> {
    let m = grid.m;
    let n = grid.n;
    let values = &grid.values;

    let i = p / n;
    let j = p - i*n;

    let mut ns: Vec<usize> = vec![];
    if i >= 1 && values[(i-1)*n+j] - values[i*n+j] >= -1 {
        ns.push((i-1)*n+j);
    }
    if i + 1 < m && values[(i+1)*n+j] - values[i*n+j] >= -1 {
        ns.push((i+1)*n+j);
    }
    if j >= 1 && values[i*n+j-1] - values[i*n+j] >= -1 {
        ns.push(i*n+j-1);
    }
    if j + 1 < n && values[i*n+j+1] - values[i*n+j] >= -1 {
        ns.push(i*n+j+1);
    }

    ns
}

fn parse(input: &String) -> (usize, usize, Grid) {
    let chars: Vec<Vec<char>> = input.lines()
        .map(|line| line.chars().collect())
        .collect();

    let m = chars.len();
    let n = chars[0].len();
    let mut values: Vec<i32> = vec![0; m*n];
    let mut start: usize = 0;
    let mut end: usize = 0;

    for i in 0..m {
        for j in 0..n {
            values[i*n+j] = match chars[i][j] {
                'S' => {
                    start = i*n+j;
                    0
                },
                'E' => {
                    end = i*n+j;
                    'z' as i32 - 'a' as i32
                },
                lc  => lc as i32 - 'a' as i32,
            };
        }
    }

    (start, end, Grid { values: values, m: m, n: n })
}


#[derive(PartialEq, Eq)]
struct State {
    cost: i32,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// slightly modified Dijkstra
fn shortest_path(grid: &Grid, start: usize, frontier: HashSet<usize>, neighbors: NeighborFn) -> Vec<i32> {
    let m = grid.m;
    let n = grid.n;

    let mut dist: Vec<i32> = vec![i32::MAX; m*n];

    let mut heap = BinaryHeap::new();

    dist[start] = 0;
    heap.push(State { cost: 0, position: start });

    let mut paths = vec![];
    while let Some(State { cost, position }) = heap.pop() {
        if frontier.contains(&position) {
            paths.push(cost);
        }

        if frontier.len() == paths.len() {
            return paths;
        }

        if cost > dist[position] {
            continue;
        }

        for neighbor in neighbors(&grid, position) {
            let next = State { cost: cost + 1, position: neighbor };
            if next.cost < dist[next.position] {
                dist[next.position] = next.cost;
                heap.push(next);
            }
        }
    }

    paths
}

pub fn solve() {
    let input = fs::read_to_string("resources/day12.txt").unwrap();
    let (start, end, grid) = parse(&input);
    let mut frontier = HashSet::new();
    frontier.insert(end);
    let shortest_path_from_start = shortest_path(&grid, start, frontier, downhill_neighbors);
    println!("{}", shortest_path_from_start[0]);

    let starts: HashSet<usize> = grid.values.iter().enumerate()
        .filter(|(_, &c)| c == 0)
        .map(|(i, _)| i)
        .collect();
                
    let paths = shortest_path(&grid, end, starts, uphill_neighbors);
    let best_trail = paths.iter().min();
    println!("{:?}", best_trail.unwrap());
}
