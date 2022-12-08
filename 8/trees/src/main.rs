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
    let mut count = 0;
    let reference = input[x].as_ref()[y];
    for i in (0..x).rev() {
        count += 1;
        if input[i].as_ref()[y] >= reference {
            break;
        }
    }
    scenic *= count;
    count = 0;
    for i in x + 1..scale {
        count += 1;
        if input[i].as_ref()[y] >= reference {
            break;
        }
    }
    scenic *= count;
    count = 0;
    for j in (0..y).rev() {
        count += 1;
        if input[x].as_ref()[j] >= reference {
            break;
        }
    }
    scenic *= count;
    count = 0;
    for j in y + 1..scale {
        count += 1;
        if input[x].as_ref()[j] >= reference {
            break;
        }
    }
    scenic *= count;
    scenic as u32
}

fn main() {
    let input: Vec<Vec<u32>> = include_str!("../input.txt")
        .lines()
        .map(|line| line.bytes().map(|c| c as u32 - b'0' as u32).collect())
        .collect();
    let scale = input[0].len();
    let mut vis = vec![vec![false; scale]; scale];
    for i in 0..scale {
        // from the west
        let mut max = 0;
        for j in 0..scale {
            let cell = input[i][j];
            vis[i][j] |= cell > max;
            max = if cell > max { cell } else { max };
        }
    }
    for i in 0..scale {
        // from the esat
        let mut max = 0;
        for j in (0..scale).rev() {
            let cell = input[i][j];
            vis[i][j] |= cell > max;
            max = if cell > max { cell } else { max };
        }
    }
    for j in 0..scale {
        // from the north
        let mut max = 0;
        for i in 0..scale {
            let cell = input[i][j];
            vis[i][j] |= cell > max;
            max = if cell > max { cell } else { max };
        }
    }
    for j in 0..scale {
        // from the south
        let mut max = 0;
        for i in (0..scale).rev() {
            let cell = input[i][j];
            vis[i][j] |= cell > max;
            max = if cell > max { cell } else { max };
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
