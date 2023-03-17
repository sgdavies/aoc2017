use std::collections::VecDeque;

pub fn solve() {
    const STEPS: usize = 377;
    let mut buff = VecDeque::<u32>::new();
    buff.push_back(0);
    for x in 1..(2017 + 1) {
        let step = STEPS % buff.len();
        // move so that selection is at end of vec
        // [..., new_s, ..., old_s] -> [..., os, ..., ns] -> [..., os, ..., ns, x]
        // because we always start at the end, new_s is at [step]
        for _ in 0..buff.len() - step {
            let tmp = buff.pop_back().unwrap();
            buff.push_front(tmp);
        }
        buff.push_back(x);
    }
    println!("17 part one {}", buff.pop_front().unwrap());

    // part two - 50_000_000 iterations
    // We only need to know the value immediately after 0 - so just keep track
    // of when we would have inserted an item at (i.e. after) position 0.
    let mut after0 = 0;
    let mut length = 1;
    let mut pos = 0;
    for i in 1..50_000_000+1 {
        pos = (pos+STEPS) % length;
        if pos==0 {
            after0 = i;
        }
        pos += 1; // Inserted value becomes new position
        length += 1;
    }
    println!("17 part two {}", after0);
}
