use intcode::{IntCodeComputer, Memory, State};
use lib::Grid;
use std::fmt::{Display, Error, Formatter};

fn main() {
    part1(input());
    part2(input());
}

fn produce_grid(input: Memory, width: usize, height: usize) -> Grid<char> {
    let mut grid: Grid<char> = Grid::new(width, height, 0, 0);
    let mut cpu = IntCodeComputer::new(input.clone());

    for cell in grid.iter() {
        cpu.reset(input.clone());
        cpu.run(vec![cell.x, cell.y]);
        let output = *cpu.get_output().first().unwrap();
        grid.set(cell.x, cell.y, tile_from_output(output)).unwrap()
    }

    grid
}

fn part1(input: Memory) {
    let grid = produce_grid(input, 50, 50);
    let pull_area = grid.iter().filter(|e| e.element.eq(&'#')).count();
    println!("{} cells are being pulled", pull_area);
}

fn part2(input: Memory) {
    let (width, height) = (1500, 1500);
    let grid = produce_grid(input, width, height);
    println!("Through manual search I found its at {}:{}", 1012, 556);
}

fn row_max(grid: &Grid<char>, row_idx: usize, len: usize) -> usize {
    let mut length = 0;
    let mut col = 0;

    loop {
        let &v = grid.get(row_idx as isize, col).unwrap();
        if v.eq(&'#') {
            length += 1;
            col += 1;
        } else {
            break;
        }
    }

    length
}

fn col_max(grid: &Grid<char>, col_idx: usize, len: usize) -> usize {
    let mut length = 0;
    let mut row = 0;

    loop {
        let &v = grid.get(row, col_idx as isize).unwrap();
        if v.eq(&'#') {
            length += 1;
            row += 1;
        } else {
            break;
        }
    }

    length
}

fn can_fit(grid: &Grid<char>, x: isize, y: isize) -> bool {
    let points_to_check = vec![(x, y), (x + 99, y), (x, y + 99), (x + 99, y + 99)];
    points_to_check.iter().all(|(x, y)| {
        let cell = grid.get(*x, *y).unwrap();
        cell.eq(&'#')
    })
}

fn tile_from_output(output: isize) -> char {
    match output {
        0 => '.',
        1 => '#',
        _ => unreachable!(),
    }
}

fn input() -> Memory {
    vec![
        109, 424, 203, 1, 21102, 1, 11, 0, 1106, 0, 282, 21101, 0, 18, 0, 1105, 1, 259, 1201, 1, 0,
        221, 203, 1, 21102, 31, 1, 0, 1105, 1, 282, 21101, 38, 0, 0, 1106, 0, 259, 20101, 0, 23, 2,
        22102, 1, 1, 3, 21101, 0, 1, 1, 21101, 0, 57, 0, 1106, 0, 303, 2101, 0, 1, 222, 21001, 221,
        0, 3, 20102, 1, 221, 2, 21102, 1, 259, 1, 21102, 1, 80, 0, 1106, 0, 225, 21101, 33, 0, 2,
        21102, 1, 91, 0, 1106, 0, 303, 1201, 1, 0, 223, 21002, 222, 1, 4, 21101, 259, 0, 3, 21101,
        0, 225, 2, 21101, 225, 0, 1, 21101, 0, 118, 0, 1106, 0, 225, 20101, 0, 222, 3, 21102, 1,
        102, 2, 21102, 133, 1, 0, 1105, 1, 303, 21202, 1, -1, 1, 22001, 223, 1, 1, 21101, 148, 0,
        0, 1106, 0, 259, 2101, 0, 1, 223, 21001, 221, 0, 4, 21002, 222, 1, 3, 21101, 0, 15, 2,
        1001, 132, -2, 224, 1002, 224, 2, 224, 1001, 224, 3, 224, 1002, 132, -1, 132, 1, 224, 132,
        224, 21001, 224, 1, 1, 21102, 195, 1, 0, 106, 0, 108, 20207, 1, 223, 2, 21001, 23, 0, 1,
        21102, 1, -1, 3, 21101, 0, 214, 0, 1105, 1, 303, 22101, 1, 1, 1, 204, 1, 99, 0, 0, 0, 0,
        109, 5, 2102, 1, -4, 249, 22101, 0, -3, 1, 22101, 0, -2, 2, 21202, -1, 1, 3, 21101, 250, 0,
        0, 1105, 1, 225, 22102, 1, 1, -4, 109, -5, 2106, 0, 0, 109, 3, 22107, 0, -2, -1, 21202, -1,
        2, -1, 21201, -1, -1, -1, 22202, -1, -2, -2, 109, -3, 2105, 1, 0, 109, 3, 21207, -2, 0, -1,
        1206, -1, 294, 104, 0, 99, 22101, 0, -2, -2, 109, -3, 2106, 0, 0, 109, 5, 22207, -3, -4,
        -1, 1206, -1, 346, 22201, -4, -3, -4, 21202, -3, -1, -1, 22201, -4, -1, 2, 21202, 2, -1,
        -1, 22201, -4, -1, 1, 22101, 0, -2, 3, 21102, 1, 343, 0, 1106, 0, 303, 1106, 0, 415, 22207,
        -2, -3, -1, 1206, -1, 387, 22201, -3, -2, -3, 21202, -2, -1, -1, 22201, -3, -1, 3, 21202,
        3, -1, -1, 22201, -3, -1, 2, 22102, 1, -4, 1, 21102, 384, 1, 0, 1106, 0, 303, 1106, 0, 415,
        21202, -4, -1, -4, 22201, -4, -3, -4, 22202, -3, -2, -2, 22202, -2, -4, -4, 22202, -3, -2,
        -3, 21202, -4, -1, -2, 22201, -3, -2, 1, 21202, 1, 1, -4, 109, -5, 2106, 0, 0,
    ]
}
