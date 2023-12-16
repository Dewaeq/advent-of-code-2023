use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

pub fn part_one(input: &str) -> usize {
    arrangements(input, true)
}

pub fn part_two(input: &str) -> usize {
    arrangements(input, false)
}

fn arrangements(input: &str, part_one: bool) -> usize {
    let mut cache = HashMap::new();

    input
        .lines()
        .map(|line| {
            let (record, nums) = line.split_once(' ').unwrap();
            let mut nums = nums
                .split(',')
                .flat_map(|x| x.parse::<usize>())
                .collect::<Vec<_>>();

            let mut record = record.chars().collect::<Vec<_>>();
            if !part_one {
                record.push('?');
                record = expand(record);
                record.remove(record.len() - 1);

                nums = expand(nums);
            }

            solve(&record, &nums, &mut cache)
        })
        .sum()
}

fn solve(record: &[char], nums: &[usize], cache: &mut HashMap<u64, usize>) -> usize {
    let mut hasher = DefaultHasher::new();

    record.hash(&mut hasher);
    nums.hash(&mut hasher);

    let key = hasher.finish();

    if let Some(&count) = cache.get(&key) {
        return count;
    }

    // 0 if record is empty but not all groups are filled
    if record.is_empty() {
        return nums.is_empty() as usize;
    }

    // 0 if all groups are filled, but there's still a defect spring left
    if nums.is_empty() {
        return !record.contains(&'#') as usize;
    }

    let mut count = 0;

    if record[0] == '?' || record[0] == '.' {
        count += solve(&record[1..], nums, cache);
    }

    if record[0] == '#' || record[0] == '?' {
        if nums[0] <= record.len()
            && !record[..nums[0]].contains(&'.')
            && (nums[0] == record.len() || record[nums[0]] != '#')
        {
            let i = (nums[0] + 1).min(record.len());
            count += solve(&record[i..], &nums[1..], cache);
        }
    }

    cache.insert(key, count);

    count
}

fn expand<U: Clone>(mut v: Vec<U>) -> Vec<U> {
    let t = v.clone();

    for _ in 0..4 {
        v.extend(t.clone());
    }

    v
}
