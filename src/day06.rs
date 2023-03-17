use std::collections::HashSet;

pub fn solve() {
    let mut blocks = vec![11, 11, 13, 7, 0, 15, 5, 5, 4, 4, 1, 1, 7, 1, 15, 11];
    let mut seen = HashSet::<Vec<u32>>::new();
    seen.insert(blocks.clone());
    let mut steps = 0;
    let mut part_one = None;
    loop {
        steps += 1;
        let mut maxi = 0;
        let mut max = 0u32;
        for (i, v) in blocks.iter().enumerate() {
            if v > &max {
                maxi = i;
                max = *v;
            }
        }

        blocks[maxi] = 0;
        while max > 0 {
            max -= 1;
            maxi += 1;
            if maxi == blocks.len() {
                maxi = 0;
            }
            blocks[maxi] += 1;
        }

        if seen.contains(&blocks) {
            if part_one.is_none() {
                part_one = Some(steps);
                steps = 0;
                seen = HashSet::<Vec<u32>>::new();
            } else {
                break;
            }
        }
        seen.insert(blocks.clone());
    }

    println!("06 part one {}", part_one.expect("Didn't get part one!?"));
    println!("06 part two {}", steps);
}
