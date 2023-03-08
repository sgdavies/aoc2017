use std::cmp::max;

pub fn solve() {
    // Read the input string from the file, trim off the newline and call count_steps to get the result
    let input = include_str!("../11.txt").trim();
    let result = count_steps(input);
    println!("Day 11, part 1: {}", result.0);
    println!("Day 11, part 2: {}", result.1);
}

fn count_steps(path: &str) -> (u32, u32) {
    // Crossing the bridge, you've barely reached the other side of the stream when a program comes up to you, clearly in distress. "It's my child process," she says, "he's gotten lost in an infinite grid!"
    // Fortunately for her, you have plenty of experience with infinite grids.
    // Unfortunately for you, it's a hex grid.
    // The hexagons ("hexes") in this grid are aligned such that adjacent hexes can be found to the north, northeast, southeast, south, southwest, and northwest:
    //   \ n  /
    // nw +--+ ne
    //   /    \
    // -+      +-
    //   \    /
    // sw +--+ se
    //   / s  \
    // You have the path the child process took. Starting where he started, you need to determine the fewest number of steps required to reach him. (A "step" means to move from the hex you are in to any adjacent hex.)
    // For example:
    // ne,ne,ne is 3 steps away.
    // ne,ne,sw,sw is 0 steps away (back where you started).
    // ne,ne,s,s is 2 steps away (se,se).
    // se,sw,se,sw,sw is 3 steps away (s,s,sw).

    // Model as a grid of hexagons, with the origin at the center (0,0).  Even columns have even y values, odd columns have odd y values.
    let mut x = 0;
    let mut y = 0;
    let mut max_distance = 0;
    for token in path.split(',') {
        match token {
            "n" => y += 2,
            "ne" => {
                x += 1;
                y += 1;
            },
            "se" => {
                x += 1;
                y -= 1;
            },
            "s" => y -= 2,
            "sw" => {
                x -= 1;
                y -= 1;
            },
            "nw" => {
                x -= 1;
                y += 1;
            },
            _ => panic!("Invalid token: {}", token),
        }

        let distance = hex_distance(x, y);
        if distance > max_distance {
            max_distance = distance;
        }
    }

    (hex_distance(x, y), max_distance)
}

fn hex_distance(x: i32, y: i32) -> u32 {
    // Return the distance from the origin to the hex at (x,y)
    let dx = x.abs();
    let dy = y.abs();
    (dx + max(0, (dy - dx) / 2)) as u32
}

#[cfg(test)]
mod tests {
    use super::count_steps;

    #[test]
    fn test_part_one() {
        assert_eq!(count_steps(&"ne,ne,ne").0, 3);
        assert_eq!(count_steps(&"ne,ne,sw,sw").0, 0);
        assert_eq!(count_steps(&"ne,ne,s,s").0, 2);
        assert_eq!(count_steps(&"se,sw,se,sw,sw").0, 3);
    }
}