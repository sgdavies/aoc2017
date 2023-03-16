use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone)]
enum State {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

pub fn solve() {
    let input = include_str!("../22.txt").trim_end();

    let mut grid = HashSet::<(i32, i32)>::new();
    for (i, line) in input.split('\n').enumerate() {
        let row: i32 = i.try_into().unwrap();
        for (j, c) in line.chars().enumerate() {
            let col: i32 = j.try_into().unwrap();
            if c == '#' {
                grid.insert((row, col));
            }
        }
    }

    let original_grid = grid.clone();

    let mut pos = (12, 12);
    let mut direction = Direction::Up;
    let mut new_infections = 0;
    for _ in 0..10_000 {
        let was_infected = grid.contains(&pos);
        direction = turn(direction, !was_infected);
        if was_infected {
            grid.remove(&pos);
        } else {
            new_infections += 1;
            grid.insert(pos);
        }
        pos = step(pos.0, pos.1, direction);
    }
    println!("22 part one {}", new_infections);

    // Part two
    let mut grid: HashMap<(i32, i32), State> = original_grid
        .iter()
        .map(|p| (*p, State::Infected))
        .collect();
    let mut pos = (12, 12);
    let mut direction = Direction::Up;
    let mut new_infections = 0;
    for _ in 0..10_000_000 {
        let state = *grid.get(&pos).unwrap_or(&State::Clean);
        direction = match state {
            State::Clean => turn(direction, true),
            State::Weakened => direction,
            State::Infected => turn(direction, false),
            State::Flagged => turn(turn(direction, true), true),
        };
        grid.insert(
            pos,
            match state {
                State::Clean => State::Weakened,
                State::Weakened => {
                    new_infections += 1;
                    State::Infected
                }
                State::Infected => State::Flagged,
                State::Flagged => State::Clean,
            },
        );

        pos = step(pos.0, pos.1, direction);
    }
    println!("22 part two {}", new_infections);
}

fn turn(current: Direction, left: bool) -> Direction {
    if left {
        // Turn left
        match current {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    } else {
        // Turn right
        match current {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

fn step(row: i32, col: i32, direction: Direction) -> (i32, i32) {
    match direction {
        Direction::Up => (row - 1, col),
        Direction::Down => (row + 1, col),
        Direction::Left => (row, col - 1),
        Direction::Right => (row, col + 1),
    }
}
