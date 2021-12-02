use glam::IVec2;

fn main() {
    let input = include_str!("../input.txt");
    let input = parse_commands(input);

    let v = part1(input.clone());
    println!("x is {}, y is {}, output is {}", v.x, v.y, v.x * v.y);

    let v = part2(input.clone());
    println!("x is {}, y is {}, output is {}", v.x, v.y, v.x * v.y);
}

fn part1(input: impl Iterator<Item = IVec2>) -> IVec2 {
    input.fold(IVec2::default(), |a, v| a + v)
}

fn part2(input: impl Iterator<Item = IVec2>) -> IVec2 {
    let mut a = 0;
    let mut p = IVec2::ZERO;

    for i in input {
        // when i.x == 0, these are no-ops
        p.x += i.x;
        p.y += a * i.x;

        // when i.y == 0, this is a no-op
        a += i.y;
    }

    p
}

fn parse_commands(c: &str) -> impl Iterator<Item = IVec2> + '_ + Clone {
    c.lines().map(|l| {
        let (w, a) = l.split_once(' ').unwrap();
        (match w {
            "down" => IVec2::new(0, 1),
            "up" => IVec2::new(0, -1),
            _ => IVec2::new(1, 0),
        }) * a.parse::<i32>().unwrap()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

        let v = part1(parse_commands(input));
        assert_eq!(v.x, 15);
        assert_eq!(v.y, 10);
    }

    #[test]
    fn ex2() {
        let input = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

        let v = part2(parse_commands(input));
        assert_eq!(v.x, 15);
        assert_eq!(v.y, 60);
    }
}
