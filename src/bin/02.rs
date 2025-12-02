advent_of_code::solution!(2);

use nom::IResult;
use nom::Parser;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::multi::separated_list1;

struct Range {
    start: u64,
    end: u64,
}

fn parse_range(input: &str) -> IResult<&str, Range> {
    let (input, start) = complete::u64(input)?;
    let (input, _) = tag("-")(input)?;
    let (input, end) = complete::u64(input)?;
    Ok((input, Range { start, end }))
}

fn parse_ranges(input: &str) -> IResult<&str, Vec<Range>> {
    let (input, ranges) = separated_list1(tag(","), parse_range).parse(input)?;
    Ok((input, ranges))
}

fn print_ranges(ranges: &[Range]) {
    for range in ranges {
        println!("{:>12} - {:>12}", range.start, range.end);
    }
}

fn is_id_invalid(id: u64) -> bool {
    let id_str = id.to_string();
    if id_str.len() % 2 != 0 {
        return false;
    }

    for i in 0..(id_str.len() / 2) {
        if id_str.as_bytes()[i] != id_str.as_bytes()[id_str.len() / 2 + i] {
            return false;
        }
    }

    return true;
}

fn get_invalid_ids(range: &Range) -> Vec<u64> {
    let mut invalid_ids = Vec::new();
    for id in range.start..=range.end {
        if is_id_invalid(id) {
            invalid_ids.push(id);
        }
    }
    invalid_ids
}

pub fn part_one(input: &str) -> Option<u64> {
    let ranges: Vec<_> = parse_ranges(input).ok()?.1;
    print_ranges(&ranges);
    let mut invalid_ids = Vec::new();
    for range in &ranges {
        println!("Processing range: {} - {}", range.start, range.end);
        let ids = get_invalid_ids(range);
        // print ids
        for id in &ids {
            println!("Invalid ID: {}", id);
        }
        invalid_ids.extend(ids);
    }

    Some(invalid_ids.iter().sum())
}

fn is_id_invalid_extensive_rec(id_str: &str, parts: usize) -> bool {
    let len = id_str.len();

    if parts > len {
        return false;
    }

    if len % (parts as usize) != 0 {
        return is_id_invalid_extensive_rec(id_str, parts + 1);
    }

    let sub_part = id_str[0..len / parts].to_string();
    for offset in 1..parts {
        let start = offset * (len / parts);
        let end = start + (len / parts);
        if &id_str[start..end] != sub_part.as_str() {
            return is_id_invalid_extensive_rec(id_str, parts + 1);
        }
    }

    true
}

fn is_id_invalid_extensive(id: u64) -> bool {
    let id_str = id.to_string();
    is_id_invalid_extensive_rec(&id_str, 2)
}

fn get_invalid_ids_extensive(range: &Range) -> Vec<u64> {
    let mut invalid_ids = Vec::new();
    for id in range.start..=range.end {
        if is_id_invalid_extensive(id) {
            invalid_ids.push(id);
        }
    }
    invalid_ids
}

pub fn part_two(input: &str) -> Option<u64> {
    let ranges: Vec<_> = parse_ranges(input).ok()?.1;
    print_ranges(&ranges);
    let mut invalid_ids = Vec::new();
    for range in &ranges {
        println!("Processing range: {} - {}", range.start, range.end);
        let ids = get_invalid_ids_extensive(range);
        // print ids
        for id in &ids {
            println!("Invalid ID: {}", id);
        }
        invalid_ids.extend(ids);
    }

    Some(invalid_ids.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
