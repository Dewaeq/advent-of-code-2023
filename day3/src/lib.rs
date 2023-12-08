fn read_part_number(scheme: &str) -> Option<u32> {
    scheme
        .chars()
        .take_while(|c| c.is_digit(10))
        .collect::<String>()
        .parse::<u32>()
        .ok()
}

pub fn part_one(input: &str) -> u32 {
    let mut s = 0;

    let lines = input.lines().collect::<Vec<_>>();
    let (width, height) = (lines[0].trim().len(), lines.len());

    let chars = input
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>();

    let mut i = 0;
    while i < chars.len() {
        let (x, y) = (i % width, i / height);
        if let Some(number) = read_part_number(&chars[i..]) {
            let number_width = (number.ilog10() + 1) as usize;

            let x_interval = x.saturating_sub(1)..(x + number_width + 1).min(width);
            let neighbours = lines[y.saturating_sub(1)][x_interval.clone()].to_owned()
                + &lines[y][x_interval.clone()]
                + &lines[(y + 1).min(height - 1)][x_interval.clone()];

            if neighbours
                .chars()
                .any(|c| c != '.' && !c.is_digit(10) && !c.is_whitespace())
            {
                s += number;
            }
            i += number_width;
        } else {
            i += 1;
        }
    }

    s
}

pub fn part_two(input: &str) -> u32 {
    let lines = input.lines().collect::<Vec<_>>();
    let (width, height) = (lines[0].trim().len(), lines.len());

    let chars = input
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>();

    let mut i = 0;
    let mut part_numbers = vec![];
    while i < chars.len() {
        let (x, y) = (i % width, i / height);
        if let Some(number) = read_part_number(&chars[i..]) {
            let number_width = (number.ilog10() + 1) as usize;

            let x_interval = x.saturating_sub(1)..(x + number_width + 1).min(width);
            if y > 0 && lines[y - 1][x_interval.clone()].contains('*') {
                let pos = (y - 1) * width
                    + x_interval.start
                    + lines[y - 1][x_interval.clone()].find('*').unwrap();
                part_numbers.push((number, pos));
            }
            if lines[y][x_interval.clone()].contains('*') {
                let pos =
                    y * width + x_interval.start + lines[y][x_interval.clone()].find('*').unwrap();
                part_numbers.push((number, pos));
            }
            if y < height - 1 && lines[y + 1][x_interval.clone()].contains('*') {
                let pos = (y + 1) * width
                    + x_interval.start
                    + lines[y + 1][x_interval.clone()].find('*').unwrap();
                part_numbers.push((number, pos));
            }

            i += number_width;
        } else {
            i += 1;
        }
    }

    part_numbers.sort_by_key(|x| x.1);

    part_numbers
        .iter()
        .filter(|x| part_numbers.iter().filter(|y| y.1 == x.1).count() == 2)
        .map(|x| x.0)
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|x| x[0] * x[1])
        .sum()
}
