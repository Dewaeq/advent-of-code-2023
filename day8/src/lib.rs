use std::cell::UnsafeCell;

struct Node<'a> {
    edges: UnsafeCell<Vec<&'a Node<'a>>>,
    name: &'a str,
    is_end: bool,
}

impl Node<'_> {
    fn new(name: &'static str, part_two: bool) -> Self {
        Node {
            edges: UnsafeCell::new(Vec::new()),
            name,
            is_end: name == "ZZZ" || part_two && name.ends_with("Z"),
        }
    }
}

pub fn part_one(input: &'static str) -> i32 {
    let (instr_str, path_str) = input.split_once("\r\n\r\n").unwrap();

    let instructions = instr_str
        .chars()
        .map(|c| (c == 'R') as usize)
        .collect::<Vec<_>>();

    let temp_nodes = path_str
        .lines()
        .map(|line| {
            (
                Node::new(&line[0..3], false),
                line[7..]
                    .split(", ")
                    .map(|x| x.replace(")", ""))
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    let mut nodes = vec![];

    for (node, edges) in &temp_nodes {
        let left = &temp_nodes.iter().find(|x| x.0.name == edges[0]).unwrap().0;
        let right = &temp_nodes.iter().find(|x| x.0.name == edges[1]).unwrap().0;

        unsafe {
            (*node.edges.get()).push(left);
            (*node.edges.get()).push(right);
        };

        nodes.push(node);
    }

    let root = *nodes.iter().find(|x| x.name == "AAA").unwrap();

    find_dist(root, &instructions)
}

fn find_dist(mut root: &Node, instructions: &Vec<usize>) -> i32 {
    let mut steps = 0;

    for instr in instructions.iter().cycle() {
        if root.is_end {
            break;
        }

        root = unsafe { (*root.edges.get())[*instr] };

        steps += 1;
    }

    steps
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        if b < a {
            std::mem::swap(&mut a, &mut b);
        }

        b %= a;
    }

    a
}

fn lcm(list: Vec<i32>) -> u64 {
    let list = list.into_iter().map(|x| x as u64).collect::<Vec<_>>();
    let mut x = list[0] * list[1] / gcd(list[0], list[1]);

    for i in 2..list.len() {
        x = x * list[i] / gcd(x, list[i])
    }

    x
}

pub fn part_two(input: &'static str) -> u64 {
    let (instr_str, path_str) = input.split_once("\r\n\r\n").unwrap();

    let instructions = instr_str
        .chars()
        .map(|c| (c == 'R') as usize)
        .collect::<Vec<_>>();

    let temp_nodes = path_str
        .lines()
        .map(|line| {
            (
                Node::new(&line[0..3], true),
                line[7..]
                    .split(", ")
                    .map(|x| x.replace(")", ""))
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    let mut nodes = vec![];

    for (node, edges) in &temp_nodes {
        let left = &temp_nodes.iter().find(|x| x.0.name == edges[0]).unwrap().0;
        let right = &temp_nodes.iter().find(|x| x.0.name == edges[1]).unwrap().0;

        unsafe {
            (*node.edges.get()).push(left);
            (*node.edges.get()).push(right);
        };

        nodes.push(node);
    }

    let roots = nodes
        .iter()
        .filter(|x| x.name.ends_with("A"))
        .collect::<Vec<_>>();

    let dists = roots
        .iter()
        .map(|&node| find_dist(node, &instructions))
        .collect::<Vec<_>>();

    lcm(dists)
}
