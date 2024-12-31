advent_of_code::solution!(22);

const SECRET_MASK: u32 = (1 << 24) - 1;

fn next_secret(mut number: u32) -> u32 {
    number ^= number << 6;
    number &= SECRET_MASK;
    number ^= number >> 5;
    number ^= number << 11;
    number & SECRET_MASK
}

struct SecretNumberGenerator {
    secret_number: u32,
}

impl SecretNumberGenerator {
    fn new(secret_number: u32) -> Self {
        SecretNumberGenerator{secret_number}
    }
}

impl Iterator for SecretNumberGenerator {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.secret_number = next_secret(self.secret_number);
        Some(self.secret_number)
    }
}


pub fn part_one(input: &str) -> Option<u64> {
    let secret_numbers = input.lines()
        .map(|n| n.parse::<u32>())
        .collect::<Result<Vec<u32>, _>>().unwrap();
    Some(secret_numbers.iter().map(|n| {
        SecretNumberGenerator::new(*n)
            .take(2000).last().unwrap() as u64
    }).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let secret_numbers = input.lines()
        .map(|n| n.parse::<u32>())
        .collect::<Result<Vec<u32>, _>>().unwrap();

    // Has this index been seen already for this number?
    let mut seen = [0; 19*19*19*19];
    // How much score has this number accumulated?
    let mut scores = [0; 19*19*19*19];
    // Max score across all scores. Updated in place
    // to avoid looping through all scores at the end.
    let mut max_score = 0;
    for (nth, sn) in secret_numbers.iter().enumerate() {
        let nth = nth + 1;
        SecretNumberGenerator::new(*sn)
            .take(2000)
            .map(|price| price.rem_euclid(10) as i8)
            .collect::<Vec<_>>()
            .windows(2)
            .map(|s| {
                let [a, b] = s.try_into().unwrap();
                // Pass on price and the diff between two prices.
                (b, b - a)
            })
            .collect::<Vec<_>>()
            .windows(4)
            .map(|s| {
                let [(_,a),(_,b),(_,c),(price,d)] = s.try_into().unwrap();
                let price = price as u32;
                let a = (a+9) as usize;
                let b = (b+9) as usize;
                let c = (c+9) as usize;
                let d = (d+9) as usize;
                // Calculate unique index for four numbers ranging
                // from -9 to 9.
                let index = d + 19 * c + 19*19 * b + 19*19*19 * a;
                (price, index)
            }).for_each(|(price, index)| {
                if seen[index] < nth {
                    seen[index] = nth;
                    scores[index] += price;
                    if scores[index] > max_score {
                        max_score = scores[index]
                    }
                }
            });
    }
    Some(max_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secret_numbers() {
        let mut number = 123;
        for next_expected in [
            15887950,
            16495136,
            527345,
            704524,
            1553684,
            12683156,
            11100544,
            12249484,
            7753432,
            5908254
        ] {
            number = next_secret(number);
            assert_eq!(number, next_expected);    
        }
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(23));
    }
}
