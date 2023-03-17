use std::fs;

pub fn solve() {
    let input = fs::read_to_string(&"05.txt").expect("Can't read 05.txt");
    let mut ins: Vec<i32> = input
        .trim_end()
        .split('\n')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    let ins_orig = ins.clone();

    let mut ip = 0usize;
    let mut steps = 0u32;
    loop {
        steps += 1;
        let nip = ip as i32 + ins[ip];
        ins[ip] += 1;

        let nip = nip.try_into();
        if let Ok(u) = nip {
            if u >= ins.len() {
                break;
            } else {
                ip = u;
            }
        } else {
            break;
        }
    }
    println!("05 part one {}", steps);

    // Part two
    let mut ins = ins_orig;
    let mut ip = 0usize;
    let mut steps = 0u32;
    loop {
        steps += 1;
        let nip = ip as i32 + ins[ip];
        ins[ip] += if ins[ip] >= 3 { -1 } else { 1 };

        let nip = nip.try_into();
        if let Ok(u) = nip {
            if u >= ins.len() {
                break;
            } else {
                ip = u;
            }
        } else {
            break;
        }
    }
    println!("05 part two {}", steps);
}
