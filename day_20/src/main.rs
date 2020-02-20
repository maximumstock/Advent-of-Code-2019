use lib::Grid;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Display, Error, Formatter};

fn main() {
    part1(include_str!("input"));
    part2(include_str!("input"));
}

fn part1(input: &str) -> usize {
    let mut map = parse_map(input);
    let shortest_path = bfs(&mut map, "AA", "ZZ");
    println!("Part 1: {}", shortest_path);
    shortest_path
}

fn part2(input: &str) -> usize {
    let mut map = parse_map(input);
    let shortest_path = bfs_with_depth(&mut map, "AA", "ZZ");
    println!("Part 2: {}", shortest_path);
    shortest_path
}

#[derive(Debug, Clone)]
enum Tile {
    Wall,
    Passage,
    Empty,
    Portal(char),
}

impl Tile {
    fn from_char(char: char) -> Tile {
        match char {
            '#' => Tile::Wall,
            '.' => Tile::Passage,
            ' ' => Tile::Empty,
            c => Tile::Portal(c),
        }
    }

    fn to_str(&self) -> char {
        match self {
            Tile::Portal(c) => *c,
            Tile::Passage => '.',
            Tile::Wall => '#',
            Tile::Empty => ' ',
        }
    }

    fn is_portal(&self) -> bool {
        match self {
            Tile::Portal(_) => true,
            _ => false,
        }
    }
}

impl Default for Tile {
    fn default() -> Self {
        Tile::Empty
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.to_str())
    }
}

#[derive(Clone, Debug)]
struct Portal {
    position: Position,
    name: String,
    portal_type: PortalType,
}

#[derive(Clone)]
struct Map {
    grid: Grid<Tile>,
    portals: HashMap<Position, Portal>,
}

#[derive(Clone, Debug, Copy)]
enum PortalType {
    Inner,
    Outer,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn neighbours(&self) -> Vec<Position> {
        [(1, 0), (0, 1), (-1, 0), (0, -1)]
            .iter()
            .map(|(dx, dy)| Position {
                x: self.x + *dx,
                y: self.y + *dy,
            })
            .collect::<Vec<Position>>()
    }
}

fn find_portal_positions_for_name(portals: &HashMap<Position, Portal>, name: &str) -> Vec<Portal> {
    portals
        .iter()
        .filter(|(pos, portal)| name.eq(&portal.name))
        .map(|(_, portal)| portal.clone())
        .collect::<Vec<Portal>>()
}

fn bfs(map: &Map, start: &str, end: &str) -> usize {
    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();

    let start = find_portal_positions_for_name(&map.portals, start)
        .first()
        .unwrap()
        .clone();
    let end = find_portal_positions_for_name(&map.portals, end)
        .first()
        .unwrap()
        .clone();

    type BFSState = (Position, usize);

    let start_state: BFSState = (start.position, 0);
    queue.push_back(start_state);
    seen.insert(start.position);

    while let Some((current_position, cost)) = queue.pop_front() {
        let current_tile = map
            .grid
            .get(current_position.x, current_position.y)
            .unwrap();

        for neighbour in current_position.neighbours() {
            if !seen.insert(neighbour) {
                continue;
            }

            if let Ok(tile) = map.grid.get(neighbour.x, neighbour.y) {
                match tile {
                    Tile::Passage => {
                        let next_state = (neighbour, cost + 1);
                        queue.push_back(next_state);
                    }
                    Tile::Portal(_) => {
                        if current_tile.is_portal() && tile.is_portal() {
                            continue;
                        }

                        let in_portal = map.portals.get(&neighbour).unwrap().clone();
                        let portal = find_portal_positions_for_name(&map.portals, &in_portal.name)
                            .iter()
                            .find(|p| p.position.ne(&neighbour))
                            .cloned();
                        if let Some(portal) = portal {
                            let next_state = (portal.position, cost);
                            seen.insert(portal.position);
                            queue.push_front(next_state);
                        } else {
                            return cost - 1;
                        }
                    }
                    Tile::Empty | Tile::Wall => (),
                }
            }
        }
    }

    usize::max_value()
}

fn bfs_with_depth(map: &mut Map, start: &str, end: &str) -> usize {
    let start = find_portal_positions_for_name(&map.portals, start)
        .first()
        .unwrap()
        .clone();
    let end = find_portal_positions_for_name(&map.portals, end)
        .first()
        .unwrap()
        .clone();

    type BFSState = (Position, usize, isize, Portal);
    type SeenState = (Position, isize);

    let mut seen: HashSet<SeenState> = HashSet::new();
    let mut queue = VecDeque::new();

    let start_state: BFSState = (start.position, 0, 0, start.clone());
    queue.push_back(start_state);
    seen.insert((start.position, 0));

    let mut grid = map.clone().grid;

    while let Some(state) = queue.pop_front() {
        let (current_position, cost, depth, last_portal) = state;
        let current_tile = map
            .grid
            .get(current_position.x, current_position.y)
            .unwrap();

        if depth > map.portals.len() as isize {
            continue;
        }
        // print!("\x1B[2J");
        // println!("{}", grid);

        for neighbour in current_position.neighbours() {
            if !seen.insert((neighbour, depth)) {
                continue;
            }

            if let Ok(tile) = map.grid.get(neighbour.x, neighbour.y) {
                match tile {
                    Tile::Passage => {
                        let next_state = (neighbour, cost + 1, depth, last_portal.clone());
                        grid.set(neighbour.x, neighbour.y, Tile::Wall).unwrap();
                        queue.push_back(next_state);
                    }
                    Tile::Portal(_) => {
                        if current_tile.is_portal() && tile.is_portal() {
                            continue;
                        }

                        let start_portal = map.portals.get(&neighbour).unwrap().clone();
                        seen.insert((start_portal.position, depth));

                        let end_portal =
                            find_portal_positions_for_name(&map.portals, &start_portal.name)
                                .iter()
                                .find(|p| p.position.ne(&neighbour))
                                .cloned();

                        if let Some(end_portal) = end_portal {
                            let next_depth = match end_portal.portal_type {
                                PortalType::Inner => depth - 1,
                                PortalType::Outer => depth + 1,
                            };

                            if next_depth < 0 {
                                continue;
                            }

                            let next_state =
                                (end_portal.position, cost, next_depth, end_portal.clone());

                            // println!(
                            //     "Porting from {:?} to {:?} to depth {} at cost {}",
                            //     start_portal, end_portal, next_state.2, next_state.1
                            // );

                            queue.push_front(next_state);
                            seen.insert((end_portal.position, next_depth));
                        } else {
                            // println!("Reached ZZ with {:?}", state.1);
                            // Implicitly found ZZ by ending up at a portal which does not have a counterpart
                            if depth.eq(&0) {
                                return cost - 1;
                            } else {
                                continue;
                            }
                        }
                    }
                    Tile::Empty | Tile::Wall => (),
                }
            }
        }
    }

    usize::max_value()
}

fn parse_map(input: &str) -> Map {
    let mut tiles: HashMap<Position, Tile> = HashMap::new();

    // Collect all tiles
    let mut current_position = Position { x: 0, y: 0 };
    for line in input.lines() {
        for ch in line.chars() {
            let tile = Tile::from_char(ch);
            tiles.insert(current_position, tile);
            current_position.x += 1;
        }
        current_position.x = 0;
        current_position.y += 1;
    }

    // Build a grid based on collected tiles
    let width = tiles.keys().map(|pos| pos.x).max().unwrap() as usize + 1;
    let height = tiles.keys().map(|pos| pos.y).max().unwrap() as usize + 1;

    let mut grid: Grid<Tile> = Grid::new(width, height, 0, 0);

    tiles
        .iter()
        .for_each(|(k, v)| grid.set(k.x as isize, k.y as isize, v.clone()).unwrap());

    Map {
        grid,
        portals: build_portal_map(&tiles),
    }
}

fn build_portal_map(tiles: &HashMap<Position, Tile>) -> HashMap<Position, Portal> {
    let portals = tiles.iter().filter_map(|(pos, tile)| {
        if let Tile::Portal(c) = tile {
            Some((*pos, c))
        } else {
            None
        }
    });

    let mut portal_pairs = vec![];

    for left in portals.clone() {
        for right in portals.clone() {
            portal_pairs.push((left, right));
        }
    }

    portal_pairs
        .iter()
        .filter(|(left, right)| {
            let sum = (left.0.x - right.0.x).abs() + (left.0.y - right.0.y).abs();
            sum.abs().eq(&1)
        })
        .map(|(left, right)| merge_portal_tiles(&tiles, &left, &right))
        .collect()
}

fn merge_portal_tiles(
    tiles: &HashMap<Position, Tile>,
    left: &(Position, &char),
    right: &(Position, &char),
) -> (Position, Portal) {
    let left_is_main = left.0.neighbours().iter().any(|pos| match tiles.get(&pos) {
        Some(Tile::Passage) => true,
        _ => false,
    });

    let mut portal_names = format!("{}{}", left.1, right.1)
        .chars()
        .collect::<Vec<char>>();
    portal_names.sort();
    let portal_name = portal_names.iter().collect::<String>();
    let portal_type = match (is_at_edge(&tiles, left.0), is_at_edge(&tiles, right.0)) {
        (true, _) => PortalType::Outer,
        (_, true) => PortalType::Outer,
        (_, _) => PortalType::Inner,
    };
    let position = if left_is_main { left.0 } else { right.0 };

    let portal = Portal {
        name: portal_name,
        position,
        portal_type,
    };

    (portal.position, portal)
}

fn is_at_edge(tiles: &HashMap<Position, Tile>, position: Position) -> bool {
    !position
        .neighbours()
        .iter()
        .map(|p| tiles.contains_key(&p))
        .all(|x| x)
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn test_example1() {
        let input = include_str!("example1");
        assert_eq!(part1(input), 23);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("input");
        assert_eq!(part1(input), 654);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("example2");
        assert_eq!(part2(input), 396);
    }
}
