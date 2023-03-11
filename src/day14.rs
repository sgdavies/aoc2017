use std::{collections::HashSet, u128};

pub fn solve() {
    let key = "jzgqcdpd"; // Example: "flqrgnkx", Actual: "jzgqcdpd"
    let ans: u32 = (0..128)
        .into_iter()
        .map(|x| count_bits(&super::day10::knot_hash(&format!("{}-{}", key, x))))
        .sum();
    println!("Day 14 part one: {}", ans);

    // part two
    let grid: HashSet<(i32, i32)> = (0..128u32)
        .into_iter()
        .map(|x| {
            (
                x,
                hash2hashset(&super::day10::knot_hash(&format!("{}-{}", key, x))),
            )
        })
        .fold(HashSet::new(), |acc, val| {
            let pairs = val.1.iter().map(|c| (val.0 as i32, *c as i32));
            let mut new_acc = acc.clone();
            new_acc.extend(pairs);
            new_acc
        });

    let mut checked = HashSet::<(i32, i32)>::new();
    let mut regions = 0;
    for row in 0..128i32 {
        for col in 0..128i32 {
            if !checked.contains(&(row, col)) {
                if grid.contains(&(row, col)) {
                    regions += 1;
                    checked.insert((row, col));
                    let mut region = adjacent(row, col);
                    while region.len() > 0 {
                        let candidate = region.pop().unwrap();
                        if grid.contains(&candidate) && !checked.contains(&candidate) {
                            checked.insert(candidate);
                            region.extend(adjacent(candidate.0, candidate.1));
                        }
                    }
                }
            }
        }
    }
    println!("Day 14 part two: {}", regions);
}

fn count_bits(hash: &str) -> u32 {
    hash2hashset(hash).len() as u32
}

fn hash2hashset(hash: &str) -> HashSet<u32> {
    let val: u128 = u128::from_str_radix(hash, 16).unwrap();
    (0..128u32).filter(|b| (val & 1 << b) > 0).collect()
}

fn adjacent(row: i32, col: i32) -> Vec<(i32, i32)> {
    vec![(-1, 0), (1, 0), (0, -1), (0, 1)]
        .iter()
        .map(|(dx, dy)| (row + dx, col + dy))
        .collect()
}
