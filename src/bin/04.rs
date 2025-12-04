use std::default;

advent_of_code::solution!(4);

enum Cell {
    Empty,
    PaperRoll(u8),
}

fn parse_diagram(input: &str) -> Vec<Vec<Cell>> {
    let mut diagram: Vec<Vec<Cell>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Cell::Empty,
                    '@' => Cell::PaperRoll(0),
                    _ => panic!("Unexpected character"),
                })
                .collect()
        })
        .collect();

    let rows = diagram.len();
    let cols = diagram[0].len();
    for r in 0..rows {
        for c in 0..cols {
            if let Cell::PaperRoll(_) = diagram[r as usize][c as usize] {
                for (nr, nc) in get_neighbors((r as i32, c as i32)) {
                    if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                        if let Cell::PaperRoll(ref mut count) = diagram[nr as usize][nc as usize] {
                            *count += 1;
                        }
                    }
                }
            }
        }
    }

    diagram
}

fn get_neighbors(pos: (i32, i32)) -> Vec<(i32, i32)> {
    let mut neighbors = Vec::new();
    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            neighbors.push((pos.0 + dx, pos.1 + dy));
        }
    }
    neighbors
}

fn print_diagram(diagram: &Vec<Vec<Cell>>) {
    let rows = diagram.len();
    let cols = diagram[0].len();
    for r in 0..rows {
        for c in 0..cols {
            let cell = &diagram[r][c];
            match cell {
                Cell::Empty => print!("."),
                Cell::PaperRoll(count) => match count {
                    &n if n < 4u8 => print!("\x1b[1;31mx\x1b[0m"),
                    _ => print!("@"),
                },
            }
        }
        println!();
    }
}

fn print_diagram_extended(diagram: &Vec<Vec<Cell>>) {
    let rows = diagram.len();
    let cols = diagram[0].len();
    for r in 0..rows {
        for c in 0..cols {
            let cell = &diagram[r][c];
            match cell {
                Cell::Empty => print!("._"),
                Cell::PaperRoll(count) => match count {
                    &n if n < 4u8 => print!("\x1b[1;31mx{}\x1b[0m", count),
                    _ => print!("@{}", count),
                },
            }
        }
        println!();
    }
}

fn get_rolls_to_remove(diagram: &Vec<Vec<Cell>>) -> Vec<(usize, usize)> {
    let mut rolls_to_remove = Vec::new();
    let rows = diagram.len();
    let cols = diagram[0].len();
    for r in 0..rows {
        for c in 0..cols {
            if let Cell::PaperRoll(count) = &diagram[r][c] {
                if *count < 4u8 {
                    rolls_to_remove.push((r, c));
                }
            }
        }
    }
    rolls_to_remove
}

fn remove_rolls(diagram: &mut Vec<Vec<Cell>>, rolls_to_remove: &Vec<(usize, usize)>) {
    for &(r, c) in rolls_to_remove {
        diagram[r][c] = Cell::Empty;
        for (nr, nc) in get_neighbors((r as i32, c as i32)) {
            if nr >= 0 && nr < diagram.len() as i32 && nc >= 0 && nc < diagram[0].len() as i32 {
                if let Cell::PaperRoll(ref mut count) = diagram[nr as usize][nc as usize] {
                    *count = count.saturating_sub(1);
                }
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let diagram = parse_diagram(input);

    print_diagram_extended(&diagram);

    Some(
        diagram
            .iter()
            .flatten()
            .map(|cell| match cell {
                Cell::PaperRoll(n) if *n < 4u8 => 1,
                _ => 0,
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut diagram = parse_diagram(input);

    let mut total_removed_rolls = 0;
    loop {
        print_diagram(&diagram);
        let rolls_to_remove = get_rolls_to_remove(&diagram);
        if rolls_to_remove.is_empty() {
            break;
        }
        remove_rolls(&mut diagram, &rolls_to_remove);
        println!("Removed {} rolls", rolls_to_remove.len());
        total_removed_rolls += rolls_to_remove.len();
    }

    Some(total_removed_rolls as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
