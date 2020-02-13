// Inspiration taken from https://github.com/prscoelho/aoc2019/blob/master/src/aoc18/mod.rs

use lib::Grid;
use std::cmp::Ordering;
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{Display, Error, Formatter};

fn main() {
    part1(input());
}

fn part1(input: &'static str) -> usize {
    let grid = parse_grid(input);
    let graph: HashMap<char, HashMap<char, usize>> = parse_graph(grid);
    let path_length = search(&graph, '@');
    println!("Need {} hops", path_length);
    path_length
}

#[derive(PartialEq, Eq, Debug)]
struct BFSState {
    keys: BTreeSet<char>,
    steps: usize,
    node: char,
}

impl Ord for BFSState {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .steps
            .cmp(&self.steps)
            .then(self.keys.len().cmp(&other.keys.len()))
    }
}

impl PartialOrd for BFSState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

fn search(graph: &HashMap<char, HashMap<char, usize>>, start: char) -> usize {
    let n_keys = graph.iter().filter(|(k, _)| k.is_lowercase()).count();

    let mut best_states: HashMap<(char, BTreeSet<char>), usize> = HashMap::new();
    best_states.insert((start, BTreeSet::new()), 0);

    let start = BFSState {
        keys: BTreeSet::new(),
        steps: 0,
        node: start,
    };

    let mut queue = BinaryHeap::new();
    queue.push(start);

    let mut cache: HashMap<(char, BTreeSet<char>), Vec<(char, usize)>> = HashMap::new();

    while let Some(next) = queue.pop() {
        if next.keys.len().eq(&n_keys) {
            return next.steps;
        }

        if let Some(&best_steps) = best_states.get(&(next.node, next.keys.clone())) {
            if next.steps > best_steps {
                continue;
            }
        }

        let cache_key = (next.node, next.keys.clone());
        let cached_entry = cache
            .entry(cache_key)
            .or_insert_with(|| search_reachable_keys_from_node(&graph, &next.keys, next.node));

        //        let cached_entry = search_reachable_keys_from_node(&graph, &next.keys, next.node);

        for &(next_node, cost) in cached_entry.iter() {
            let mut next_keys = next.keys.clone();
            next_keys.insert(next_node);
            let next_steps = next.steps + cost;

            let best_entry = best_states
                .entry((next_node, next_keys.clone()))
                .or_insert(usize::max_value());

            if next_steps < *best_entry {
                *best_entry = next_steps;

                let next_state = BFSState {
                    steps: next_steps,
                    node: next_node,
                    keys: next_keys,
                };

                queue.push(next_state);
            }
        }
    }

    usize::max_value()
}

#[derive(PartialEq, Eq)]
struct DijkstraState {
    cost: usize,
    node: char,
}

impl PartialOrd for DijkstraState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for DijkstraState {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.node.cmp(&other.node))
    }
}

// Returns a list of keys that are reachable for a given node with their distances
fn search_reachable_keys_from_node(
    graph: &HashMap<char, HashMap<char, usize>>,
    keys: &BTreeSet<char>,
    start: char,
) -> Vec<(char, usize)> {
    let mut distances = HashMap::new();
    for &key in graph.keys() {
        distances.insert(key, usize::max_value());
    }
    *distances.get_mut(&start).unwrap() = 0;

    let mut heap = BinaryHeap::new();

    heap.push(DijkstraState {
        cost: 0,
        node: start,
    });
    let mut reachable = HashSet::new();

    while let Some(DijkstraState { cost, node }) = heap.pop() {
        if node.is_lowercase() && !keys.contains(&node) {
            reachable.insert(node);
            continue;
        }

        if cost > distances[&node] {
            continue;
        }

        for (&next_node, &next_cost) in graph.get(&node).unwrap().iter() {
            // check if we have key
            if next_node.is_uppercase() && !keys.contains(&next_node.to_ascii_lowercase()) {
                continue;
            }

            let next = DijkstraState {
                cost: cost + next_cost,
                node: next_node,
            };

            if next.cost < distances[&next_node] {
                distances.insert(next_node, next.cost);
                heap.push(next);
            }
        }
    }

    reachable
        .into_iter()
        .map(|node| (node, distances[&node]))
        .collect()
}

fn parse_graph(grid: Grid<Tile>) -> HashMap<char, HashMap<char, usize>> {
    let mut result = HashMap::new();

    for x in grid.iter() {
        match x.element {
            Tile::Door(c) | Tile::Key(c) => {
                let position = Position { x: x.x, y: x.y };
                let reachable_nodes = find_reachable_nodes_from(&grid, position);
                result.insert(c, reachable_nodes);
            }
            _ => (),
        }
    }

    result
}

fn find_reachable_nodes_from(grid: &Grid<Tile>, position: Position) -> HashMap<char, usize> {
    let mut reachable_nodes = HashMap::new();
    let mut visited = HashSet::new();
    visited.insert(position);

    let mut queue = VecDeque::new();
    queue.push_front((position, 0));

    let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    while let Some((current_position, steps)) = queue.pop_back() {
        for direction in &directions {
            let new_position = Position {
                x: current_position.x + direction.0,
                y: current_position.y + direction.1,
            };

            if let Ok(neighbour) = grid.get(new_position.x, new_position.y) {
                if !visited.contains(&new_position) {
                    visited.insert(new_position);
                    match neighbour {
                        Tile::Key(c) | Tile::Door(c) => {
                            reachable_nodes.insert(*c, steps + 1);
                        }
                        Tile::Passage => {
                            queue.push_front((new_position, steps + 1));
                        }
                        _ => (),
                    }
                }
            }
        }
    }

    reachable_nodes
}

fn parse_grid(input: &str) -> Grid<Tile> {
    let lines: Vec<String> = input
        .split('\n')
        .map(|x| String::from(x.trim()))
        .collect::<Vec<String>>();
    let width = lines[0].len();
    let height = lines.len();

    let mut grid: Grid<Tile> = Grid::new(width, height, 0, 0);

    let mut x: isize = 0;

    for (y, line) in lines.into_iter().enumerate() {
        for char in line.chars() {
            grid.set(x, y as isize, Tile::from_char(char)).unwrap();
            x += 1;
        }
        x = 0;
    }

    grid
}

#[derive(Debug, Clone, PartialEq)]
enum Tile {
    Wall,
    Passage,
    Door(char),
    Key(char),
}

impl Tile {
    fn to_char(&self) -> char {
        match self {
            Tile::Passage => '.',
            Tile::Wall => '#',
            Tile::Key(c) => *c,
            Tile::Door(c) => *c,
        }
    }

    fn from_char(char: char) -> Tile {
        match char {
            '#' => Tile::Wall,
            '.' => Tile::Passage,
            c => {
                if c.is_ascii_uppercase() {
                    Tile::Door(c)
                } else {
                    Tile::Key(c)
                }
            }
        }
    }
}

impl Default for Tile {
    fn default() -> Self {
        Tile::Passage
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.to_char())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
struct Position {
    x: isize,
    y: isize,
}

fn input() -> &'static str {
    "#################################################################################
    #.......#.......#..q..W...#..n......F...#.......#...#...#.....#.....#.#.......#.#
    #.#######.###.###.#####.#.#.###########.#.#####.#.#.#.###.###.#.###.#.#.#.###.#.#
    #..x........#.....#...#.#.#.#.........#.#...#...#.#...#...#.#.....#.#...#.#...#.#
    ###########.#######.#.#.#.###.#######.#.#.#.#.###.#####.###.#######.#.###.#.###.#
    #...#...#.......#...#...#.#...#.....#.#.#.#.#.....#...#...#.#.......#.#..b#.#...#
    #.#.#.#.#######.#.#######.#.###.###.#.#.###.#####.#.#.###.#.#.#######.#.###.#.#.#
    #.#...#...#.....#.#.....#.#.#...#.#.#...#...#.....#.#...#...#.....#.#.#.#...#.#.#
    #.#######.#######.#.#####.#.#.###.#.###.#.#########.###.###.#####.#.#.#.###.###.#
    #.......#...#...#.#.........#...#...#...#.#...........#.#.......#.#...#...#.#...#
    #.#####.###.#.#.#Z#############.#.#####.#.#.###########.#.#######.#.#####.#.#.#.#
    #.#.....#.#.#.#...#...#.........#.#...#.#.#.....#.......#...#.....#.#...#.#...#.#
    #.#.#####.#.#.#####.#.#.#########.#.#.#.#.#####.#.#######.###.#####.#.#.#.#####.#
    #.#.#c..#...#...#...#.#.........#...#.#.#.#.....#.......#.#...#.#...#.#.#...#...#
    ###.#.#.#.#####.#.###.#####.###.#####.###.#.###########.#.#.###.#.###.#.###.#.###
    #.G.#.#...#...#.#.#...#...#.#...#...#...#.#.....#.......#.#...#.#...#.#.#...#.#.#
    #.###.#####.#.#.#.#.###.#.###.#####.###.#.#.#####.#####.#.###.#.###.###.#.###.#.#
    #.#.........#.#.#.#.....#...#.#.....#...#...#...#...#...#.#.#.#...#.....#k..#.#.#
    #.###########.#.#.#########.#.#.#####O#######.#.#.#.#.###.#.#.###.#####.###.#.#.#
    #.#...#...#i....#...#.....#...#.....#.#.#.....#.#.#.#...#...#.#...#...#.#.D.#.#.#
    #.#.#.#.#.#####.###.###.#.#####.###.#.#.#.#####.###.###.###.#.#.#.#.#.#.#.###.#.#
    #.#.#...#.#...#.#.#...#.#...#...#...#...#...#.......#...#...#.#.#.#.#...#.#...#.#
    #.#.#####.#.#.#.#.#####.###.#.###.#####.#.#.#############.#####.###.#####.#.###.#
    #.#.....#...#.#...#...#.#...#...#.....#.#.#.......#.......#.....#...#.....#.#...#
    #.#####.#####.###.#.#.#.#.#########.#.#.#.#######.#.#######.#####.###.#####.#.#.#
    #.....#.#.....#.....#...#.........#.#.#.#.#.....#.#...#.........#...#...#.....#.#
    #.###.#.#.###.###################.###.#.###.###.#.###.#.#####.#.###.###.#########
    #.#.B.#.#.U.#.#.....#.......#...#.....#.#...#.....#...#.#...#.#...#...#...#.....#
    ###.###.###.#.#.###.#.#.#####.#.###.###.#.#######.#.#####.#.#####.###.###.#.###.#
    #...#...#...#.#..a#.#.#.......#.....#...#...#...#.#.......#.....#......h#...#.#.#
    #.#.#.###.#######.#.#################.###.#.#.#.#.#############.#########.###J#.#
    #.#.#.#.#.#...#...#.#.......#.......#...#.#...#.#.#.....#.....#.....#...#.....#.#
    #.###.#.#.#.#.#.###.#.#####.#.#####.###.#.#####T###.###.#.###.#####.#.#.#####.#.#
    #...#...#...#...#.#.......#.#.....#..y..#.#...#...#.#.#.....#.....#...#.#.....#.#
    #.#.###.#########.#########.#####.#########.#.###.#.#.#######.###.#####.#######.#
    #.#...#.#.....#.....#.#.........#...#...#...#.....#.......#...#...#...#...#.....#
    #.###.#.#.#.#.###.#.#.#.#.#########.###.#.###.###########.#.#######.#.###.#.###.#
    #...#...#.#.#.#...#...#.#.#.......#...#.#...#.#.........#.#.#.......#u..#.R.#.#.#
    ###.#######.#.#.#####.#.###.###.#####.#.#.#.###.#######.#.#.#A#########.#####.#.#
    #...........#...#.....#.......#...........#...........#...#...........#.........#
    #######################################.@.#######################################
    #.#...........#.......#...#.....#...#.......#...........#...............#.......#
    #.#.#######.#.###.###.#.#.###.###.#.#.#.#.###.#####.#.###.###.#.#######.###.###.#
    #...#r......#.....#.#...#...#.....#.#.#.#.#...#...#.#.#...#.#.#.#...#.......#...#
    #####.#############.#######.###.###.#.#.#.#.#####.#.###.###.#.###.#.#############
    #j..#.#.......#...........#...#...#...#.#.#.....#.#...#.....#.....#.#.......#...#
    #.#.#.#####.#.#.#######.#.###.#########.#.#####.#.###.#####.#######.#.#####.#.#.#
    #.#...#..d#.#...#.....#.#...#.#.....#...#.....#.#...#...#...#....m#.#.#e..#...#.#
    #.#####.#.#.#####.###.#.###.#.#.###.#.###.#####.#.###.###.###.###.#.#.#.#.###.#.#
    #.#.....#.#.....#.M.#.#...#.....#...#...#.#...#...#...#...#.#...#.#.#.#.#...#.#.#
    #.#.#####.#####.###.#.###.#######.#####.#.#.#.###.#.###.###.#.###.#.#.#.###.###.#
    #.....#...#...#...#.#...#.#...#.......#.#...#.....#.#...#...#.#...#...#.#.....V.#
    #.#####.###.#.###.#.###.###.#.#######.#.#.#########.#.###.#.#.#.###.###.#######.#
    #.#.#...#...#.#...#.#.S.....#.....#.#.#.#t#.........#...#.#.#.#.#.#.#g#...#.....#
    #.#.#.###.###.#.###.#############.#.#.#.###.#######.###.#.#.#.#.#.#.#.###.#####.#
    #...#.#...#...#...#...#...#...#...#...#.#...#.....#...#.#.#.#s#.#.#.....#.#...#.#
    #####.#####.#####.###.###.#.#.#.###.###.#.###.###.#####.#.###.#.#.#####.#.#.#.#.#
    #.....#...#.#...#...#.#.N.#.#.#.#.#.....#.....#.#.#.......#...#.#...#...#...#.#.#
    #.###.#.#.#.#.#.#.###.#.###.#.#.#.#####.#.#####.#.#.#######.###.#.###.#######.###
    #.#...#.#.#...#...#...#.....#p..#.....#.#.#.....#.#.#...P...#l..#.#...#.....#...#
    #K#####.#.#.#######L###.#########.###.#.#.###.#.#.#.#.#######.###.#.###.#####.#.#
    #.#.....#.#.#.....#...#...#.....Y.#.#.#.#.....#.#.#.#.#.#...#.#...#.....#...#.#.#
    #.#.#####.#.#####.#######.#######.#.#.#.#.#######.#.#.#.#.#.###.#.#####.#.#.###.#
    #.....#...#...#.......#.#...#...#o..#...#.#.....#...#.#.#.#.#...#.......#.#.....#
    #######.#####.#.#####.#.###.#.#.#########.#.###.#####.#.#.#.#.###########.#####.#
    #.......#.....#.#.........#...#.........#.#...#.......#...#.........#.....#...#.#
    #.#######.#####.#.#####################.#.###.#######################.#####.###.#
    #....v..#.......#.#...........#...#.....#...#.#.........#.............#.#.C.....#
    #.#####.###########.#########.#.#.#.#########.###.#####.#.#############.#.#######
    #.#...#.....#.......#.........#.#...#...#.....#...#.....#.......#.......#.....#.#
    #.#.#.#####.#.#######.#######.#.#####.#.#.#####.###.#####.#####.#.#####.#####.#.#
    #...#.#...#...#.....#.....#...#.#.....#.#....f..#.#.Q...#.#.....#.#...#.....#...#
    #####.###.#######.#####.#.#####.#.###.###########.#####.###.#######.#.#.###.###.#
    #.....#.....#...#.#...#.#...#...#...#...#.#...........#...#.#...#...#...#.....#.#
    #.#####.###.#.#.#.#.#.#####.#.#########.#.#.#####.###.###.#.#.#.#.#######.#####.#
    #...#.H.#.....#.#.#.#.....#.#.#...#...#.#...#...#.#.....#.#...#.#...#.....#...#.#
    ###.#.#.#######.#.#.#####.#.#.#.#.#.#.#.#.#####.#.#######.#####.###.#######.#.#.#
    #.#.#.#.....#.#.#.#.....#...#...#...#.#.#.#.....#.......#.......#..w#......z#.I.#
    #.#.#######.#.#.#.#####.#############.#.#.###.#X#######.#########.###.###########
    #.......E...#.........#.................#.....#.......#.........................#
    #################################################################################"
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn test_small_labyrinth() {
        let input = "#########
                            #b.A.@.a#
                            #########";

        let runner = part1(input);
        assert_eq!(runner, 8);
    }

    #[test]
    fn test_medium_labyrinth() {
        let input = "\
                ########################
                #f.D.E.e.C.b.A.@.a.B.c.#
                ######################.#
                #d.....................#
                ########################";
        let result = part1(input);
        assert_eq!(result, 86);
    }

    #[test]
    fn test_medium_labyrinth2() {
        let input = "\
                #################
                #i.G..c...e..H.p#
                ########.########
                #j.A..b...f..D.o#
                ########@########
                #k.E..a...g..B.n#
                ########.########
                #l.F..d...h..C.m#
                #################";
        let result = part1(input);
        assert_eq!(result, 132);
    }

    #[test]
    fn test_medium_labyrinth3() {
        let input = "\
                ########################
                #@..............ac.GI.b#
                ###d#e#f################
                ###A#B#C################
                ###g#h#i################
                ########################";
        let result = part1(input);
        assert_eq!(result, 81);
    }
}
