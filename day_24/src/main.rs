use lib::Grid;
use std::collections::HashSet;
use std::fmt::{Display, Formatter, Result};

fn main() {
    part1(input());
}

fn part1(input: &str) -> usize {
    let grid = parse_grid(&input);
    println!(
        "{}\nInitial Bio-Diversity: {}",
        grid,
        get_biodiversity_for_grid(&grid)
    );
    let first_recurring = find_first_recurring(&grid);
    println!("First recurring:\n{}", first_recurring);

    let bio_diversity = get_biodiversity_for_grid(&first_recurring);
    println!("Bio-Diversity: {}", bio_diversity);
    bio_diversity
}

fn get_biodiversity_for_grid(grid: &Grid<Tile>) -> usize {
    grid.grid()
        .iter()
        .enumerate()
        .map(|(idx, e)| match e {
            Tile::Free => 0,
            Tile::Bug => (2 as usize).pow(idx as u32),
        })
        .sum()
}

fn find_first_recurring(grid: &Grid<Tile>) -> Grid<Tile> {
    let mut seen: HashSet<Vec<Tile>> = HashSet::new();
    seen.insert(grid.grid());

    let mut last_grid = grid.clone();
    loop {
        last_grid = calculate_next_grid(&last_grid);
        if !seen.insert(last_grid.grid()) {
            break last_grid;
        }
    }
}

fn calculate_next_grid(original_grid: &Grid<Tile>) -> Grid<Tile> {
    let mut new_grid = original_grid.clone();
    for item in original_grid.iter() {
        let n_bugs = vec![(1, 0), (0, 1), (-1, 0), (0, -1)]
            .iter()
            .map(|(x, y)| match original_grid.get(item.x + *x, item.y + *y) {
                Some(Tile::Bug) => 1,
                _ => 0,
            })
            .sum::<u32>();

        let old_tile = original_grid.get(item.x, item.y).unwrap();

        let tile_change = match (old_tile, n_bugs) {
            (Tile::Free, 1) | (Tile::Free, 2) | (Tile::Bug, 1) => Tile::Bug,
            (Tile::Free, _) | (Tile::Bug, _) => Tile::Free,
        };

        new_grid.set(item.x, item.y, tile_change).unwrap();
    }

    new_grid
}

fn parse_grid(input: &str) -> Grid<Tile> {
    let lines = input.lines();
    let width = lines.clone().into_iter().map(|x| x.trim().len()).max().unwrap();
    let height = lines.clone().count();
    let mut grid: Grid<Tile> = Grid::new(width, height, 0, 0);

    let (mut x, mut y) = (0, 0);
    for line in input.lines() {
        for char in line.chars() {
            let t = Tile::from_char(char);
            grid.set(x, y, t).unwrap();
            x += 1;
        }
        x = 0;
        y += 1;
    }
    grid
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Tile {
    Bug,
    Free,
}

impl Tile {
    fn to_char(&self) -> char {
        match self {
            Tile::Bug => '#',
            Tile::Free => '.',
        }
    }

    fn from_char(c: char) -> Self {
        match c {
            '#' => Tile::Bug,
            '.' => Tile::Free,
            _ => unreachable!(),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.to_char())
    }
}

impl Default for Tile {
    fn default() -> Self {
        Tile::Free
    }
}

fn input() -> &'static str {
    include_str!("input")
}

#[cfg(test)]
mod tests {
    use crate::{calculate_next_grid, get_biodiversity_for_grid, parse_grid};

    #[test]
    fn test_biodiversity() {
        let input = ".....\n.....\n.....\n#....\n.#...";
        let grid = parse_grid(&input);
        assert_eq!(get_biodiversity_for_grid(&grid), 2_129_920);
    }

    #[test]
    fn test_calculate_next_grid() {
        let grid = parse_grid(&"....#\n#..#.\n#..##\n..#..\n#....");
        let expected = parse_grid(&"#..#.\n####.\n###.#\n##.##\n.##..");
        let calculated = calculate_next_grid(&grid);
        assert_eq!(calculated.grid(), expected.grid());
    }
}
