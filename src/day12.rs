use std::collections::{BTreeMap, HashMap, HashSet};

pub fn solve() {
    // read input from 12.txt and strip the newline
    let input = include_str!("../12.txt").trim();

    let mut merged_groups = BTreeMap::<u32, HashSet<u32>>::new();
    let mut parents = HashMap::<u32, u32>::new();

    // Each time we merge two groups, we need to update the parent of all the
    // children of the merged group to the parent of the other group.
    for line in input.lines() {
        // println!("\t{:?}", merged_groups);
        // println!("\t{:?}", parents);
        // println!("{}", line);
        let mut parts = line.split(" <-> ");
        let id = parts.next().unwrap().parse::<u32>().unwrap();
        let mut group = HashSet::<u32>::new();
        group.insert(id);
        for linked_id in parts.next().unwrap().split(", ") {
            group.insert(linked_id.parse::<u32>().unwrap());
        }
        loop {
            let base_group = group.clone();
            for linked_id in &base_group {
                if let Some(other_parent) = parents.get(linked_id) {
                    if let Some(other_group) = merged_groups.get(other_parent) {
                        for other_id in other_group {
                            group.insert(*other_id);
                        }
                        merged_groups.remove(linked_id);
                    }
                }
            }
            if base_group.len() == group.len() {
                break;
            }
        }

        let min_id = group.iter().min().unwrap();
        for linked_id in &group {
            parents.insert(*linked_id, *min_id);
        }
        merged_groups.insert(*min_id, group);
    }

    // println!("{:?}", merged_groups);
    // println!("{:?}", parents);
    println!("12 part one {}", merged_groups.get(&0).unwrap().len());
    println!("12 part two {}", merged_groups.len());
}
