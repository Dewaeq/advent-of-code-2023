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
