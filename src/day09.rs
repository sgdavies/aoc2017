use std::fs;

pub fn solve() {
    let input = fs::read_to_string(&"09.txt").expect("Can't read 09.txt");

    let ans = score(&input);
    println!("{}", ans.0);
    println!("{}", ans.1);
}

fn score(input: &str) -> (u32, u32) {
    let mut part_one = 0;
    let mut part_two = 0;
    let mut depth = 1u32;
    let mut garbage = false;
    let mut skip_next = false;
    for c in input.chars() {
        if skip_next {
            skip_next = false;
        } else if garbage {
            if c == '!' {
                skip_next = true
            } else if c == '>' {
                garbage = false;
            } else {
                part_two += 1;
            }
        } else if c == '!' {
            skip_next = true;
        } else if c == '<' {
            garbage = true;
        } else if c == '{' {
            part_one += depth;
            depth += 1;
        } else if c == '}' {
            depth -= 1;
        }
    }
    (part_one, part_two)
}

#[cfg(test)]
mod tests {
    use crate::day09::score;

    #[test]
    fn examples() {
        assert_eq!(score(&"{}").0, 1);
        assert_eq!(score(&"{{{}}}").0, 6);
        assert_eq!(score(&"{{},{}}").0, 5);
        assert_eq!(score(&"{{{},{},{{}}}}").0, 16);
        assert_eq!(score(&"{<a>,<a>,<a>,<a>}").0, 1);
        assert_eq!(score(&"{{<ab>},{<ab>},{<ab>},{<ab>}}").0, 9);
        assert_eq!(score(&"{{<!!>},{<!!>},{<!!>},{<!!>}}").0, 9);
        assert_eq!(score(&"{{<a!>},{<a!>},{<a!>},{<ab>}}").0, 3);
    }
}
