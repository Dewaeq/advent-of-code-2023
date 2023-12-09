// general formule: d = (t - x) * x

pub fn part_one(input: &str) -> u32 {
    let (time_str, dist_str) = input.split_once("\n").unwrap();

    let time = time_str.split_whitespace().flat_map(|x| x.parse::<u32>());
    let dist = dist_str.split_whitespace().flat_map(|x| x.parse::<u32>());

    time.zip(dist)
        .map(|(t, d)| {
            let t = t as f32;
            let d = d as f32;
            let x = (t.powi(2) / 4. - d).sqrt();

            let x1 = x + t / 2.;
            let x2 = -x + t / 2.;

            ((x1.floor() - x2.floor()).abs() - ((x1 - x1.floor()) < 0.001) as u32 as f32) as u32
        })
        .product()
}

pub fn part_two(input: &str) -> u32 {
    let (time_str, dist_str) = input.split_once("\n").unwrap();

    let t = time_str[5..]
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse::<f32>()
        .unwrap();
    let d = dist_str[9..]
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse::<f32>()
        .unwrap();

    let x = (t.powi(2) / 4. - d).sqrt();

    let x1 = x + t / 2.;
    let x2 = -x + t / 2.;

    ((x1.floor() - x2.floor()).abs() - ((x1 - x1.floor()) < 0.001) as u32 as f32) as u32
}
