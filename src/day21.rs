use std::collections::{HashMap, HashSet};

// Design thoughts:
// data - grid
// operations:
// a- get size of grid
// b- divide into 2x2 or 3x3 squares
// c- lookup in table based on a 2x2 or 3x3 square
// d- join squares: square-by-square into a row, then rows into a grid
// definitions:
// e- parse/from/slashed/strings into 2x2 or 3x3
// f- find all reflections and rotations of these 2x2 and 3x3
// Affordances:
// slash/ed/strings - a, b, c, d, e
// array/vec - a, d, f
// Proposal:
// - use slashed strings everywhere apart from calculating reflections & rotations
// e.g. for a 4x4 square this looks like abcd/efgh/ijkl/mnop

pub fn solve() {
    let input = include_str!("../21.txt").trim_end();
    let rules = get_rules(input);

    let mut grid = ".#./..#/###".to_string();
    let mut size = 3; // Length of each row
    for step in 0..18 {
        if step == 5 {
            println!(
                "21 part one: {}",
                grid.chars().into_iter().filter(|c| *c == '#').count()
            );
        }
        // abcd/efgh/ijkl/mnop
        // breakit() => [ab/ef, cd/gh, ij/mn, kl/op]
        // rules => [ABC/DEF/GHI, JKL/MNO/PQR, STU/VWX/YZ1, 234/567/890]
        // reasseble() => ABCJLK,DEFMNO,GHIPQR,STU234,VWX567,YZ1890
        grid = reassemble(
            breakit(&grid, size)
                .iter()
                .map(|chunk| {
                    if let Some(val) = rules.get(chunk) {
                        val.to_string()
                    } else {
                        // println!("{:?}", rules);
                        panic!("Coulnd't find {} in map", chunk);
                    }
                })
                .collect::<Vec<String>>(),
        )
        .join("/");
        size = if size % 2 == 0 {
            (size / 2) * 3
        } else {
            assert!(size % 3 == 0, "Impossible string! {} {}", grid, size);
            (size / 3) * 4
        };
        // println!("{}\n{}\n", grid, grid.split("/").collect::<Vec<&str>>().join("\n"));
    }

    println!(
        "21 part two: {}",
        grid.chars().into_iter().filter(|c| *c == '#').count()
    );
}

fn get_rules(input: &str) -> HashMap<String, String> {
    let mut rules = HashMap::<String, String>::new();
    for line in input.split('\n') {
        let mut parts = line.split(" => ");
        let key = parts.next().unwrap();
        let val = parts.next().unwrap();
        for version in all_orientations(key) {
            rules.insert(version, val.to_string());
        }
    }
    rules
}

fn all_orientations(chunk: &str) -> HashSet<String> {
    let mut ors = HashSet::<String>::new();
    // Turn into an array
    // Add all rotations
    // Flip, then add all rotations again
    match chunk.len() {
        5 => {
            // 2x2
            let rows: Vec<Vec<char>> = chunk.split('/').map(|s| s.chars().collect()).collect();
            let mut grids: Vec<[[char; 2]; 2]> = Vec::new();
            let mut grid: [[char; 2]; 2] = [[' '; 2]; 2];
            for i in 0..2 {
                for j in 0..2 {
                    grid[i][j] = *rows.get(i).unwrap().get(j).unwrap();
                }
            }
            for _ in 0..4 {
                // ACW
                // 00 01 => 01 11
                // 10 11    00 10
                grids.push(grid);
                let t = grid[0][0];
                grid[0][0] = grid[0][1];
                grid[0][1] = grid[1][1];
                grid[1][1] = grid[1][0];
                grid[1][0] = t;
            }
            grid.reverse();
            for _ in 0..4 {
                grids.push(grid);
                let t = grid[0][0];
                grid[0][0] = grid[0][1];
                grid[0][1] = grid[1][1];
                grid[1][1] = grid[1][0];
                grid[1][0] = t;
            }

            for grid in grids {
                ors.insert(
                    grid.into_iter()
                        .map(|ca| {
                            let mut s = "".to_string();
                            for c in ca {
                                s.push(c);
                            }
                            s
                        })
                        .collect::<Vec<String>>()
                        .join("/"),
                );
            }
        }
        11 => {
            // 3x3
            let rows: Vec<Vec<char>> = chunk.split('/').map(|s| s.chars().collect()).collect();
            let mut grids: Vec<[[char; 3]; 3]> = Vec::new();
            let mut grid: [[char; 3]; 3] = [[' '; 3]; 3];
            for i in 0..3 {
                for j in 0..3 {
                    grid[i][j] = *rows.get(i).unwrap().get(j).unwrap();
                }
            }
            for _ in 0..4 {
                // ACW
                // 00 01 02    02 12 22
                // 10 11 12 => 01 11 21
                // 20 21 22    00 10 20
                grids.push(grid);
                let new_grid = [
                    [grid[0][2], grid[1][2], grid[2][2]],
                    [grid[0][1], grid[1][1], grid[2][1]],
                    [grid[0][0], grid[1][0], grid[2][0]],
                ];
                grid = new_grid;
            }
            grid.reverse();
            for _ in 0..4 {
                grids.push(grid);
                let new_grid = [
                    [grid[0][2], grid[1][2], grid[2][2]],
                    [grid[0][1], grid[1][1], grid[2][1]],
                    [grid[0][0], grid[1][0], grid[2][0]],
                ];
                grid = new_grid;
            }

            for grid in grids {
                let val = grid
                    .into_iter()
                    .map(|ca| {
                        let mut s = "".to_string();
                        for c in ca {
                            s.push(c);
                        }
                        s
                    })
                    .collect::<Vec<String>>()
                    .join("/");
                ors.insert(val);
            }
        }
        _x => {
            panic!("Invalid length {} {}", _x, chunk);
        }
    }
    ors
}

fn breakit(grid: &str, size: usize) -> Vec<String> {
    // Take input like abcd/efgh/ijkl/mnop - 2 chunks of size 2
    // Return vec: [ab/ef, cd/gh, ij/mn, kl/op]
    // or abcdef/ghijkl/mnopqr/stuvwx/yzABCD/EFGHIJ - 3 chunks of size 2
    // => [ab/gh, cd/ij, ef/kl, mn/st, op/uv, qr/wx, yz/EF, AB/GH, CD/IJ]
    // size should always equal the length of each row, and the number of rows
    let rows: Vec<Vec<char>> = grid.split("/").map(|s| s.chars().collect()).collect();
    assert_eq!(size, rows.len());
    assert_eq!(size, rows.get(0).unwrap().len());
    let chunk_size = if size % 2 == 0 { 2 } else { 3 };
    let n_chunks = size / chunk_size;
    let mut chunks = Vec::<String>::new();
    for row in 0..n_chunks {
        for chunk in 0..n_chunks {
            let mut new_chunk = Vec::<String>::new();
            for i in 0..chunk_size {
                let mut cs = String::new();
                for j in 0..chunk_size {
                    cs.push(
                        *rows
                            .get(row * chunk_size + i)
                            .unwrap()
                            .get(chunk * chunk_size + j)
                            .unwrap(),
                    );
                }
                new_chunk.push(cs);
            }
            chunks.push(new_chunk.join("/"));
        }
    }
    chunks
}

fn reassemble(chunks: Vec<String>) -> Vec<String> {
    // In: [ABC/DEF/GHI, JKL/MNO/PQR, STU/VWX/YZ1, 234/567/890]
    // Out: [ABCJLK, STU234, DEFMNO, VWX567, GHIPQR, YZ1890]
    //      ABC     JKL         ABCJLK
    //      DEF     MNO         DEFMNO
    //      GHI     PQR         GHIPQR
    //                      =>  STU234
    //      STU     234         VWX567
    //      VWX     567         YZ1890
    //      YZ1     890
    // or In: [abc/././., def/././., ghi/././., JLK/././., MNO/././., PQR/././., s///, t///, u///]
    // => [abcdefghi, ..., ..., JLKMNOPQR, ..., ..., ssstttuuu, ..., ...]
    let chunks: Vec<Vec<&str>> = chunks.iter().map(|s| s.split("/").collect()).collect();
    let chunk_len = chunks.get(0).unwrap().get(0).unwrap().len();
    assert_eq!(chunk_len, chunks.get(0).unwrap().len());
    let mut out = Vec::<String>::new();
    let nchunks = chunks.len();
    let nrows = int_sqrt(nchunks); // In our example, 2 groups of 2
    for i in 0..nrows {
        for fragment_id in 0..chunk_len {
            let mut s = String::new();
            for j in 0..nrows {
                s.push_str(chunks.get(i * nrows + j).unwrap().get(fragment_id).unwrap());
            }
            out.push(s);
        }
    }

    out
}

fn int_sqrt(x: usize) -> usize {
    for i in 1..x + 1 {
        if i * i == x {
            return i;
        }
    }
    panic!("You didn't give me a square number: {}", x);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_ors() {
        let set = super::all_orientations(".#./..#/###");
        println!(
            "{:?}",
            set.iter().collect::<std::collections::BTreeSet<&String>>()
        );
        assert_eq!(8, set.len());
    }
}
