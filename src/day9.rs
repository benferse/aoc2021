use std::collections::{BinaryHeap, VecDeque};

pub fn find_low_points(height_map: &[&[u8]]) -> Vec<(u8, (usize, usize))> {
    let mut low_points = vec![];
    let n_cols = height_map[0].len();
    let n_rows = height_map.len();

    for j in 0..n_rows {
        for k in 0..n_cols {
            let c = height_map[j][k];

            // Check each direction to see if this point is lower
            // than all of them
            if (j > 0 && height_map[j-1][k] <= c) ||
               (j < n_rows - 1 && height_map[j+1][k] <= c) ||
               (k > 0 && height_map[j][k-1] <= c) ||
               (k < n_cols - 1 && height_map[j][k+1] <= c){
                continue;
            }

            let d = c - '0' as u8;
            low_points.push((d, (j, k)));
        }
    }

    low_points
}

static NINE: u8 = '9' as u8;
static VISITED: u8 = 'x' as u8;

pub fn find_basins(height_map: &mut [&mut [u8]]) -> BinaryHeap<Vec<(u8, u8)>> {
    let mut basins = BinaryHeap::new();

    for x in 0..height_map.len() {
        for y in 0..height_map[0].len() {
            let node = height_map[x][y];
            if !(node == NINE || node == VISITED) {
                let basin = find_basin(height_map, x, y);
                basins.push(basin);
            }
        }
    }

    basins
}

pub fn find_basin(height_map: &mut [&mut [u8]], x: usize, y: usize) -> Vec<(u8, u8)> {
    // Perform a BFS starting at (x,y), proceeding in all four directions,
    // stopping when reaching a node that has already been visited, or a '9'
    // which is not inside any basin by definition
    let mut basin_nodes = vec![];
    let mut q = VecDeque::new();

    q.push_back((x, y));

    while let Some(node) = q.pop_front() {
        height_map[node.0][node.1] = VISITED;
    }

    basin_nodes
}

#[cfg(test)]
mod answers {
    use super::*;

    static SAMPLE: &[&[u8]] = &[
        "2199943210".as_bytes(),
        "3987894921".as_bytes(),
        "9856789892".as_bytes(),
        "8767896789".as_bytes(),
        "9899965678".as_bytes(),
    ];

    #[test]
    fn example1() {
        let mut result = find_low_points(SAMPLE);
        result.sort_unstable();
        assert_eq!(result, vec![(0, (0,9)), (1, (0, 1)), (5, (2, 2)), (5, (4, 6))]);

        let c = result
            .into_iter()
            .fold(0u32, |acc, x| acc + 1 + x.0 as u32);

        assert_eq!(c, 15);
    }

   #[test]
   fn puzzle1() {
        let input = include_str!("./input/day9")
            .lines()
            .map(|line| line.trim().as_bytes())
            .collect::<Vec<_>>();

        let result = find_low_points(&input)
            .into_iter()
            .fold(0u32, |acc, x| acc + 1 + x.0 as u32);

        assert_eq!(result, 436);
    }
}
