use std::collections::HashMap;


advent_of_code::solution!(21);


// This includes the optimal path from one button to another. It's 11x11 matrix,
// with each row denoting start from 0..9 and then A, and each column
// denoting end from 0..9 and A.
const NUMPAD: [&str; 121] = [
    "A", "^<A", "^A", "^>A", "^^<A", "^^A", "^^>A", "^^^<A", "^^^A", "^^^>A", ">A",
    ">vA", "A", ">A", ">>A", "^A", "^>A", "^>>A", "^^A", "^^>A", "^^>>A", ">>vA",
    "vA", "<A", "A", ">A", "<^A", "^A", "^>A", "<^^A", "^^A", "^^>A", "v>A",
    "<vA", "<<A", "<A", "A", "<<^A", "<^A", "^A", "<<^^A", "<^^A", "^^A", "vA",
    ">vvA", "vA", "v>A", "v>>A", "A", ">A", ">>A", "^A", "^>A", "^>>A", ">>vvA",
    "vvA", "<vA", "vA", "v>A", "<A", "A", ">A", "<^A", "^A", "^>A", "vv>A",
    "<vvA", "<<vA", "<vA", "vA", "<<A", "<A", "A", "<<^A", "<^A", "^A", "vvA",
    ">vvvA", "vvA", "vv>A", "vv>>A", "vA", "v>A", "v>>A", "A", ">A", ">>A", ">>vvvA",
    "vvvA", "<vvA", "vvA", "vv>A", "<vA", "vA", "v>A", "<A", "A", ">A", "vvv>A",
    "<vvvA", "<<vvA", "<vvA", "vvA", "<<vA", "<vA", "vA", "<<A", "<A", "A", "vvvA",
    "<A", "^<<A", "<^A", "^A", "^^<<A", "<^^A", "^^A", "^^^<<A", "<^^^A", "^^^A", "A"
];
// Optimal path from one button to another on arrow pad. It's a 5x5 matrix,
// with each row denoting start position <, >, A, ^, v
// and each column denoting end position <, >, A, ^, v.
const KEYPAD: [&str; 25] = [
    "A", ">>A", ">>^A", ">^A", ">A",
    "<<A", "A", "^A", "<^A", "<A",
    "v<<A", "vA", "A", "<A", "<vA",
    "v<A", "v>A", ">A", "A", "vA",
    "<A", ">A", "^>A", "^A", "A"
];

fn shortest_path(start: char, end: char) -> &'static str {
    if start == 'A' && end == 'A' {
        return "A"
    } else if '0' <= start && start <= '9' || '0' <= end && end <= '9' {
        let y_index: u8 = if start != 'A' { start as u8 - b'0' } else { 10 };
        let x_index: u8 = if end != 'A' { end as u8 - b'0' } else { 10 };
        return NUMPAD[y_index as usize * 11 + x_index as usize];
    } else if "<^v>".contains(start) || "<^v>".contains(end) {
        let y_index = match start {
            '<' => 0,
            '>' => 1,
            'A' => 2,
            '^' => 3,
            'v' => 4,
            _ => { unreachable!(); }
        };
        let x_index = match end {
            '<' => 0,
            '>' => 1,
            'A' => 2,
            '^' => 3,
            'v' => 4,
            _ => { unreachable!(); }
        };
        return KEYPAD[y_index as usize * 5 + x_index as usize];
    } else {
        unreachable!()
    }
}

type Args<'a> = (&'a str, i32, bool);
type Cache<'a> =  HashMap<Args<'a>, u64>;
fn solve_robot<'a>(code: &'a str, arrow_robots: i32, is_numpad: bool, mut cache: &mut Cache<'a>) -> u64 {
    if let Some(v) = cache.get(&(code, arrow_robots, is_numpad)) {
        return *v
    }
    if arrow_robots == 0 {
        return code.len() as u64
    }
    let mut state = 'A';
    let mut min_path = 0;
    for point in code.chars() {
        let path = shortest_path(state, point);
        min_path += solve_robot(path, arrow_robots - 1, false, &mut cache);
        state = point;   
    }
    cache.insert((code, arrow_robots, is_numpad), min_path);
    min_path
}

fn solve_code(code: &str, robots: i32) -> u64 {
    let mut cache: Cache = HashMap::new();
    let seq_length = solve_robot(code, robots + 1, true, &mut cache);
    let code_num: u64 = code.trim_start_matches('0').trim_end_matches('A').parse().unwrap();
    seq_length * code_num
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(input.lines()
        .map(|code| solve_code(code, 2))
        .sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(input.lines()
        .map(|code| solve_code(code, 25))
        .sum())
}

#[cfg(test)]
mod tests {
    macro_rules! tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            #[allow(non_snake_case)]
            fn $name() {
                let (input, expected) = $value;
                assert_eq!(expected, solve_code(input, 2));
            }
        )*
        }
    }
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }

    tests! {
        test_029A: ("029A", 68 * 29),
        test_980A: ("980A", 60 * 980),
        test_179A: ("179A", 68 * 179),
        test_456A: ("456A", 64 * 456),
        test_379A: ("379A", 64 * 379),
    }
}
