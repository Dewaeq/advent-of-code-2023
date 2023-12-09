pub fn part_one(input: &str) -> u32 {
    let (seeds_str, maps_str) = input[6..].split_once("seed-to-soil map:").unwrap();

    let mut ss = seeds_str
        .split_whitespace()
        .flat_map(|x| x.parse::<u32>())
        .collect::<Vec<_>>();

    let maps = maps_str
        .split("map:")
        .map(|x| {
            x.split_whitespace()
                .flat_map(|y| y.parse::<u32>())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for map in maps {
        let mut new_ss = ss.clone();

        for x in map.chunks(3) {
            let [dest_start, source_start, range]: [u32; 3] = x.try_into().unwrap();

            for (i, s) in ss.iter().enumerate() {
                if *s >= source_start && *s < source_start + range {
                    new_ss[i] = *s - source_start + dest_start;
                }
            }
        }

        ss = new_ss.clone();
    }

    *ss.iter().min().unwrap()
}
