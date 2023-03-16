use std::collections::VecDeque;

pub fn solve() {
    let input = vec![
        106, 16, 254, 226, 55, 2, 1, 166, 177, 247, 93, 0, 255, 228, 60, 36,
    ];

    println!("{}", part_one(256, &input));
    println!(
        "{}",
        knot_hash("106,16,254,226,55,2,1,166,177,247,93,0,255,228,60,36")
    );
}

fn part_one(size: u16, input: &[u8]) -> u16 {
    let v: Vec<u16> = (0..size).collect();
    let (start, _skip, v) = knot(0, 0, input, &v);

    // println!("{} - {:?}", start, v);
    if start < v.len() - 1 {
        v[start] * v[start + 1]
    } else {
        v[start] * v[0]
    }
}

pub fn knot_hash(input: &str) -> String {
    let mut input: Vec<u8> = input.chars().into_iter().map(|c| c as u8).collect();
    input.extend_from_slice(&[17, 31, 73, 47, 23]);

    let mut start = 0;
    let mut skip = 0;
    let mut v: Vec<u16> = (0..256).collect();
    for _ in 0..64 {
        // (start, skip, v) = knot(start, skip, &input, &v);
        let blob = knot(start, skip, &input, &v);
        start = blob.0;
        skip = blob.1;
        v = blob.2;
    }

    // Rotate back to the start
    let mut v_end = Vec::<u16>::new();
    v_end.extend_from_slice(&v[start..v.len()]);
    v_end.extend_from_slice(&v[0..start]);

    let dense = s2d(&v_end);
    dense
        .iter()
        .map(|x| format!("{:02x}", x))
        .collect::<Vec<String>>()
        .join("")
}

fn s2d(v: &[u16]) -> Vec<u8> {
    let mut out = Vec::<u8>::new();
    let mut v = VecDeque::<u16>::from(v.to_owned());
    while !v.is_empty() {
        let mut next = 0u8;
        for _ in 0..16 {
            next ^= v.pop_front().unwrap() as u8;
        }
        out.push(next);
    }

    out
}

fn knot(start: usize, skip: usize, input: &[u8], v: &[u16]) -> (usize, usize, Vec<u16>) {
    let mut start = start;
    let mut skip = skip;
    let mut v = v.to_owned();

    for l in input {
        // println!("{},{},{} - {:?}", start, skip, l, v);
        let mut rev = Vec::<u16>::new();
        rev.extend_from_slice(&v[0..(*l as usize)]);
        rev.reverse();
        rev.extend_from_slice(&v[(*l as usize)..v.len()]);

        let the_skip = (*l as usize + skip) % v.len();
        v = Vec::<u16>::new();
        v.extend_from_slice(&rev[the_skip..rev.len()]);
        v.extend_from_slice(&rev[0..the_skip]);

        start += v.len() - the_skip;
        start %= v.len();
        skip += 1;
    }

    (start, skip, v.to_vec())
}

#[cfg(test)]
mod tests {
    use super::knot_hash;
    use super::part_one;

    #[test]
    fn test_one() {
        assert!(part_one(5, &vec![3u8, 4, 1, 5]) == 12);
    }

    #[test]
    fn test_two() {
        let t1 = knot_hash("");
        println!("{}", t1);
        assert!(t1 == "a2582a3a0e66e6e86e3812dcb672a272");
        assert!(knot_hash("AoC 2017") == "33efeb34ea91902bb2f59c9920caa6cd");
        assert!(knot_hash("1,2,3") == "3efbe78a8d82f29979031a4aa0b16a9d");
        assert!(knot_hash("1,2,4") == "63960835bcdc130f0b66d7ff4f6a5a8e");
    }
}
