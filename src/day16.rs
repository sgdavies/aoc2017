use crate::utils::p2s;

pub fn solve() {
    let input = include_str!("../16.txt").trim();
    let mut programs = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
    ];
    let mut iter = 0;
    loop {
        dance_once(input, &mut programs);
        iter += 1;

        if iter == 1 {
            println!("16 part one {}", p2s(&programs));
        }
        if programs
            == [
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
            ]
        {
            break;
        }
    }

    let remaining_loops = 1_000_000_000 % iter;
    for _ in 0..remaining_loops {
        dance_once(input, &mut programs);
    }

    println!("16 part two {}", p2s(&programs));
}

fn dance_once(input: &str, programs: &mut [char; 16]) {
    for line in input.to_string().split(',') {
        let mut chars = line.chars();
        match chars.next().unwrap() {
            's' => {
                let amount = chars.as_str().parse::<usize>().unwrap();
                let mut new_progs: [char; 16] = [' '; 16];
                for i in 0..amount {
                    new_progs[i] = programs[programs.len() - amount + i];
                }
                for i in 0..programs.len() - amount {
                    new_progs[amount + i] = programs[i];
                }
                *programs = new_progs;
            }
            c => {
                let mut parts = chars.as_str().split('/');
                let a = parts.next().unwrap();
                let b = parts.next().unwrap();
                let (a, b) = match c {
                    'x' => (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()),
                    'p' => (
                        programs
                            .iter()
                            .position(|c| *c == a.chars().next().unwrap())
                            .unwrap(),
                        programs
                            .iter()
                            .position(|c| *c == b.chars().next().unwrap())
                            .unwrap(),
                    ),
                    _c => panic!("Unexpected char: {}", _c),
                };
                programs.swap(a, b);
            }
        };
    }
}
