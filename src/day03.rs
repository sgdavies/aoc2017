use crate::utils::neighbours;
use std::collections::HashMap;

const TARGET: u32 = 361527;
pub fn solve() {
    let mut x = 0i32;
    let mut y = 0i32;
    let mut num = 1;
    let mut vals: HashMap<(i32, i32), u32> = HashMap::new();
    vals.insert((x, y), num);
    let mut smax = 1;

    let mut part_one = None;
    let mut part_two = None;
    while part_one.is_none() || part_two.is_none() {
        // TODO ||
        // Right
        let mut s = 0;
        while s < smax {
            num += 1;
            s += 1;
            x += 1;

            if part_one.is_none() {
                if let Some(one) = check_one(x, y, num) {
                    part_one = Some(one);
                }
            }
            if part_two.is_none() {
                if let Some(two) = check_two(x, y, &mut vals) {
                    part_two = Some(two);
                }
            }
        }
        // Up
        s = 0;
        while s < smax {
            num += 1;
            s += 1;
            y += 1;

            if part_one.is_none() {
                if let Some(one) = check_one(x, y, num) {
                    part_one = Some(one);
                }
            }
            if part_two.is_none() {
                if let Some(two) = check_two(x, y, &mut vals) {
                    part_two = Some(two);
                }
            }
        }
        // Left - length increases
        smax += 1;
        s = 0;
        while s < smax {
            num += 1;
            s += 1;
            x -= 1;

            if part_one.is_none() {
                if let Some(one) = check_one(x, y, num) {
                    part_one = Some(one);
                }
            }
            if part_two.is_none() {
                if let Some(two) = check_two(x, y, &mut vals) {
                    part_two = Some(two);
                }
            }
        }
        // Down
        s = 0;
        while s < smax {
            num += 1;
            s += 1;
            y -= 1;

            if part_one.is_none() {
                if let Some(one) = check_one(x, y, num) {
                    part_one = Some(one);
                }
            }
            if part_two.is_none() {
                if let Some(two) = check_two(x, y, &mut vals) {
                    part_two = Some(two);
                }
            }
        }
        // length increases
        smax += 1;
    }

    println!("{}", part_one.expect("Didn't get part one answer"));
    println!("{}", part_two.expect("Didn't get part two answer"));
}

fn check_one(x: i32, y: i32, num: u32) -> Option<u32> {
    if num == TARGET {
        Some(x.abs() as u32 + y.abs() as u32)
    } else {
        None
    }
}

fn check_two(x: i32, y: i32, vals: &mut HashMap<(i32, i32), u32>) -> Option<u32> {
    let v = neighbours(x, y)
        .into_iter()
        .fold(0u32, |acc, xy| acc + *vals.get(&xy).unwrap_or(&0));
    vals.insert((x, y), v);

    if v > TARGET {
        Some(v)
    } else {
        None
    }
}
