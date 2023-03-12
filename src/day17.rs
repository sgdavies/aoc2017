use std::collections::VecDeque;

pub fn solve() {
    let steps = 377;
    let mut buff = VecDeque::<u32>::new();
    buff.push_back(0);
    for x in 1..(2017 + 1) {
        let step = steps % buff.len();
        // move so that selection is at end of vec
        // [..., new_s, ..., old_s] -> [..., os, ..., ns] -> [..., os, ..., ns, x]
        // because we always start at the end, new_s is at [step]
        for _ in 0..buff.len() - step {
            let tmp = buff.pop_back().unwrap();
            buff.push_front(tmp);
        }
        buff.push_back(x);
    }
    println!("17 part one: {}", buff.pop_front().unwrap());

    // part two - 50_000_000 iterations
    // can't keep adding in middle of array (& resizing) -- too slow
    // Instead use linked list.  Implement as array[entry] = next.
    const LIM: usize = 50_000_000;
    let mut abuff = Vec::<usize>::with_capacity(LIM + 1); // Too large to alloc on stack
    for _ in 0..LIM + 1 {
        abuff.push(2 * LIM); // Seed with invalid value
    }
    let mut abuff = abuff.into_boxed_slice();
    abuff[0] = 1;
    abuff[1] = 0;

    let mut current = 1;
    for x in 2..LIM + 1 {
        for _ in 0..steps {
            current = abuff[current];
        }
        abuff[x] = abuff[current];
        abuff[current] = x;
        current = x;

        if x == 2017 {
            println!("... {}", abuff[x]); // part one
        }
    }

    println!("27 part two: {}", abuff[0]);
}
