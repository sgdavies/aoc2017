use std::collections::HashMap;

pub fn solve() {
    let input = include_str!("../23.txt").trim();
    let lines: Vec<&str> = input.split('\n').collect();
    let mut registers = HashMap::<&str, i64>::new();

    let (mul_count, _) = run(&lines, &mut registers);
    println!("23 part one {}", mul_count);

    // Part two - see comments in 23.txt
    // 3 loops would require ~1_000 * 100_000 * 100_000 = too many iterations
    // On each loop, f is only set to 0 (and hence h +1) if some d{2-b} * e{2-b} == b
    // i.e. if b is not prime.
    let mut b = 109_900;
    let mut not_prime_count = 0;
    while b <= 126_900 {
        if !prime(b) {
            not_prime_count += 1;
        }
        b += 17;
    }
    println!("23 part two {}", not_prime_count);
}

fn prime(x: i32) -> bool {
    if x % 2 == 0 {
        return false;
    }

    let stop = f64::sqrt(x as f64);
    let mut test = 3;
    while (test as f64) < stop {
        if x % test == 0 {
            return false;
        }
        test += 2;
    }

    true
}

fn run<'a>(
    lines: &Vec<&'a str>,
    registers: &'a mut HashMap<&'a str, i64>,
) -> (i32, &'a mut HashMap<&'a str, i64>) {
    let mut ip = 0;
    let mut mul_count = 0;
    loop {
        if ip >= lines.len() {
            break;
        }
        let mut words = lines.get(ip).unwrap().split_whitespace();
        let op = words.next().unwrap();
        let x = words.next().unwrap(); // May be a register or a value
        let y = value(words.next().unwrap(), registers);

        match op {
            "set" => {
                registers.insert(x, y);
            }
            "sub" => {
                let x_val = *registers.get(x).unwrap_or(&0);
                registers.insert(x, x_val - y);
            }
            "mul" => {
                mul_count += 1;
                let x_val = *registers.get(x).unwrap_or(&0);
                registers.insert(x, x_val * y);
            }
            "jnz" => {
                if value(x, registers) != 0 {
                    ip = (ip as i64 + y).try_into().unwrap();
                    ip -= 1;
                }
            }
            unknown => panic!("Unrecognised op: {}", unknown),
        }
        ip += 1;
    }
    (mul_count, registers)
}

fn value(s: &str, registers: &HashMap<&str, i64>) -> i64 {
    if let Ok(val) = s.parse::<i64>() {
        val
    } else {
        *registers.get(s).unwrap_or(&0)
    }
}
