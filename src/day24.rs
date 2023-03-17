use std::collections::HashSet;

pub fn solve() {
    let input = include_str!("../24.txt").trim();
    let components: Vec<(u32, u32)> = input
        .split('\n')
        .into_iter()
        .map(|s| {
            let mut words = s.split('/');
            let a = words.next().unwrap().parse::<u32>().unwrap();
            let b = words.next().unwrap().parse::<u32>().unwrap();
            (a, b)
        })
        .collect();
    let set: HashSet<(u32, u32)> = components.iter().copied().collect();
    assert_eq!(components.len(), set.len());

    let ans = solve_from(0, 0, (0, 0, 0), 0, set);
    println!("24 part one {}", ans.0);
    println!("24 part two {}", ans.2);
}

pub fn solve_from(
    score: u32,
    length: u32,
    best: (u32, u32, u32),
    end: u32,
    pairs_left: HashSet<(u32, u32)>,
) -> (u32, u32, u32) {
    let mut strongest = best.0;
    let mut longest = best.1;
    let mut strongest_longest = best.2;

    for pair in &pairs_left {
        if pair.0 == end || pair.1 == end {
            let new_end = if pair.0 == end { pair.1 } else { pair.0 };
            let mut new_pairs_left = pairs_left.clone();
            new_pairs_left.remove(pair);

            let new_score = score + pair.0 + pair.1;
            let new_sl = if length + 1 >= longest && new_score > strongest_longest {
                new_score
            } else {
                strongest_longest
            };
            let new_best = (
                std::cmp::max(strongest, new_score),
                std::cmp::max(longest, length + 1),
                new_sl,
            );

            let res = solve_from(new_score, length + 1, new_best, new_end, new_pairs_left);
            let new_strongest = res.0;
            let new_longest = res.1;
            let new_strongest_longest = res.2;
            if new_strongest > strongest {
                strongest = new_strongest;
            }
            if new_longest >= longest {
                longest = new_longest;
                if new_strongest_longest > strongest_longest {
                    strongest_longest = new_strongest_longest;
                }
            }
        }
    }

    (strongest, longest, strongest_longest)
}
