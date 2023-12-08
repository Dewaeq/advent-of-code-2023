pub fn part_one(input: &str) -> u32 {
    input.lines().map(|line| {
        let (_, numbers) = line.split_once(": ").unwrap();
        let parts = numbers.split('|').map(|x| x.split_whitespace().map(|y| y.parse::<u32>().unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>();

        let count = parts[1].iter().filter(|x| parts[0].contains(x)).count();
        if count == 0 {
            0
        } else {
            u32::pow(2, parts[1].iter().filter(|x| parts[0].contains(x)).count() as u32 - 1)
        }
    }).sum()
}

pub fn part_two(input: &str) -> u32 {
    let num_cards = input.lines().count();
    let mut counts = vec![1; num_cards];

    input.lines().enumerate().for_each(|(i, line)| {
        let (_, numbers) = line.split_once(": ").unwrap();
        let parts = numbers.split('|').map(|x| x.split_whitespace().map(|y| y.parse::<u32>().unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>();

        let count = parts[1].iter().filter(|x| parts[0].contains(x)).count();
        for j in (i + 1)..(i + count + 1).min(num_cards) {
            counts[j] += counts[i];
        }
    });

    counts.iter().sum::<usize>() as u32

    /*
     * Suuuuuuuper slow
     
    let mut counts = vec![1; input.lines().count()];
    let cards = input.lines().enumerate().collect::<Vec<_>>();
    let mut queue = cards.clone();

    while let Some((i, line)) = queue.pop() {
        let (_, numbers) = line.split_once(": ").unwrap();
        let parts = numbers.split('|').map(|x| x.split_whitespace().map(|y| y.parse::<u32>().unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>();

        let count = parts[1].iter().filter(|x| parts[0].contains(x)).count();

        for j in (i + 1)..(i + count + 1).min(cards.len()) {
            counts[j] += 1;
            queue.push(cards[j]);
        }
    }

    counts.iter().sum()*/
}
