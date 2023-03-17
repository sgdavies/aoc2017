use std::collections::HashMap;

pub fn solve() {
    const STEPS: u32 = 12368930;
    let mut lines = include_str!("../25.txt").trim().split('\n');
    lines.next(); // start in state A
    lines.next(); // steps

    let mut states = HashMap::<char, [Action; 2]>::new();
    loop {
        // Blank line or end
        if lines.next().is_none() {
            break;
        }
        let name = lines.next().unwrap().chars().nth(9).unwrap();
        lines.next(); // if current==0
        let v0 = lines
            .next()
            .unwrap()
            .chars()
            .nth(22)
            .unwrap()
            .to_digit(10)
            .unwrap() as usize;
        let d0 = if lines.next().unwrap().chars().nth(27).unwrap() == 'r' {
            true
        } else {
            false
        };
        let ns0 = lines.next().unwrap().chars().nth(26).unwrap();

        lines.next(); // if current==1
        let v1 = lines
            .next()
            .unwrap()
            .chars()
            .nth(22)
            .unwrap()
            .to_digit(10)
            .unwrap() as usize;
        let d1 = if lines.next().unwrap().chars().nth(27).unwrap() == 'r' {
            true
        } else {
            false
        };
        let ns1 = lines.next().unwrap().chars().nth(26).unwrap();

        states.insert(
            name,
            [
                Action {
                    val: v0,
                    step_right: d0,
                    next: ns0,
                },
                Action {
                    val: v1,
                    step_right: d1,
                    next: ns1,
                },
            ],
        );
    }

    const TAPE_LEN: usize = 10_000; // Hack - just increase if this turns out to be too small
    let mut tape = [0usize; TAPE_LEN];
    let mut head = TAPE_LEN / 2;
    let mut state = 'A';
    for _ in 0..STEPS {
        let action = &states.get(&state).unwrap()[tape[head]];
        tape[head] = action.val;
        state = action.next;
        match action.step_right {
            true => head += 1,
            false => head -= 1,
        }
    }

    println!("Merry Christmas! {}", tape.into_iter().sum::<usize>());
}

struct Action {
    val: usize,
    step_right: bool, // left false, right true
    next: char,
}
