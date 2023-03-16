use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

#[derive(Debug)]
struct Disc {
    _name: String,
    size: u32,
    children: Option<Vec<String>>,
}

impl Disc {
    fn find_unbalanced(&self, discs: &HashMap<String, Disc>) -> Two {
        // Search recursively, and stop as soon as we have the answer
        if let Some(children) = &self.children {
            let mut sizes: Vec<(u32, u32)> = Vec::new(); // (Total weight of disc & children, disc size)
            for child in children.iter() {
                let child = discs.get(child).expect("Missing child");
                match child.find_unbalanced(discs) {
                    Two::Balanced(weight) => sizes.push((weight, child.size)),
                    Two::Unbalanced(answer) => return Two::Unbalanced(answer),
                }
            }
            sizes.sort_unstable();
            if sizes[0].0 != sizes.last().unwrap().0 {
                // Unbalanced! Which is the odd one out?
                // println!("{:?}", sizes);
                assert!(sizes.len() > 2);
                let diff = sizes.last().unwrap().0 - sizes[0].0;
                if sizes[0].0 == sizes[1].0 {
                    // The last element is too large.  Need to reduce the holder's size.
                    Two::Unbalanced(sizes.last().unwrap().1 - diff)
                } else {
                    // The first element is too small.  Need to increase its size.
                    Two::Unbalanced(sizes[0].1 + diff)
                }
            } else {
                Two::Balanced(self.size + sizes.iter().map(|ab| ab.0).sum::<u32>())
            }
        } else {
            Two::Balanced(self.size)
        }
    }
}

enum Two {
    Balanced(u32),   // Total weight of this disc and everything on it
    Unbalanced(u32), // The answer
}

pub fn solve() {
    let input = fs::read_to_string(&"07.txt").expect("Can't read 07.txt");
    let mut kids = HashSet::new();
    let discs: HashMap<String, Disc> = input
        .trim_end()
        .split('\n')
        .map(|s| {
            let mut words: VecDeque<&str> = s.trim_end().split_whitespace().collect();
            let name = words.pop_front().expect("No name");
            let size: u32 = words
                .pop_front()
                .expect("No size")
                .replace("(", "")
                .replace(")", "")
                .parse()
                .expect("Couldn't parse size");
            let children: Option<Vec<String>> = if !words.is_empty() {
                words.pop_front(); // ->
                let childs: Vec<String> = words.into_iter().map(|w| w.replace(",", "")).collect();
                kids.extend(childs.iter().map(|c| c.to_owned()));
                Some(childs)
            } else {
                None
            };
            (
                name.to_string(),
                Disc {
                    _name: name.to_string(),
                    size,
                    children,
                },
            )
        })
        .collect();

    let root = discs
        .keys()
        .filter(|k| !kids.contains(*k))
        .collect::<Vec<&String>>()
        .pop()
        .expect("No parent?");
    println!("{}", root);

    let part_two = discs.get(root).expect("No root").find_unbalanced(&discs);
    match part_two {
        Two::Balanced(_) => panic!("Tower is balanced - no answer"),
        Two::Unbalanced(ans) => println!("{}", ans),
    };
}
