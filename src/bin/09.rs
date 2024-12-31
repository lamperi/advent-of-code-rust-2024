advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let disk_map: Vec<u32> = input.trim_end().chars().map(|c| c.to_digit(10)).collect::<Option<Vec<_>>>().unwrap();
    let disk_size: u32 = disk_map.iter().sum();
    let mut files = vec![0; disk_size as usize];
    let mut pos: usize = 0;
    for (i, length) in disk_map.into_iter().enumerate() {
        let file_id = if i % 2 == 0 { (i/2) as i32 } else { -1 };
        let length = length as usize;
        for block in files.iter_mut().skip(pos).take(length) {
            *block = file_id;
        }
        pos += length;
    }
    let len = files.len();
    let mut begin = 0;
    let mut end = len - 1;
    while begin < end {
        while files[begin] != -1 {
            begin += 1;
        }
        while files[end] == -1 {
            end -= 1;
        }
        if begin < end {
            files.swap(begin, end);
        }
    }
    Some(files.into_iter().enumerate()
        .filter(|(_, b)| *b != -1)
        .map(|(a, b)| (a as u64)*(b as u64))
        .sum())
}

struct FSBlock {
    file_id: i32,
    start_position: usize,
    length: usize
}


pub fn part_two(input: &str) -> Option<u64> {
    let disk_map = input.trim_end().chars().map(|c| c.to_digit(10)).collect::<Option<Vec<_>>>().unwrap();
    let mut files = Vec::new();
    let mut free_spaces = Vec::new();
    let mut pos: usize = 0;
    for (i, length) in disk_map.into_iter().enumerate() {
        let file_id = if i % 2 == 0 { (i/2) as i32 } else { -1 };
        let length = length as usize;
        let block = FSBlock{file_id, start_position: pos, length};
        if file_id == -1 {
            free_spaces.push(block);    
        } else {
            files.push(block);
        }
        pos += length;
    }
    for file_to_defrag in files.iter_mut().rev() {
        for free_space in free_spaces.iter_mut() {
            if free_space.start_position > file_to_defrag.start_position {
                break
            }
            if free_space.length >= file_to_defrag.length {
                free_space.length -= file_to_defrag.length;
                file_to_defrag.start_position = free_space.start_position;
                free_space.start_position += file_to_defrag.length;
                break
            }
        }
    }
    
    Some(files.into_iter().map(|file| {
        ((file.length * (file.start_position + file.start_position + file.length - 1)/2) as u64) * (file.file_id as u64)
    }).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
