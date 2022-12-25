fn from_snafu(s: &str) -> i64 {
    let mut place = 1;
    let mut total = 0;
    for digit in s.chars().rev() {
        let digit = match digit {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => panic!(),
        };
        total += place * digit;
        place *= 5;
    }
    total
}

fn to_snafu(i: i64) -> String {
    let mut result = vec![];
    let mut x = i;
    loop {
        let m = x % 5;
        let (digit, adj) = match m {
            0 => ('0', 0),
            1 => ('1', 0),
            2 => ('2', 0),
            3 => ('=', 1),
            4 => ('-', 1),
            _ => panic!(),
        };
        x = x / 5 + adj;
        result.push(digit);
        if x == 0 {
            break;
        }
    }
    result.into_iter().rev().collect()
}

fn main() {
    let input = include_str!("../input.txt");
    let ans = input.lines().map(|x| from_snafu(x)).sum();
    println!("part 1: {} ({})", to_snafu(ans), ans);
}
