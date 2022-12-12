use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn find_pos(input: &[u8], target: u8, cols: usize) -> (usize, usize) {
    let pos = input.iter().position(|c| *c == target).unwrap();
    let y = pos % (cols + 1); // +1 for the newline
    let x = pos / (cols + 1);
    (x, y)
}

fn main() {
    let input = include_bytes!("../input");
    let mut map: Vec<Vec<u8>> = input
        .split(|c| *c == b'\n')
        .map(|line| line.to_vec())
        .filter(|line| line.len() > 0)
        .collect();
    let rows = map.len();
    let cols = map[0].len();
    let (start_x, start_y) = find_pos(input, b'S', cols);
    let (end_x, end_y) = find_pos(input, b'E', cols);
    map[start_x][start_y] = b'a';
    map[end_x][end_y] = b'z';
    let mut dist: Vec<Vec<u32>> = vec![vec![u32::MAX; cols]; rows];
    dist[start_x][start_y] = 0;
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0u32, start_x, start_y)));
    let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    while let Some(Reverse((cost, x, y))) = heap.pop() {
        if cost > dist[x][y] {
            continue;
        }
        for (x_mod, y_mod) in directions {
            let new_x: usize = (x as i32 + x_mod) as usize;
            let new_y: usize = (y as i32 + y_mod) as usize;
            if new_x >= rows || new_y >= cols {
                continue;
            }
            if (map[new_x][new_y] <= (1 + map[x][y])) && (dist[new_x][new_y] > cost + 1) {
                dist[new_x][new_y] = cost + 1;
                heap.push(Reverse((cost + 1, new_x, new_y)));
            }
        }
    }
    println!("{}", dist[end_x][end_y]);

    // part 2
    let (start_x, start_y) = find_pos(input, b'E', cols);
    let (old_start_x, old_start_y) = find_pos(input, b'S', cols);
    map[old_start_x][old_start_y] = b'a';
    map[start_x][start_y] = b'z';
    let mut dist: Vec<Vec<u32>> = vec![vec![u32::MAX; cols]; rows];
    dist[start_x][start_y] = 0;
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0u32, start_x, start_y)));
    let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    while let Some(Reverse((cost, x, y))) = heap.pop() {
        if cost > dist[x][y] {
            continue;
        }
        for (x_mod, y_mod) in directions {
            let new_x: usize = (x as i32 + x_mod) as usize;
            let new_y: usize = (y as i32 + y_mod) as usize;
            if new_x >= rows || new_y >= cols {
                continue;
            }
            if (map[new_x][new_y] >= (map[x][y] - 1)) // inverted from part 1
		&& (dist[new_x][new_y] > cost + 1)
            {
                dist[new_x][new_y] = cost + 1;
                heap.push(Reverse((cost + 1, new_x, new_y)));
            }
        }
    }
    let mut end_x = 0;
    let mut end_y = 0;
    let mut end = u32::MAX;
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            if map[row][col] == b'a' {
                let score = dist[row][col];
                if score < end {
                    end_x = row;
                    end_y = col;
                    end = score;
                }
            }
        }
    }
    println!("{}", dist[end_x][end_y]);
}
