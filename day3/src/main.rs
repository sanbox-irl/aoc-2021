use std::cmp::Ordering;

fn main() {
    let input = include_str!("../input.txt");
    let (gamma, epsilon) = one::<12>(input);
    println!(
        "Gamma is {}, Epsilon is {}, output is {}",
        gamma,
        epsilon,
        gamma * epsilon
    );

    let (oxy, carb) = two::<12>(input);
    println!("Oxy is {}, Carb is {}, output is {}", oxy, carb, oxy * carb);
}

fn one<const N: usize>(txt: &'static str) -> (usize, usize)
where
    [bool; N]: Default,
    [usize; N]: Default,
{
    let gamma_rate = gamma_rate(parse_input::<N>(txt));
    let epsilon = epsilon_rate(gamma_rate);

    (
        bin_to_usize(gamma_rate.into_iter()),
        bin_to_usize(epsilon.into_iter()),
    )
}

fn two<const N: usize>(txt: &'static str) -> (usize, usize)
where
    [bool; N]: Default,
{
    (
        bin_to_usize(find_number(parse_input::<N>(txt), false).into_iter()),
        bin_to_usize(find_number(parse_input::<N>(txt), true).into_iter()),
    )
}

fn find_number<const N: usize>(input: impl Iterator<Item = [bool; N]>, invert: bool) -> [bool; N] {
    let mut input: Vec<_> = input.collect();

    for i in 0..N {
        let positive_bits = input.iter().filter(|v| v[i]).count();
        let cmp = positive_bits.cmp(&(input.len() - positive_bits));

        let mut target = cmp >= Ordering::Equal;
        if invert {
            target = !target;
        }

        input = input.into_iter().filter(|v| v[i] == target).collect();

        if input.len() == 1 {
            return input[0];
        }
    }

    panic!("couldn't find a valid input!");
}

fn gamma_rate<const N: usize>(input: impl Iterator<Item = [bool; N]>) -> [bool; N]
where
    [bool; N]: Default,
    [usize; N]: Default,
{
    let mut scores: [usize; N] = Default::default();
    let mut len = 0;

    for (i, item) in input.enumerate() {
        for (i, bit) in item.into_iter().enumerate() {
            scores[i] += bit as usize;
        }

        len = i;
    }

    // divide it by 2...
    len /= 2;

    let mut output: [bool; N] = Default::default();

    for (i, bit) in scores.into_iter().map(|bit| bit > len).enumerate() {
        output[i] = bit;
    }

    output
}

fn epsilon_rate<const N: usize>(gamma_rate: [bool; N]) -> [bool; N]
where
    [bool; N]: Default,
{
    let mut output: [bool; N] = Default::default();

    for (i, bit) in gamma_rate.into_iter().enumerate() {
        output[i] = !bit;
    }

    output
}

fn bin_to_usize(input: impl Iterator<Item = bool> + DoubleEndedIterator) -> usize {
    let mut output = 0;
    for (i, input) in input.rev().enumerate() {
        output += input as usize * 2usize.pow(i as u32);
    }

    output
}

fn parse_input<const N: usize>(txt: &'static str) -> impl Iterator<Item = [bool; N]>
where
    [bool; N]: Default,
{
    txt.lines().map(|v| {
        let mut output: [bool; N] = Default::default();

        for (i, chr) in v.chars().enumerate() {
            output[i] = chr == '1';
        }

        output
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex_one() {
        let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

        let (gamma, epsilon) = one::<5>(input);
        assert_eq!(gamma, 22);
        assert_eq!(epsilon, 9);
    }

    #[test]
    fn ex_two() {
        let input = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

        let (one, two) = two::<5>(input);
        assert_eq!(one, 23);
        assert_eq!(two, 10);
    }
}
