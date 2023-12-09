use std::collections::HashMap;

pub fn part_one(input: &str) -> u32 {
    let (seeds_str, maps_str) = input[6..].split_once("seed-to-soil map:").unwrap();

    let mut seeds = HashMap::new();

    seeds_str
        .split_whitespace()
        .flat_map(|x| x.parse::<u32>())
        .for_each(|x| {
            seeds.insert(x, x);
        });

    let maps = maps_str
        .split("map:")
        .map(|x| {
            x.split_whitespace()
                .flat_map(|y| y.parse::<u32>())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for map in maps {
        let mut new_seeds = seeds.clone();

        for x in map.chunks(3) {
            let [dest_start, source_start, range]: [u32; 3] = x.try_into().unwrap();

            for i in 0..range {
                if let Some(_) = seeds.remove(&(source_start + i)) {
                    new_seeds.insert(dest_start + i, dest_start + i);
                    new_seeds.remove(&(source_start + i));
                }
            }
        }

        seeds = new_seeds;
    }

    *seeds.values().min().unwrap()
}
