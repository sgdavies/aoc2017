use crate::utils::p2s;
use std::collections::HashMap;

#[derive(Debug)]
enum Tile {
    Continue,
    Turn,
    Letter(char),
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn step(pos: (i32, i32), direction: &Direction) -> (i32, i32) {
    let row = pos.0;
    let col = pos.1;
    match direction {
        Direction::Up => (row - 1, col),
        Direction::Down => (row + 1, col),
        Direction::Left => (row, col - 1),
        Direction::Right => (row, col + 1),
    }
}

pub fn solve() {
    let input = include_str!("../19.txt").trim_end();
    let mut grid = HashMap::<(i32, i32), Tile>::new();
    for (row, line) in input.split('\n').enumerate() {
        let row: i32 = row.try_into().unwrap();
        for (col, c) in line.chars().enumerate() {
            let col: i32 = col.try_into().unwrap();
            match c {
                ' ' => {}
                '|' => {
                    grid.insert((row, col), Tile::Continue);
                }
                '-' => {
                    grid.insert((row, col), Tile::Continue);
                }
                '+' => {
                    grid.insert((row, col), Tile::Turn);
                }
                letter => {
                    grid.insert((row, col), Tile::Letter(letter));
                }
            }
        }
    }

    let grid = grid; // No longer mut
    let mut col = 0;
    loop {
        // Find start position
        if grid.contains_key(&(0, col)) {
            break;
        }
        col += 1;
    }

    let mut pos = (0, col);
    let mut direction = Direction::Down;
    let mut word = Vec::<char>::new();
    let mut steps = 0;
    loop {
        steps += 1; // pre-emptive
        match grid.get(&pos) {
            Some(tile) => {
                pos = match tile {
                    Tile::Continue => step(pos, &direction),
                    Tile::Letter(a) => {
                        word.push(*a);
                        step(pos, &direction)
                    }
                    Tile::Turn => {
                        match direction {
                            Direction::Up | Direction::Down => {
                                let left = (pos.0, pos.1 - 1);
                                let right = (pos.0, pos.1 + 1);
                                if let Some(_tile) = grid.get(&left) {
                                    direction = Direction::Left;
                                    left
                                } else if let Some(_tile) = grid.get(&right) {
                                    direction = Direction::Right;
                                    right
                                } else {
                                    (-1, -1) // Hack - will drop out in next loop
                                }
                            }
                            Direction::Left | Direction::Right => {
                                let up = (pos.0 - 1, pos.1);
                                let down = (pos.0 + 1, pos.1);
                                if let Some(_tile) = grid.get(&up) {
                                    direction = Direction::Up;
                                    up
                                } else if let Some(_tile) = grid.get(&down) {
                                    direction = Direction::Down;
                                    down
                                } else {
                                    (-1, -1) // Hack - will drop out in next loop
                                }
                            }
                        }
                    }
                }
            }
            None => {
                steps -= 1; // The final step was into empty space - ignore it
                break;
            } // End of the line
        }
    }

    println!("19 part one {}", p2s(&word));
    println!("19 part two {}", steps);
}
