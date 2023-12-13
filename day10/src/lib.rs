#[derive(Clone, Copy, Debug, PartialEq)]
struct Pipe {
    a: Option<usize>,
    b: Option<usize>,
    is_start: bool,
    c: char,
    pos: (i32, i32),
}

impl Pipe {
    fn new(c: char, i: usize, width: usize, height: usize) -> Self {
        let (x, y) = (i % width, i / width);
        let (mut a, mut b) = match c {
            '|' => (i.checked_sub(width), Some(i + width)),
            '-' => (i.checked_sub(1), Some(i + 1)),
            'L' => (i.checked_sub(width), Some(i + 1)),
            'J' => (i.checked_sub(width), i.checked_sub(1)),
            '7' => (Some(i + width), i.checked_sub(1)),
            'F' => (Some(i + 1), Some(i + width)),
            _ => (None, None),
        };

        if let Some(k) = a {
            if (k == i + 1 && x == width - 1)
                || (k == x + width && y == height - 1)
                || (k == i - 1 && x == 0)
                || (k == i - width && y == 0)
            {
                a = None;
            }
        }
        if let Some(k) = b {
            if (k == i + 1 && x == width - 1)
                || (k == x + width && y == height - 1)
                || (k == i - 1 && x == 0)
                || (k == i - width && y == 0)
            {
                b = None;
            }
        }

        Pipe {
            a,
            b,
            is_start: c == 'S',
            c,
            pos: (x as i32, y as i32),
        }
    }

    fn next(&self, from: usize) -> Option<usize> {
        if self.a == Some(from) {
            self.b
        } else if self.b == Some(from) {
            self.a
        } else {
            None
        }
    }
}

fn find_loop(map: &Vec<Pipe>, start: usize) -> Option<Vec<Pipe>> {
    let mut path = vec![map[start]];
    let mut prev = start;
    let mut idx = map[start].a;

    while let Some(i) = idx {
        if i == start {
            break;
        }

        path.push(map[i]);
        idx = map[i].next(prev);
        prev = i;
    }

    idx.map(|_| path)
}

pub fn part_one(input: &str) -> i32 {
    let width = input.lines().nth(0).unwrap().trim().len();
    let height = input.lines().count();

    let mut map = input
        .replace("\n", "")
        .replace("\r", "")
        .chars()
        .enumerate()
        .map(|(i, c)| Pipe::new(c, i, width, height))
        .collect::<Vec<_>>();

    let start_index = map.iter().position(|x| x.is_start).unwrap();
    for t in ['|', '-', 'L', 'F', 'J', '7'] {
        map[start_index] = Pipe::new(t, start_index, width, height);

        if let Some(l) = find_loop(&map, start_index) {
            for (i, t) in map.iter().enumerate() {
                if l.contains(t) {
                    print!("{}", t.c)
                } else {
                    print!(".");
                }
                if (i + 1) % width == 0 {
                    println!();
                }
            }
            return (l.len() / 2) as i32;
        }
    }

    0
}

/// Combination of Pick's theorem (A = interior + points/2 - 1)
/// and the Shoelace formula to calculate A
fn num_enclosed(points: &Vec<(i32, i32)>) -> i32 {
    points
        .iter()
        .enumerate()
        .map(|(i, &p)| det(p, points[(i + 1) % points.len()]))
        .sum::<i32>()
        .abs()
        / 2
        - (points.len() / 2) as i32
        + 1
}

fn det(a: (i32, i32), b: (i32, i32)) -> i32 {
    a.0 * b.1 - a.1 * b.0
}

pub fn part_two(input: &str) -> i32 {
    let width = input.lines().nth(0).unwrap().trim().len();
    let height = input.lines().count();

    let mut map = input
        .replace("\n", "")
        .replace("\r", "")
        .chars()
        .enumerate()
        .map(|(i, c)| Pipe::new(c, i, width, height))
        .collect::<Vec<_>>();

    let start_index = map.iter().position(|x| x.is_start).unwrap();

    for t in ['|', '-', 'L', 'F', 'J', '7'] {
        map[start_index] = Pipe::new(t, start_index, width, height);

        if let Some(l) = find_loop(&map, start_index) {
            let l = l
                .iter()
                .map(|p| (p.pos.0 as i32, p.pos.1 as i32))
                .collect::<Vec<_>>();

            return num_enclosed(&l);
        }
    }

    0
}
