pub fn solve() {
    let a_factor = 16807u64;
    let b_factor = 48271u64;
    let mask = 0b1111_1111_1111_1111;

    let mut a = 591u64;
    let mut b = 393u64;
    let mut judge = 0u32;

    for _ in 0..40_000_000 {
        a = (a * a_factor) % 2147483647;
        b = (b * b_factor) % 2147483647;

        if (a & mask) == (b & mask) {
            judge += 1;
        }
    }
    println!("Day 15 part one: {}", judge);

    a = 591;
    b = 393;
    judge = 0;

    for _ in 0..5_000_000 {
        // hack for do..while (!)
        while {
            a = (a * a_factor) % 2147483647;
            a % 4 > 0
        } {}
        while {
            b = (b * b_factor) % 2147483647;
            b % 8 > 0
        } {}

        if (a & mask) == (b & mask) {
            judge += 1;
        }
    }
    println!("Day 15 part two: {}", judge);
}
