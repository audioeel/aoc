use std::fs;
use std::collections::HashSet;

fn max_scenic_score(grid: &Vec<Vec<u8>>) -> usize {
    let m = grid.len();
    let n = grid[0].len();

    let west_boundary = 0;
    let east_boundary = n-1;
    let north_boundary = 0;
    let south_boundary = m-1;

    // this 2D vector keeps track of the scores - to save a bit on space, we represent it as a 1D
    // vector
    // we don't need scores for the boundaries of the forest.
    // score indexes are offset by 1
    let mut scores: Vec<usize> = vec![1; (m-2) * (n-2)]; 

    // these 2D vectors keep track of the index v[i][j] of the closest tree larger than the tree
    // at (i,j) for each traversal direction: we'll refer to them as traversal caches.
    // To save a bit on space, we use 1D contiguous vecs.
    let mut west_to_east_largest: Vec<usize> = vec![0; m*n];
    let mut east_to_west_largest: Vec<usize> = vec![0; m*n];
    let mut north_to_south_largest: Vec<usize> = vec![0; m*n];
    let mut south_to_north_largest: Vec<usize> = vec![0; m*n];

    // set east-most boundary indices
    for i in 0..m {
        east_to_west_largest[i*n + east_boundary] = east_boundary;
    }

    // set south-most boundary indices
    for j in 0..n {
        south_to_north_largest[south_boundary*n + j] = south_boundary;
    }

    // Scan north to south, west to east
    // For each tree T, in each traversal direction, check its direct neighbor: if T
    // is larger than its neighbor, then check if it's also larger than the closest
    // tree larger than the neighbor (should be in the respective traversal cache).
    // Keep doing this until the index of the tree larger than T is found if there are any.
    // Compute the traversal direction score by subtracting the index of T from that of the closest 
    // tree larger than it.
    // Keep track of the index of that tree in the respective traversal cache at T's index.
    for i in north_boundary+1..south_boundary-1 {
        for j in west_boundary+1..east_boundary-1 {
            // west to east traversal - start at direct neighbor
            let mut idx = j-1;
            while grid[i][j] > grid[i][idx] && idx != west_boundary {
                idx = west_to_east_largest[i*n + idx];
            }
            west_to_east_largest[i*n + j] = idx;
            scores[(i-1)*(n-2) + j-1] *= j - idx;

            // east to west traversal - start at direct neighbor
            let mut idx = east_boundary - (j-1);
            while grid[i][east_boundary-j] > grid[i][idx] && idx != east_boundary {
                idx = east_to_west_largest[i*n + idx];
            }
            east_to_west_largest[i*n + east_boundary-j] = idx;
            scores[(i-1)*(n-2) + east_boundary-j-1] *= idx - (east_boundary - j);

            // north to south traversal - start at direct neighbor
            let mut idx = i-1;
            while grid[i][j] > grid[idx][j] && idx != north_boundary {
                idx = north_to_south_largest[idx*n + j];
            }
            north_to_south_largest[i*n + j] = idx;
            scores[(i-1)*(n-2) + j-1] *= i - idx;

            // south to north traversal - start at direct neighbor
            let mut idx = south_boundary - (i-1);
            while grid[south_boundary-i][j] > grid[idx][j] && idx != south_boundary {
                idx = south_to_north_largest[idx*n + j];
            }
            south_to_north_largest[(south_boundary-i)*n + j] = idx;
            scores[(south_boundary-i-1)*(n-2) + j-1] *= idx - (south_boundary - i);
        }
    }

    // find highest scenic score
    *scores.iter().max().unwrap()
}

fn visible_trees(grid: &Vec<Vec<u8>>) -> HashSet<usize> {
    let m = grid.len();
    let n = grid[0].len();

    let mut visible = HashSet::new();
    for i in 0..m {
        // start of row
        visible.insert(i*n); 
        // end of row
        visible.insert((i+1)*n - 1);
    }
    for j in 1..n-1 {
        // start of column
        visible.insert(j);
        // end of column
        visible.insert((m-1)*n + j);
    }

    for i in 1..m-1 {
        // from the west looking east
        let mut west_to_east_largest = grid[i][0];
        // from the east looking west
        let mut east_to_west_largest = grid[i][n-1];
        for j in 1..n-1 {
            if grid[i][j] > west_to_east_largest {
                west_to_east_largest = grid[i][j];
                visible.insert(i*n + j);
            }
            if grid[i][n-1-j] > east_to_west_largest {
                east_to_west_largest = grid[i][n-1-j];
                visible.insert(i*n + n-1-j);
            }
        }
    }

    for j in 1..n-1 {
        // from the north looking south
        let mut north_to_south_largest = grid[0][j];
        // from the south looking north
        let mut south_to_north_largest = grid[m-1][j];
        for i in 1..m-1 {
            if grid[i][j] > north_to_south_largest {
                north_to_south_largest = grid[i][j];
                visible.insert(i*n + j);
            }
            if grid[m-1-i][j] > south_to_north_largest {
                south_to_north_largest = grid[m-1-i][j];
                visible.insert((m-1-i)*n + j);
            }
        }
    }


    visible
}

fn as_u8(ch: char) -> u8 {
    match ch {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        _ => panic!("Unexpected input: {ch}"),
    }
}

pub fn solve() {
    let input = fs::read_to_string("resources/day8.txt").unwrap();
    let grid: Vec<Vec<u8>> = input.lines()
        .map(|line| line.chars()
             .map(as_u8)
             .collect())
        .collect();

    let visible = visible_trees(&grid);
    println!("{}", visible.len());

    let score = max_scenic_score(&grid);
    println!("{}", score);
}
