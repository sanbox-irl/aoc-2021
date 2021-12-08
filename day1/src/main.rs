fn main() {
    let txt = include_str!("../input.txt");
    let nums: Vec<usize> = txt.lines().map(|v| v.parse().unwrap()).collect();

    let answer = count_incrementing(&nums);
    println!("{}", answer);

    let answer2 = count_windows(&nums);
    println!("{}", answer2);
}

fn count_incrementing(n: &[usize]) -> usize {
    n.windows(2).filter(|v| v[0] < v[1]).count()
}

fn count_windows(mut nums: &[usize]) -> usize {
    let mut last_value = None;
    let mut count = 0;

    loop {
        if nums.len() < 3 {
            break;
        }

        let window = &nums[..3];
        let value: usize = window.iter().copied().sum();

        if let Some(last_value) = last_value {
            if last_value < value {
                count += 1;
            }
        }

        nums = &nums[1..];
        last_value = Some(value);
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let txt = "199
200
208
210
200
207
240
269
260
263";
        let nums: Vec<usize> = txt.lines().map(|v| v.parse().unwrap()).collect();

        let answer = count_incrementing(&nums);

        assert_eq!(answer, 7);
    }

    #[test]
    fn example_2() {
        let txt = "199
200
208
210
200
207
240
269
260
263";
        let nums: Vec<usize> = txt.lines().map(|v| v.parse().unwrap()).collect();

        let answer = count_windows(&nums);

        assert_eq!(answer, 5);
    }
}
