advent_of_code::solution!(1);

use nom::IResult;

// L10 = -10
// R90 = 90
fn rotation(input: &str) -> IResult<&str, i64> {
    let (input, dir) = nom::character::complete::one_of("LR")(input)?;
    let (input, count) = nom::character::complete::i64(input)?;
    let rot = match dir {
        'L' => -count,
        'R' => count,
        _ => unreachable!(),
    };
    Ok((input, rot))
}

fn parse_rotations(input: &str) -> Vec<i64> {
    input
        .lines()
        .filter_map(|line| {
            let (_, rot) = rotation(line).ok()?;
            Some(rot)
        })
        .collect()
}

fn wrap(count: i64) -> i64 {
    let count = (count % 100 + 100) % 100;
    count
}

fn running_sum(rotations: &[i64]) -> Vec<i64> {
    let mut sum = 50;
    rotations
        .iter()
        .map(|&r| {
            sum = wrap(sum + r);
            sum
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let rotations: Vec<_> = parse_rotations(input);
    println!("{:?}", rotations);
    let sums = running_sum(&rotations);
    println!("{:?}", sums);
    let zero_counts = sums.iter().filter(|&&x| x == 0).count();
    Some(zero_counts as u64)
}

fn rotate(state: i64, rotation_count: i64) -> (i64, u64) {
    let new_state = state + rotation_count;

    let mut zero_crossings = new_state.abs() as u64 / 100;
    if new_state <= 0 && state != 0 {
        zero_crossings += 1;
    }
    (wrap(new_state), zero_crossings)
}

fn counting_running_sum(rotations: &[i64]) -> Vec<(i64, u64)> {
    let mut state = 50;
    rotations
        .iter()
        .map(|&r| {
            let (new_state, zero_crossings) = rotate(state, r);
            state = new_state;
            (new_state, zero_crossings)
        })
        .collect()
}

pub fn part_two(input: &str) -> Option<u64> {
    let rotations: Vec<_> = parse_rotations(input);
    let sums = counting_running_sum(&rotations);

    // print zipped arrays
    for (r, (s, zc)) in rotations.iter().zip(sums.iter()) {
        println!(
            "Rotation: {:>4} => Sum: {:>4}, Zero Crossings: {}",
            r, s, zc
        );
    }

    let zero_counts = sums.iter().map(|&(_, zc)| zc).sum::<u64>();
    Some(zero_counts as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
