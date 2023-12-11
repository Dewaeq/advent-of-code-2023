fn diff(v: Vec<i32>) -> Vec<i32> {
    (1..v.len()).map(|i| v[i] - v[i - 1]).collect::<Vec<_>>()
}

fn all_zero(v: &Vec<i32>) -> bool {
    v.iter().all(|&x| x == 0)
}

fn extrapolate(v: Vec<i32>, part_one: bool) -> i32 {
    if all_zero(&v) {
        return 0;
    }

    if part_one {
        *v.last().unwrap() + extrapolate(diff(v), part_one)
    } else {
        *v.first().unwrap() - extrapolate(diff(v), part_one)
    }
}

pub fn part_one(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let history = line
                .split_whitespace()
                .flat_map(|x| x.parse::<i32>())
                .collect::<Vec<_>>();

            extrapolate(history, true)
        })
        .sum()
}

pub fn part_two(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let history = line
                .split_whitespace()
                .flat_map(|x| x.parse::<i32>())
                .collect::<Vec<_>>();

            extrapolate(history, false)
        })
        .sum()
}
