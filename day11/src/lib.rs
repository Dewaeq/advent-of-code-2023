pub fn part_one(input: &str) -> i32 {
    let mut galaxies = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.trim().chars().enumerate().flat_map(move |(x, c)| {
                if c == '#' {
                    Some((x as i32, y as i32))
                } else {
                    None
                }
            })
        })
        .collect::<Vec<_>>();

    let width = input.lines().nth(0).unwrap().trim().len();
    let height = input.lines().count();

    let mut open_rows: Vec<i32> = vec![0; height];
    let mut open_cols: Vec<i32> = vec![0; width];

    for (y, l) in input.lines().enumerate() {
        if !l.contains("#") {
            for i in y..height {
                open_rows[i] += 1;
            }
        }
    }

    for x in 0..input.lines().nth(0).unwrap().trim().len() {
        if !galaxies.iter().any(|g| g.0 == x as i32) {
            for i in x..width {
                open_cols[i] += 1;
            }
        }
    }

    for g in &mut galaxies {
        g.0 += open_cols[g.0 as usize];
        g.1 += open_rows[g.1 as usize];
    }

    let mut s = 0;

    for a in &galaxies {
        for b in &galaxies {
            s += (a.0 - b.0).abs() + (a.1 - b.1).abs()
        }
    }

    s / 2
}

pub fn part_two(input: &str) -> i64 {
    let mut galaxies = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.trim().chars().enumerate().flat_map(move |(x, c)| {
                if c == '#' {
                    Some((x as i64, y as i64))
                } else {
                    None
                }
            })
        })
        .collect::<Vec<_>>();

    let width = input.lines().nth(0).unwrap().trim().len();
    let height = input.lines().count();

    let mut open_rows: Vec<i64> = vec![0; height];
    let mut open_cols: Vec<i64> = vec![0; width];

    for (y, l) in input.lines().enumerate() {
        if !l.contains("#") {
            for i in y..height {
                open_rows[i] += 1;
            }
        }
    }

    for x in 0..input.lines().nth(0).unwrap().trim().len() {
        if !galaxies.iter().any(|g| g.0 == x as i64) {
            for i in x..width {
                open_cols[i] += 1;
            }
        }
    }

    for g in &mut galaxies {
        g.0 += open_cols[g.0 as usize] * (1_000_000 - 1);
        g.1 += open_rows[g.1 as usize] * (1_000_000 - 1);
    }

    let mut s = 0;

    for a in &galaxies {
        for b in &galaxies {
            s += (a.0 - b.0).abs() + (a.1 - b.1).abs()
        }
    }

    s / 2
}
