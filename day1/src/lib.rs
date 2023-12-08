pub fn part_one(input: &str) -> u32 {
    let mut s = 0;
    for line in input.lines() {
        let digits = line
            .chars()
            .map(|c| c.to_digit(10))
            .flatten()
            .collect::<Vec<_>>();
        s += digits.first().unwrap() * 10 + digits.last().unwrap();
    }

    s
}

fn read_digit(line: &str) -> Option<u32> {
    const TEXT: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    match line.chars().next()?.to_digit(10) {
        Some(d) => Some(d),
        _ => TEXT
            .iter()
            .enumerate()
            .find(|&(i, text)| line.starts_with(text))
            .map(|x| x.0 as u32 + 1),
    }
}

pub fn part_two(input: &str) -> u32 {
    let mut s = 0;
    for l in input.lines() {
        let mut digits = vec![];

        for i in 0..l.len() {
            let a = read_digit(&l[i..]);
            if let Some(d) = a {
                digits.push(d);
            }
        }

        s += digits.first().unwrap() * 10 + digits.last().unwrap();
    }

    s
}
