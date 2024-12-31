use std::collections::{HashMap, HashSet};
use std::cmp::Ordering;

advent_of_code::solution!(5);


fn parse(input: &str) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let (rules, updates) = input.split_once("\n\n").unwrap();
    let rules: Option<Vec<(u32, u32)>> = rules.lines().map(|line| {
        line.split_once('|').and_then(|(left, right)| {
            match (left.parse::<u32>().ok(), right.parse::<u32>().ok()) {
                (Some(a), Some(b)) => Some((a,b)),
                _ => None
            }
        })
    }).collect();
    let rules = rules.expect("unable to parse rules");
    let updates: Option<Vec<Vec<u32>>> = updates.lines().map(|line| {
        line.split(',').map(|s| s.parse::<u32>().ok()).collect()
    }).collect();
    let updates = updates.expect("unable to parse updates");
    (rules, updates)
}

fn is_valid(update: &&Vec<u32>, rules: &[(u32, u32)]) -> bool {
    update.iter().enumerate().all(|(index, val)| {
        rules.iter()
            .filter(|(before,_)| {
                before == val
            })
            .all(|(_, after)| {
                !update[..index].iter().any(|v| v == after)
            })
    })
}

fn middle(update: &Vec<u32>) -> u32 {
    update[update.len() / 2]
}

fn sort_update(update: &[u32], rules: &[(u32, u32)]) -> Vec<u32> {
    let my_nums: HashSet<u32> = HashSet::from_iter(update.iter().copied());
    let mut total_order: HashMap<u32, HashSet<u32>> = HashMap::new();
    for (a, b) in rules.iter().copied() {
        if my_nums.contains(&a) && my_nums.contains(&b) {
            total_order.entry(a).or_default().insert(b);
            total_order.entry(b).or_default();
        }
    }
    for _i in 0..my_nums.len() {
        let mut new_total_order: HashMap<u32, HashSet<u32>> = HashMap::new();

        for (&k, v) in total_order.iter() {
            let copied_keys: Vec<_> = v.iter().copied().collect();
            for val in copied_keys {
                let one_descendant = total_order.get(&val).unwrap();
                for &desc in one_descendant.iter() {
                    new_total_order.entry(k).or_default().insert(desc);
                }
            }
        }
        for (&k, v) in total_order.iter_mut() {
            v.extend(new_total_order.entry(k).or_default().iter());
        }
    }

    let mut new = update.to_vec();
    new.sort_by(|a, b| -> Ordering {
        if total_order.get(a).unwrap().contains(b) {
            Ordering::Less
        } else if total_order.get(b).unwrap().contains(a) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });
    new
}

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, updates) = parse(input);
    Some(updates.iter()
        .filter(|v| is_valid(v, &rules))
        .map(middle)
        .sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, updates) = parse(input);
    Some(updates.iter()
        .filter(|v| !is_valid(v, &rules))
        .map(|v| sort_update(v, &rules))
        .map(|v| middle(&v))
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
