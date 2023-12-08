pub fn part_one(input: &str) -> u32 {
    input
        .lines()
        .map(|game| {
            let (info, sets) = game.split_once(":").unwrap();
            let index = info[5..].parse::<u32>().unwrap();

            let valid = sets.replace(";", ",").split(",").all(|x| {
                let (count, color) = x.trim().split_once(" ").unwrap();
                let c = count.parse::<u32>().unwrap();
                match color {
                    "green" => c <= 13,
                    "blue" => c <= 14,
                    "red" => c <= 12,
                    _ => true,
                }
            });

            index * (valid as u32)
        })
        .sum::<u32>()
}

pub fn part_two(input: &str) -> u32 {
    input
        .lines()
        .map(|game| {
            let (_, sets) = game.split_once(":").unwrap();
            let (mut r, mut g, mut b) = (0, 0, 0);

            sets.replace(";", ",").split(",").for_each(|x| {
                let (count, color) = x.trim().split_once(" ").unwrap();
                let c = count.parse::<u32>().unwrap();
                match color {
                    "red" => r = r.max(c),
                    "green" => g = g.max(c),
                    "blue" => b = b.max(c),
                    _ => (),
                }
            });

            r * g * b
        })
        .sum::<u32>()
}
