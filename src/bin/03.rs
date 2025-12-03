use std::vec;

advent_of_code::solution!(3);

fn get_battery_joltage_indices(bank: &Vec<u8>, count: usize) -> Vec<usize> {
    let mut joltage_indices: Vec<usize> = vec![];
    let mut last_index: Option<usize> = None;
    for digit_index in 0..count {
        let start_index = if let Some(index) = last_index {
            index + 1
        } else {
            0
        };

        let mut battery_index = start_index;

        for (index, battery) in bank
            .iter()
            .enumerate()
            .take(bank.len() - count + digit_index + 1)
            .skip(start_index)
        {
            if battery > &bank[battery_index] {
                battery_index = index;
            }
        }
        joltage_indices.push(battery_index);
        last_index = Some(battery_index);
    }
    joltage_indices
}

pub fn display_pack_colorized(bank: &Vec<u8>, joltage_indices: &Vec<usize>) -> String {
    let mut output = String::new();
    for (index, &digit) in bank.iter().enumerate() {
        if joltage_indices.contains(&index) {
            output.push_str(&format!("\x1b[1;31m{}\x1b[0m", digit));
        } else {
            output.push_str(&format!("{}", digit));
        }
    }
    output
}

pub fn get_joltage_value(bank: &Vec<u8>, joltage_indices: &Vec<usize>) -> u64 {
    let mut value = 0;
    for &index in joltage_indices {
        value = value * 10 + bank[index] as u64;
    }
    value
}

pub fn get_total_output_joltage(banks: &Vec<Vec<u8>>, count: usize) -> u64 {
    let battery_joltages_indices: Vec<Vec<usize>> = banks
        .clone()
        .into_iter()
        .map(|bank| get_battery_joltage_indices(&bank, count))
        .collect();

    for (bank, joltage_indices) in banks
        .clone()
        .into_iter()
        .zip(battery_joltages_indices.iter())
    {
        println!(
            "Battery Bank: {}, Joltage: {} (indices: {:?})",
            display_pack_colorized(&bank, joltage_indices),
            get_joltage_value(&bank, joltage_indices),
            joltage_indices
        );
    }

    banks
        .into_iter()
        .zip(battery_joltages_indices.iter())
        .map(|(bank, joltage)| get_joltage_value(&bank, joltage))
        .sum::<u64>()
        .into()
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap_or(0) as u8)
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>()
}

pub fn part_one(input: &str) -> Option<u64> {
    let battery_banks = parse_input(input);
    Some(get_total_output_joltage(&battery_banks, 2))
}

pub fn part_two(input: &str) -> Option<u64> {
    let battery_banks = parse_input(input);
    Some(get_total_output_joltage(&battery_banks, 12))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
