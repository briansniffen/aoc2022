use itertools::Either;

fn print_vis<T>(vis: &[T])
where
    T: AsRef<[bool]>,
{
    for row in vis {
        for cell in row.as_ref() {
            if *cell {
                print!(".");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
}

fn scenic<T>(input: &[T], x: usize, y: usize) -> u32
where
    T: AsRef<[u32]>,
{
    let scale = input.len();
    let mut scenic = 1;
    let reference = input[x].as_ref()[y];
    let dirs = [(true, true), (true, false), (false, true), (false, false)];
    for (swap_ij, lower_side) in dirs {
        let mut count = 0;
        let center = if swap_ij { y } else { x };
        let range = if lower_side {
            Either::Left((0..center).rev())
        } else {
            Either::Right(center + 1..scale)
        };
        for i in range {
            count += 1;
            let (a, b) = if swap_ij { (x, i) } else { (i, y) };
            if input[a].as_ref()[b] >= reference {
                break;
            }
        }
        scenic *= count;
    }
    scenic as u32
}

fn main() {
    let input: Vec<Vec<u32>> = include_str!("../input.txt")
        .lines()
        .map(|line| line.bytes().map(|c| c as u32 - b'0' as u32).collect())
        .collect();
    let scale = input[0].len();
    let mut vis = vec![vec![false; scale]; scale];
    let dirs = [(true, true), (true, false), (false, true), (false, false)];
    for (swap_ij, rev_inner) in dirs {
        for a in 0..scale {
            let mut max = 0;
            let range = if rev_inner {
                Either::Left(0..scale)
            } else {
                Either::Right((0..scale).rev())
            };
            for b in range {
                let (i, j) = if swap_ij { (a, b) } else { (b, a) };
                let cell = input[i][j];
                vis[i][j] |= cell > max;
                max = if cell > max { cell } else { max };
            }
        }
    }
    for i in 0..scale {
        // correct for edges
        vis[0][i] = true;
        vis[scale - 1][i] = true;
        vis[i][0] = true;
        vis[i][scale - 1] = true;
    }
    print_vis(&vis);
    let count: usize = vis
        .iter()
        .map(|row: &Vec<bool>| row.iter().filter(|b| **b).count())
        .sum();
    println!("{count}");
    let mut max = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    for i in 0..scale {
        for j in 0..scale {
            let score = scenic(&input, i, j);
            if score > max {
                max_x = i;
                max_y = j;
                max = score;
            }
            print!("{:3}", score);
        }
        println!("");
    }
    println!("{max} ({max_x},{max_y})");
}
