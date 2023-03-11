use std::collections::HashMap;

pub fn solve() {
    let input = include_str!("../13.txt").trim();

    let scanners: HashMap<u32, u32> = input
        .lines()
        .into_iter()
        .map(|line| {
            let mut line = line.split(": ");
            let layer = line.next().unwrap().parse::<u32>().unwrap();
            let depth = line.next().unwrap().parse::<u32>().unwrap();
            (layer, depth)
        })
        .collect();

    let mut delay = 0;
    loop {
        // This is a bit slow because fold() operates on the whole map - it would be better
        // to iterate and drop out early as soon as being caught
        let (caught, severity) = scanners.iter().fold((false, 0), |acc, el| {
            let (c, s) = score(*el.0, *el.1, delay);
            (acc.0 | c, acc.1 + s)
        });
        if delay == 0 {
            println!("13 Part one: {}", severity);
        }
        if !caught {
            println!("13 Part two: {}", delay);
            break;
        }
        delay += 1;
    }
}

fn score(layer: u32, depth: u32, start_offset: u32) -> (bool, u32) {
    // calculate current scanner pos as a function of time (layer+start_offset)
    // scanner loops from [0, ..., N-1, ..., 1] then round to 0 again
    // length is 2*(N-1) = 2N-2
    // 3: [0 1 2|1]
    // 4: [0 1 2 3|2 1]
    let pos = (layer + start_offset) % (2 * depth - 2);
    // We only actually care if it's at 0 - not it's exact position
    // let pos = if pos < *depth {
    //     pos
    // } else {
    //     (2*depth -2) - pos
    // };
    if pos == 0 {
        (true, layer * depth) // A bit clunky but we need to spot being caught on layer 0
    } else {
        (false, 0)
    }
}
