use std::collections::HashMap;

pub fn solve() {
    let input = include_str!("../20.txt").trim_end();
    // Part one: the long-term answer is whichever has the lowest acceleration
    let manhatten_accels: Vec<i32> = input
        .lines()
        .map(|line| {
            let mut words = line.split("a=<");
            words.next();
            let accel = words.next().unwrap();
            let accel = accel.trim_end_matches('>');
            accel
                .split(',')
                .fold(0, |acc, num| acc + num.parse::<i32>().unwrap().abs())
        })
        .collect();

    let mut min_accel = manhatten_accels.get(0).unwrap();
    let mut closest = 0;
    for particle_id in 1..manhatten_accels.len() {
        let accel = manhatten_accels.get(particle_id).unwrap();
        if accel < min_accel {
            min_accel = accel;
            closest = particle_id;
        }
    }
    println!("20 part one {}", closest);

    // part two - have to simulate it properly!
    let mut particles: Vec<Particle> = input
        .lines()
        .map(|line| {
            let mut segments = line.split(", ");
            let p = seg2nums(segments.next().unwrap());
            let v = seg2nums(segments.next().unwrap());
            let a = seg2nums(segments.next().unwrap());
            let alive = true;
            Particle { p, v, a, alive }
        })
        .collect();
    // println!("There are {} particles", particles.len());

    for _ in 0..100 {
        // TODO - better end condition (if needed)
        let mut new_positions = HashMap::<(i64, i64, i64), u32>::new();

        // One simulation step, also recording number of particles in each new position
        particles = particles
            .iter()
            .map(|p| match p.alive {
                true => {
                    let new_v = (p.v.0 + p.a.0, p.v.1 + p.a.1, p.v.2 + p.a.2);
                    let new_p = (p.p.0 + new_v.0, p.p.1 + new_v.1, p.p.2 + new_v.2);
                    new_positions.insert(new_p, 1 + *new_positions.get(&new_p).unwrap_or(&0));
                    Particle {
                        p: new_p,
                        v: new_v,
                        a: p.a,
                        alive: p.alive,
                    }
                }
                false => *p,
            })
            .collect();

        // Filter step - kill any where there are >1 particle in any position
        particles = particles
            .iter()
            .map(|p| match p.alive {
                true => match new_positions.get(&p.p).unwrap() {
                    1 => *p,
                    _ => Particle {
                        p: p.p,
                        v: p.v,
                        a: p.a,
                        alive: false,
                    },
                },
                false => *p,
            })
            .collect();
    }

    println!(
        "20 part two {}",
        particles.iter().filter(|p| p.alive).count()
    );
}

#[derive(Copy, Clone)]
struct Particle {
    p: (i64, i64, i64),
    v: (i64, i64, i64),
    a: (i64, i64, i64),
    alive: bool,
}

fn seg2nums(segment: &str) -> (i64, i64, i64) {
    let mut chunks = segment.split("=<");
    chunks.next(); // "p=<"
    let temp: Vec<i64> = chunks
        .next()
        .unwrap()
        .trim_end_matches('>')
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    (
        *temp.get(0).unwrap(),
        *temp.get(1).unwrap(),
        *temp.get(2).unwrap(),
    )
}
