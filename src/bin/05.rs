advent_of_code::solution!(5);

#[derive(Clone)]
struct Range {
    start: u64,
    end: u64,
}

struct Database {
    fresh_ingredient_ranges: Vec<Range>,
    ingredient_ids: Vec<u64>,
}

enum ParserState {
    FreshIngredients,
    IngredientIDs,
}

fn parse_database(input: &str) -> Database {
    let lines = input.lines().collect::<Vec<&str>>();

    let mut state = ParserState::FreshIngredients;
    let mut fresh_ingredient_ranges = Vec::new();
    let mut ingredient_ids = Vec::new();

    for line in lines {
        match state {
            ParserState::FreshIngredients => {
                if line.is_empty() {
                    state = ParserState::IngredientIDs;
                } else {
                    let parts: Vec<&str> = line.split('-').collect();
                    let start: u64 = parts[0].parse().unwrap();
                    let end: u64 = parts[1].parse().unwrap();
                    fresh_ingredient_ranges.push(Range { start, end });
                }
            }
            ParserState::IngredientIDs => {
                let id: u64 = line.parse().unwrap();
                ingredient_ids.push(id);
            }
        }
    }

    Database {
        fresh_ingredient_ranges,
        ingredient_ids,
    }
}

fn print_database(database: &Database) {
    println!("Fresh Ingredient Ranges:");
    for range in &database.fresh_ingredient_ranges {
        println!("{}-{}", range.start, range.end);
    }
    println!("\nIngredient IDs:");
    for id in &database.ingredient_ids {
        println!("{}", id);
    }
}

fn count_fresh_ingredients(database: &Database) -> u64 {
    let mut count = 0;
    for id in &database.ingredient_ids {
        for range in &database.fresh_ingredient_ranges {
            if *id >= range.start && *id <= range.end {
                count += 1;
                break;
            }
        }
    }
    count
}

pub fn part_one(input: &str) -> Option<u64> {
    let database = parse_database(input);
    print_database(&database);
    Some(count_fresh_ingredients(&database))
}

fn merge_duplicate_ranges(ranges: &mut Vec<Range>) {
    ranges.sort_by(|a, b| a.start.cmp(&b.start));
    let mut merged_ranges: Vec<Range> = Vec::new();

    for range in ranges.iter() {
        if let Some(last) = merged_ranges.last_mut() {
            if range.start <= last.end + 1 {
                last.end = last.end.max(range.end);
            } else {
                merged_ranges.push(Range {
                    start: range.start,
                    end: range.end,
                });
            }
        } else {
            merged_ranges.push(Range {
                start: range.start,
                end: range.end,
            });
        }
    }

    *ranges = merged_ranges;
}

fn get_num_total_possible_fresh_ingredients(database: &Database) -> u64 {
    let mut total = 0;
    let mut ranges = database.fresh_ingredient_ranges.clone();
    merge_duplicate_ranges(&mut ranges);
    for range in &ranges {
        total += range.end - range.start + 1;
    }
    total
}

pub fn part_two(input: &str) -> Option<u64> {
    let database = parse_database(input);
    Some(get_num_total_possible_fresh_ingredients(&database))
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
        assert_eq!(result, Some(14));
    }
}
