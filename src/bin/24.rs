use std::{collections::{HashMap, HashSet, VecDeque}, fmt, str::FromStr};

advent_of_code::solution!(24);

#[derive(Copy, Clone, Debug)]
enum Op {
    OR,
    AND,
    XOR
}
#[derive(Debug)]
struct InvalidOp;

impl Op {
    fn apply(&self, left: bool, right: bool) -> bool {
        match self {
            Self::OR => left | right,
            Self::AND => left & right,
            Self::XOR => left ^ right
        }
    }
}

#[derive(Debug)]
struct Gate<'a> {
    left: &'a str,
    right: &'a str,
    output: &'a str,
    op: Op
}


impl FromStr for Op {
    type Err = InvalidOp;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "OR"  => Ok(Op::OR),
            "AND" => Ok(Op::AND),
            "XOR" => Ok(Op::XOR),
            _ => Err(InvalidOp),
        }
    }
}

fn parse(input: &str) -> (HashMap<&str, bool>, Vec<Gate<'_>>){
    let (inputs, gates)  = input.split_once("\n\n").unwrap();
    let inputs = inputs.lines().map(|line| {
        let (reg, val) = line.split_once(": ").unwrap();
        (reg, val.parse::<u8>().unwrap() != 0)
    }).collect::<HashMap<_, _>>();
    let gates = gates.lines().map(|line| {
        let mut iter = line.split_ascii_whitespace();
        let left = iter.next().unwrap();
        let op: &str = iter.next().unwrap();
        let right: &str = iter.next().unwrap();
        iter.next().unwrap();
        let output = iter.next().unwrap();
        Gate{
            left,
            right,
            output,
            op: op.parse().unwrap()
        }
    }).collect::<Vec<_>>();
    (inputs, gates)
}

fn run<'a>(mut inputs: HashMap<&'a str, bool>, gates: &Vec<Gate<'a>>) -> HashMap<&'a str, bool>{
    let mut by_input: HashMap<&str, Vec<&Gate<'_>>> = HashMap::new();
    for gate in gates {
        by_input.entry(gate.left)
            .or_insert_with(|| Vec::new())
            .push(gate);
        by_input.entry(gate.right)
            .or_insert_with(|| Vec::new())
            .push(gate);
    }
    let mut outputs = Vec::new();
    let mut queue = VecDeque::from_iter(inputs.keys().copied());
    while let Some(input) = queue.pop_front() {
        match by_input.get_mut(input) {
            Some(gates) => {
                gates.retain(|gate| {
                    let left_bit = inputs.get(gate.left);
                    let right_bit = inputs.get(gate.right);
                    if left_bit.is_some() && right_bit.is_some() {
                        let out_bit = gate.op.apply(*left_bit.unwrap(), *right_bit.unwrap());
                        inputs.insert(gate.output, out_bit);
                        queue.push_back(gate.output);
                        return false
                    }
                    true
                });
                if !gates.is_empty() {
                    queue.push_back(input);
                }
            },
            None => {
                outputs.push(input);
            }
        }
    }
    outputs.iter()
        .map(|output| (*output, *inputs.get(output).unwrap()))
        .collect::<HashMap<&'a str, bool>>()
}

fn output_to_integer(outputs: HashMap<&str, bool>) -> u64 {
    let mut output = 0;
    for z in 0..outputs.len()  {
        let key = fmt::format(format_args!("z{:0>2}", z));
        let bit = (*outputs.get(&*key).unwrap() as u64) << z;
        output |= bit;
    }
    output
}

pub fn part_one(input: &str) -> Option<u64> {
    let (inputs, gates) = parse(input);
    let outputs: HashMap<&str, bool> = run(inputs, &gates);
    Some(output_to_integer(outputs))
}

pub fn part_two(input: &str) -> Option<String> {
    let (_inputs, gates) = parse(input);

    let by_output = gates.iter().map(|gate| {
        (gate.output, gate)
    }).collect::<HashMap<_, _>>();

    let mut found_bad_gates = HashSet::new();
    for gate in by_output.values() {
        let left = by_output.get(gate.left);
        let right = by_output.get(gate.right);
        let left_type = left.map(|g| g.op);
        let right_type = right.map(|g| g.op);
        match (gate.op, left_type, right_type) {
            // Gate which exists right after inputs.
            (Op::AND, None, None) | (Op::XOR, None, None) => {
                assert!(gate.left.starts_with("x") || gate.left.starts_with("y"));
                if gate.output.starts_with("z") && gate.output != "z00" {
                    found_bad_gates.insert(gate.output);
                }
            }
            // This should only exist for z01.
            (Op::XOR, Some(Op::AND), Some(Op::XOR)) |
            (Op::XOR, Some(Op::XOR), Some(Op::AND)) => {
                assert_eq!(gate.output, "z01");
            }
            (Op::AND, Some(Op::XOR), Some(Op::OR)) |
            (Op::AND, Some(Op::OR), Some(Op::XOR)) => {
                // This gate comes 41 times.
                if gate.output.starts_with("z") {
                    found_bad_gates.insert(gate.output);
                }
            }
            // This should only exist after first input.
            (Op::AND, Some(Op::XOR), Some(Op::AND)) => {
                let parent: &&Gate<'_> = &by_output[gate.right];
                assert!(parent.left == "x00" || parent.left == "y00");
                assert!(parent.right == "x00" || parent.right == "y00");
            }
            (Op::AND, Some(Op::AND), Some(Op::XOR)) => {
                let parent: &&Gate<'_> = &by_output[gate.left];
                assert!(parent.left == "x00" || parent.left == "y00");
                assert!(parent.right == "x00" || parent.right == "y00");
            }
            (Op::AND, Some(_), Some(Op::OR)) => {
                found_bad_gates.insert(gate.left);
            }
            (Op::AND, Some(Op::OR), Some(_)) => {
                found_bad_gates.insert(gate.right);
            }
            (Op::AND, Some(Op::XOR), Some(Op::XOR)) |
            (Op::AND, Some(Op::AND), Some(Op::AND)) => {
                // Bad gate! But cannot know which parent is bad.
            }
            (Op::OR, Some(Op::AND), Some(Op::AND)) => {
                // This gate comes 41 times.
                if gate.output.starts_with("z") && gate.output != "z45" {
                    found_bad_gates.insert(gate.output);
                }
            }
            (Op::OR, Some(Op::AND), Some(_)) => {
                found_bad_gates.insert(gate.right);
            }
            (Op::OR, Some(_), Some(Op::AND)) => {
                found_bad_gates.insert(gate.left);
            }
            (Op::XOR, Some(Op::OR), Some(Op::XOR)) |
            (Op::XOR, Some(Op::XOR), Some(Op::OR)) => {
                // This gate comes 41 times.
                if !gate.output.starts_with("z") {
                    found_bad_gates.insert(gate.output);
                }
            }
            (Op::XOR, Some(Op::XOR), Some(Op::XOR)) |
            (Op::XOR, Some(Op::OR), Some(Op::OR))
            => {
                // One parent is bad but cannot know which one.
            }
            (Op::XOR, Some(Op::OR), Some(_)) => {
                found_bad_gates.insert(gate.right);
            }
            (Op::XOR, Some(_), Some(Op::OR)) => {
                found_bad_gates.insert(gate.left);
            }
            _ => {
                panic!("node {} of type {:?} has left parent {} of {:?} and right parent {} of {:?}", gate.output, gate.op, gate.left, left_type, gate.right, right_type);
            }
        }
    }
    let mut bad_gates = Vec::from_iter(found_bad_gates);
    bad_gates.sort();
    Some(bad_gates.join(","))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_one_two() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(2024));
    }
}
