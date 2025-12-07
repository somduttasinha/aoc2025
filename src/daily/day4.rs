use crate::common;
use crate::common::Part;

type Row = Vec<char>;
type Grid = Vec<Row>;

pub fn day4(part: Part) {
    let path = "src/resources/day4.txt";
    let raw = common::read(path);
    let res = run(raw, part);
    println!("res: {}", res);
}

fn run(raw: String, part: Part) -> usize {
    let lines = raw.lines();
    let mut grid: Grid = Vec::new();

    for line in lines {
        let mut row = Vec::<char>::new();
        for char in line.chars() {
            row.push(char);
        }
        grid.push(row);
    }

    match part {
        Part::Part1 => {
            return get_accessible_rolls(&mut grid).len();
        }
        Part::Part2 => {
            // we are allowed to change the board
            let mut total_accessible_rows = 0;
            loop {
                let accessible_rows = get_accessible_rolls(&mut grid);
                if accessible_rows.len() == 0 {
                    break;
                }
                for (r, c) in accessible_rows.iter() {
                    grid[*r][*c] = '.';
                }
                total_accessible_rows += accessible_rows.len();
            }
            total_accessible_rows
        }
    }
}

fn get_accessible_rolls(grid: &mut Grid) -> Vec<(usize, usize)> {
    let mut accessible_rows = Vec::<(usize, usize)>::new();
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if *grid
                .get(row)
                .expect("Expect row")
                .get(col)
                .expect("expect entry")
                == '@'
            {
                let rolls_around = get_rolls_around(row, col, grid);
                if rolls_around < 4 {
                    accessible_rows.push((row, col));
                }
            }
        }
    }
    accessible_rows
}

fn get_rolls_around(r: usize, c: usize, grid: &mut Grid) -> u8 {
    let transformations: Vec<(isize, isize)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
    ];

    let positions: Vec<(usize, usize)> = transformations
        .iter()
        .map(|t| (r.checked_add_signed(t.0), c.checked_add_signed(t.1)))
        .filter(|p| p.0.is_some() && p.1.is_some()) // only keep rows on the board
        .map(|(x, y)| (x.unwrap(), y.unwrap()))
        .filter(|p| p.0 < grid.len() && p.1 < grid[r].len())
        .collect();

    let mut total = 0;
    for pos in positions {
        total += match grid.get(pos.0) {
            Some(row) => match row.get(pos.1) {
                Some('@') => 1,
                _ => 0,
            },
            _ => 0,
        }
    }
    total
}

#[test]
fn test_example_part_1() {
    let raw = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.";

    assert_eq!(run(raw.to_string(), Part::Part1), 13);
}

#[test]
fn test_example_part_2() {
    let raw = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.";

    assert_eq!(run(raw.to_string(), Part::Part2), 43);
}
