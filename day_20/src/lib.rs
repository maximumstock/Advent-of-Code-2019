// use lib::Grid;
// use std::cmp::Ordering;
// use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
// use std::fmt::{Display, Error, Formatter};
//
// fn main() {
//     part1(input());
//     part2(input());
// }
//
// fn part1(input: &'static str) -> usize {
//     let (portal_tiles, grid) = parse_grid(input);
//     let graph = create_graph(&grid, &portal_tiles);
//     let shortest_path = find_shortest_path(&graph, "AA", "ZZ");
//     println!("Shortest path is {}", shortest_path);
//     shortest_path
// }
//
// fn part2(input: &'static str) -> usize {
//     let (portal_tiles, grid) = parse_grid(input);
//     let graph = create_graph(&grid, &portal_tiles);
//     let first_part = find_shortest_path(&graph, "AA", "ZZ");
//     let second_part = find_shortest_path(&graph, "ZZ", "AA");
//     let shortest_path = first_part + second_part + 1;
//     println!("Shortest path with max depth is {}", shortest_path);
//     shortest_path
// }
//
// #[derive(Debug, Eq, PartialEq, Hash, Clone)]
// enum Tile {
//     Passage,
//     Empty,
//     Portal(String),
//     Wall,
// }
//
// impl Tile {
//     fn to_char(&self) -> String {
//         match self {
//             Tile::Passage => ".",
//             Tile::Wall => "#",
//             Tile::Empty => " ",
//             Tile::Portal(c) => c,
//         }
//         .parse()
//         .unwrap()
//     }
//
//     fn from_char(c: char) -> Self {
//         match c {
//             '#' => Tile::Wall,
//             '.' => Tile::Passage,
//             ' ' => Tile::Empty,
//             _ => Tile::Portal(c.to_string()),
//         }
//     }
// }
//
// #[derive(Debug, Clone, Eq, PartialEq, Hash)]
// struct Node {
//     position: Position,
//     tile: Tile,
// }
//
// impl Default for Tile {
//     fn default() -> Self {
//         Tile::Empty
//     }
// }
//
// impl Display for Tile {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
//         write!(f, "{}", self.to_char())
//     }
// }
//
// #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
// struct Position {
//     x: isize,
//     y: isize,
// }
//
// impl Position {
//     fn neighbours(&self) -> Vec<Position> {
//         [(1, 0), (0, 1), (-1, 0), (0, -1)]
//             .iter()
//             .map(|(dx, dy)| Position {
//                 x: self.x + *dx,
//                 y: self.y + *dy,
//             })
//             .collect::<Vec<Position>>()
//     }
// }
//
// impl Ord for Position {
//     fn cmp(&self, other: &Self) -> Ordering {
//         self.x.cmp(&other.x).then(self.y.cmp(&other.y))
//     }
// }
//
// impl PartialOrd for Position {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(&other))
//     }
// }
//
// fn parse_grid(input: &'static str) -> (HashMap<Position, Tile>, Grid<Tile>) {
//     let mut map: HashMap<Position, Tile> = HashMap::new();
//     let mut current_position = Position { x: 0, y: 0 };
//
//     for line in input.lines() {
//         for ch in line.chars() {
//             let tile = Tile::from_char(ch);
//             map.insert(current_position, tile);
//             current_position.x += 1;
//         }
//         current_position.x = 0;
//         current_position.y += 1;
//     }
//
//     let width = map.keys().map(|pos| pos.x).max().unwrap() as usize + 1;
//     let height = map.keys().map(|pos| pos.y).max().unwrap() as usize + 1;
//     let mut grid: Grid<Tile> = Grid::new(width, height, 0, 0);
//
//     map.iter()
//         .for_each(|(k, v)| grid.set(k.x, k.y, v.clone()).unwrap());
//
//     let mut portal_map = map
//         .iter()
//         .filter(|(pos, tile)| match tile {
//             Tile::Portal(_) => true,
//             _ => false,
//         })
//         .map(|(pos, tile)| (pos.clone(), tile.clone()))
//         .collect::<HashMap<Position, Tile>>();
//
//     merge_portal_halfs(&mut portal_map, &grid);
//
//     (portal_map, grid)
// }
//
// fn merge_portal_halfs(portal_map: &mut HashMap<Position, Tile>, grid: &Grid<Tile>) {
//     for (pos, tile) in &portal_map.clone() {
//         for neighbour_position in &pos.neighbours() {
//             if let Some(other) = portal_map.get(neighbour_position) {
//                 if let Tile::Portal(other_name) = other {
//                     // Merge both portal halfs to one portal
//                     if let Tile::Portal(tile_name) = tile {
//                         if tile_name.len().ne(&other_name.len()) {
//                             continue;
//                         }
//
//                         let (merged_name, merged_position) = merge_portals(
//                             &grid,
//                             tile_name.clone(),
//                             *pos,
//                             other_name.clone(),
//                             *neighbour_position,
//                         );
//
//                         portal_map.remove(pos);
//                         portal_map.remove(neighbour_position);
//                         portal_map.insert(merged_position, Tile::Portal(merged_name));
//                     }
//                 }
//             }
//         }
//     }
// }
//
// fn is_main_portal(grid: &Grid<Tile>, position: Position) -> bool {
//     position
//         .neighbours()
//         .iter()
//         .any(|n| match grid.get(n.x, n.y) {
//             Ok(Tile::Passage) => true,
//             _ => false,
//         })
// }
//
// fn merge_portals(
//     grid: &Grid<Tile>,
//     tile_name_1: String,
//     tile_position_1: Position,
//     tile_name_2: String,
//     tile_position_2: Position,
// ) -> (String, Position) {
//     let mut merged_name = vec![tile_name_1, tile_name_2];
//     merged_name.sort();
//     let merged_name = merged_name.iter().fold(String::new(), |mut x, b| {
//         x.push_str(b);
//         x
//     });
//     if is_main_portal(grid, tile_position_1) {
//         (merged_name, tile_position_1)
//     } else {
//         (merged_name, tile_position_2)
//     }
// }
//
// fn create_graph(
//     grid: &Grid<Tile>,
//     portals: &HashMap<Position, Tile>,
// ) -> HashMap<Node, HashMap<Node, usize>> {
//     let portal_nodes = portals
//         .iter()
//         .map(|(pos, tile)| Node {
//             position: *pos,
//             tile: tile.clone(),
//         })
//         .collect::<Vec<Node>>();
//
//     let mut graph: HashMap<Node, HashMap<Node, usize>> = HashMap::new();
//
//     for node in portal_nodes {
//         let mut paths: HashMap<Node, usize> = find_reachable_nodes(&grid, &portals, &node);
//
//         if let Some(counterpart) = find_counterpart_node(&portals, &node) {
//             paths.insert(counterpart, 0);
//         }
//
//         graph.insert(node, paths);
//     }
//
//     graph
// }
//
// fn find_counterpart_node(portals: &HashMap<Position, Tile>, node: &Node) -> Option<Node> {
//     portals
//         .iter()
//         .filter(|(o_pos, _)| node.position.ne(*o_pos))
//         .find(|(_, o_tile)| match (o_tile, &node.tile) {
//             (Tile::Portal(n1), Tile::Portal(n2)) => n1.eq(n2),
//             _ => false,
//         })
//         .map(|(pos, tile)| Node {
//             position: *pos,
//             tile: tile.clone(),
//         })
// }
//
// fn find_reachable_nodes(
//     grid: &Grid<Tile>,
//     portals: &HashMap<Position, Tile>,
//     node: &Node,
// ) -> HashMap<Node, usize> {
//     let mut result = HashMap::new();
//     let mut queue = VecDeque::new();
//     let mut seen = HashSet::new();
//
//     seen.insert(node.position);
//     queue.push_front((node.position, 0));
//
//     while let Some((pos, steps)) = queue.pop_back() {
//         for neighbour in pos.neighbours() {
//             if seen.contains(&neighbour) {
//                 continue;
//             }
//             seen.insert(neighbour);
//
//             // If the next tile is part of a known merged portal
//             if let Some(tile) = portals.get(&neighbour) {
//                 if let Tile::Portal(_) = tile {
//                     let node = Node {
//                         position: neighbour,
//                         tile: tile.clone(),
//                     };
//                     result.insert(node, steps);
//                     continue;
//                 }
//             }
//
//             if let Ok(tile) = grid.get(neighbour.x, neighbour.y) {
//                 match tile {
//                     Tile::Portal(name) => {
//                         // Ignore portal tiles right next to you
//                         if steps == 0 {
//                             continue;
//                         }
//                         let node = Node {
//                             position: neighbour,
//                             tile: tile.clone(),
//                         };
//                         result.insert(node, steps + 1);
//                     }
//                     Tile::Passage => {
//                         queue.push_front((neighbour, steps + 1));
//                     }
//                     _ => (),
//                 }
//             }
//         }
//     }
//
//     result
// }
//
// #[derive(Eq, PartialEq)]
// struct DijkstraState {
//     cost: usize,
//     node: Node,
// }
//
// impl Ord for DijkstraState {
//     fn cmp(&self, other: &Self) -> Ordering {
//         other.cost.cmp(&self.cost)
//     }
// }
//
// impl PartialOrd for DijkstraState {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }
//
// fn find_node_with_name(graph: &HashMap<Node, HashMap<Node, usize>>, name: &str) -> Node {
//     graph
//         .keys()
//         .find(|node| node.tile.eq(&Tile::Portal(name.to_string())))
//         .unwrap()
//         .clone()
// }
//
// fn find_shortest_path(
//     graph: &HashMap<Node, HashMap<Node, usize>>,
//     start: &str,
//     end: &str,
// ) -> usize {
//     let start = find_node_with_name(&graph, start);
//     let end = find_node_with_name(&graph, end);
//
//     let mut queue = BinaryHeap::new();
//     let mut distances: HashMap<Node, usize> = HashMap::new();
//     let mut seen = HashSet::new();
//
//     distances.insert(start.clone(), 0);
//
//     let start_state = DijkstraState {
//         cost: 0,
//         node: start,
//     };
//     queue.push(start_state);
//
//     while let Some(next) = queue.pop() {
//         if !seen.insert(next.node.clone()) {
//             continue;
//         }
//
//         if let Some(neighbours) = graph.get(&next.node) {
//             for (node, &cost) in neighbours {
//                 let new_cost = next.cost + cost;
//                 // println!("Can reach {:?} from {:?} at {}", node, next.node, cost);
//                 let old_cost = *distances.get(node).unwrap_or(&usize::max_value());
//                 if new_cost < old_cost {
//                     let entry = distances.entry(node.clone()).or_insert(new_cost);
//                     *entry = new_cost;
//                 }
//                 let new_state = DijkstraState {
//                     cost: new_cost,
//                     node: node.clone(),
//                 };
//                 queue.push(new_state);
//             }
//         }
//     }
//     println!("{:?}", distances);
//
//     *distances.get(&end).unwrap_or(&usize::max_value()) - 1
// }
//
// fn find_shortest_path_depth(
//     graph: &HashMap<Node, HashMap<Node, usize>>,
//     start: &str,
//     end: &str,
// ) -> usize {
//     let start = find_node_with_name(&graph, start);
//     let end = find_node_with_name(&graph, end);
//
//     let mut queue_left = VecDeque::new();
//     let mut queue_right = VecDeque::new();
//
//     queue_left.push_front((start.clone(), 0 as usize, 0 as usize));
//     queue_right.push_front((end.clone(), 0 as usize, 0 as usize));
//
//     bfs(&graph, &mut queue_left, end);
//     bfs(&graph, &mut queue_right, start);
//
//     loop {
//         match find_matching_element(&queue_left, &queue_right) {
//             Some((_, left_cost, right_cost)) => return left_cost + right_cost + 1,
//             None => continue,
//         }
//     }
// }
//
// type BFSState = (Node, usize, usize);
//
// fn find_matching_element(
//     queue_left: &VecDeque<BFSState>,
//     queue_right: &VecDeque<BFSState>,
// ) -> Option<(Node, usize, usize)> {
//     None
// }
//
// fn bfs(graph: &HashMap<Node, HashMap<Node, usize>>, queue: &mut VecDeque<BFSState>, goal: Node) {
//     let mut seen = HashSet::new();
//     while let Some((node, cost, depth)) = queue.pop_back() {
//         if let Some(neighbours) = graph.get(&node) {
//             for (neighbour, &cost) in neighbours {
//                 if !seen.insert(neighbour.clone()) {
//                     continue;
//                 }
//             }
//         }
//     }
// }
//
// fn input() -> &'static str {
//     include_str!("input")
// }
//
// #[cfg(test)]
// mod tests {
//     use crate::{part1, part2};
//
//     #[test]
//     fn test_part1() {
//         let input = include_str!("example1");
//         assert_eq!(part1(input), 23);
//     }
//
//     #[test]
//     fn test_part2() {
//         let input = include_str!("example2");
//         assert_eq!(part2(input), 396);
//     }
// }
