use std::collections::{HashSet, HashMap};
use itertools::Itertools;
use std::cmp::Ordering;

fn main() {
    let input = input();
    let (n_asteroids, location) = part1(input);
    println!("Can see {:?} asteroids on {:?}", n_asteroids, location);

    let location = part2(input, &Location { x: 27, y: 19 });
    println!("200th location: {:?} - {:?}", location, location.x * 100 + location.y);
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Location {
    x: i32,
    y: i32,
}

impl Location {
    fn distance(&self, other: &Location) -> Vector {
        Vector {
            x: other.x - self.x,
            y: other.y - self.y,
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Vector {
    x: i32,
    y: i32,
}

impl Ord for Vector {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.angle().cmp(&other.angle()) {
            Ordering::Equal => self.len_as_i32().cmp(&other.len_as_i32()),
            any => any
        }
    }
}

impl PartialOrd for Vector {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Option::from(self.cmp(&other))
    }
}

impl Vector {
    fn normalize(&self) -> Vector {
        if self.x == 0 && self.y != 0 {
            Vector { x: 0, y: self.y / self.y.abs() }
        } else if self.x != 0 && self.y == 0 {
            Vector { x: self.x / self.x.abs(), y: 0 }
        } else if self.x != 0 && self.y != 0 {
            let gcd = num::integer::gcd(self.x, self.y);
            if self.x % gcd == 0 {
                Vector { x: self.x / gcd, y: self.y / gcd }
            } else if self.y % gcd == 0 {
                Vector { x: self.x / gcd, y: self.y / gcd }
            } else {
                self.clone()
            }
        } else {
            self.clone()
        }
    }

    fn angle(&self) -> i64 {
        let reference = Vector { x: 0, y: 1 };
        let radians: f32 = (self.x * reference.x + self.y * reference.y) as f32 / (self.len() * reference.len()) as f32;

        let mut degrees = radians.acos().to_degrees();

        if self.x < 0 {
            degrees += 180.0;
        }

        (degrees * 1_000_000 as f32) as i64
    }

    fn len(&self) -> f32 {
        ((self.x * self.x + self.y * self.y) as f32).sqrt()
    }

    fn len_as_i32(&self) -> i32 {
        (self.len() * 100.0) as i32
    }
}

fn part1(input: &'static str) -> (usize, Location) {
    let mut locations: Vec<Location> = parse_locations(input);
    println!("Found {:?} locations", locations.len());
    find_best_location(&mut locations)
}

fn part2(input: &'static str, starting_location: &Location) -> Location {
    // Translate locations from input into <angle, vector> tuples
    let vectors: Vec<(i64, Vector)> = parse_locations(input)
        .iter()
        .filter(|l| starting_location.ne(*l))
        .map(|l| starting_location.distance(l))
        .map(|v| v.clone())
        .sorted()
        .map(|v| (v.angle().clone(), v.clone()))
        .collect();

    // Group <angle, vector> tuples into buckets by angle value to iterate over them later
    let mut vector_map: AngleVectorMap = vectors.iter()
        .map(|x| (x.0.clone(), x.1.clone()))
        .into_group_map();

    let sorted_keys = vector_map.keys()
        .sorted_by(|left, right| left.cmp(right))
        .cloned()
        .collect::<Vec<i64>>();

    let mut n_asteroids = vectors.len();
    let mut asteroid_order = vec![];

    for angle_key in sorted_keys.iter().cycle() {
        if n_asteroids == 0 {
            break;
        }
        let mut asteroids_for_angle = vector_map.get_mut(angle_key).unwrap().clone();
        match asteroids_for_angle.first().cloned() {
            Some(vec) => {
                asteroids_for_angle.remove(0);
                asteroid_order.push(vec);
                n_asteroids -= 1;
            }
            None => ()
        }
    }

    let vector_at_position = asteroid_order.get(200).unwrap();
    Location {
        x: starting_location.x + vector_at_position.x,
        y: starting_location.y + vector_at_position.y,
    }
}

type AngleVectorMap = HashMap<i64, Vec<Vector>>;

fn find_best_location(locations: &mut Vec<Location>) -> (usize, Location) {
    locations.iter()
        .map(|l| {
            let n_line_of_sight = find_los_asteroids(&locations, l);
            (n_line_of_sight, l.clone())
        })
        .max_by_key(|(n, _)| *n)
        .expect("No locations found")
        .clone()
}

fn find_asteroids(locations: &Vec<Location>, loc: &Location) -> HashSet<Vector> {
    locations.iter()
        .filter(|other| loc.ne(other))
        .map(|other| loc.distance(&other).normalize())
        .collect::<HashSet<Vector>>()
}

fn find_los_asteroids(locations: &Vec<Location>, loc: &Location) -> usize {
    let unique_vectors = find_asteroids(&locations, &loc);
    unique_vectors.len()
}

fn parse_locations(input: &'static str) -> Vec<Location> {
    input.lines()
        .into_iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.trim().chars().into_iter()
                .enumerate()
                .filter(|(_, char)| *char == '#')
                .map(|(column, _)| {
                    Location { x: column as i32, y: row as i32 }
                })
                .collect::<Vec<Location>>()
        })
        .collect()
}

fn input() -> &'static str {
    "..#..###....#####....###........#
     .##.##...#.#.......#......##....#
     #..#..##.#..###...##....#......##
     ..####...#..##...####.#.......#.#
     ...#.#.....##...#.####.#.###.#..#
     #..#..##.#.#.####.#.###.#.##.....
     #.##...##.....##.#......#.....##.
     .#..##.##.#..#....#...#...#...##.
     .#..#.....###.#..##.###.##.......
     .##...#..#####.#.#......####.....
     ..##.#.#.#.###..#...#.#..##.#....
     .....#....#....##.####....#......
     .#..##.#.........#..#......###..#
     #.##....#.#..#.#....#.###...#....
     .##...##..#.#.#...###..#.#.#..###
     .#..##..##...##...#.#.#...#..#.#.
     .#..#..##.##...###.##.#......#...
     ...#.....###.....#....#..#....#..
     .#...###..#......#.##.#...#.####.
     ....#.##...##.#...#........#.#...
     ..#.##....#..#.......##.##.....#.
     .#.#....###.#.#.#.#.#............
     #....####.##....#..###.##.#.#..#.
     ......##....#.#.#...#...#..#.....
     ...#.#..####.##.#.........###..##
     .......#....#.##.......#.#.###...
     ...#..#.#.........#...###......#.
     .#.##.#.#.#.#........#.#.##..#...
     .......#.##.#...........#..#.#...
     .####....##..#..##.#.##.##..##...
     .#.#..###.#..#...#....#.###.#..#.
     ............#...#...#.......#.#..
     .........###.#.....#..##..#.##..."
}

#[cfg(test)]
mod tests {
    use crate::{Vector, parse_locations, Location, part1, find_asteroids, find_los_asteroids};

    #[test]
    fn test_normalize_vector() {
        assert_eq!(Vector { x: 2, y: 2 }.normalize(), Vector { x: 1, y: 1 });
        assert_eq!(Vector { x: -2, y: 2 }.normalize(), Vector { x: -1, y: 1 });
        assert_eq!(Vector { x: 2, y: -2 }.normalize(), Vector { x: 1, y: -1 });
        assert_eq!(Vector { x: -10, y: -5 }.normalize(), Vector { x: -2, y: -1 });
        assert_eq!(Vector { x: -5, y: -10 }.normalize(), Vector { x: -1, y: -2 });
        assert_eq!(Vector { x: 7, y: -2 }.normalize(), Vector { x: 7, y: -2 });
        assert_eq!(Vector { x: -7, y: -2 }.normalize(), Vector { x: -7, y: -2 });
        assert_eq!(Vector { x: -6, y: -10 }.normalize(), Vector { x: -3, y: -5 });
        assert_eq!(Vector { x: -4, y: 0 }.normalize(), Vector { x: -1, y: 0 });
    }

    #[test]
    fn test_angle() {
        assert_eq!(Vector { x: 2, y: 2 }.angle(), 45_000_000);
        assert_eq!(Vector { x: 3, y: 1 }.angle(), 71_565_056);
        assert_eq!(Vector { x: -2, y: 9 }.angle(), 192_528_816);
    }

    #[test]
    fn test_part1_small() {
        let input: &'static str = " ......#.#.
                                    #..#.#....
                                    ..#######.
                                    .#.#.###..
                                    .#..#.....
                                    ..#....#.#
                                    #..#....#.
                                    .##.#..###
                                    ##...#..#.
                                    .#....####";

        let best_location = Location { x: 5, y: 8 };

        let locations = parse_locations(input);
        let unique_vectors = find_asteroids(&locations, &best_location);
        let (n_asteroids, found_location) = part1(input.clone());

        assert_eq!(unique_vectors.len(), 33);
        assert_eq!(n_asteroids, 33);
        assert_eq!(best_location, found_location);
    }

    #[test]
    fn test_part1_small_2() {
        let input: &'static str = "\
            #.#...#.#.
            .###....#.
            .#....#...
            ##.#.#.#.#
            ....#.#.#.
            .##..###.#
            ..#...##..
            ..##....##
            ......#...
            .####.###.";

        let best_location = Location { x: 1, y: 2 };

        let locations = parse_locations(input);
        let unique_vectors = find_asteroids(&locations, &best_location);
        let (n_asteroids, found_location) = part1(input.clone());

        assert_eq!(unique_vectors.len(), 35);
        assert_eq!(n_asteroids, 35);
        assert_eq!(best_location, found_location);
    }

    #[test]
    fn test_part1_small_3() {
        let input: &'static str = "\
            .#..#..###
            ####.###.#
            ....###.#.
            ..###.##.#
            ##.##.#.#.
            ....###..#
            ..#.#..#.#
            #..#.#.###
            .##...##.#
            .....#.#..";

        let best_location = Location { x: 6, y: 3 };

        let locations = parse_locations(input);
        let unique_vectors = find_asteroids(&locations, &best_location);
        let (n_asteroids, found_location) = part1(input.clone());

        assert_eq!(unique_vectors.len(), 41);
        assert_eq!(n_asteroids, 41);
        assert_eq!(best_location, found_location);
    }

    #[test]
    fn test_part1_medium() {
        let input: &'static str = "\
            .#..##.###...#######
            ##.############..##.
            .#.######.########.#
            .###.#######.####.#.
            #####.##.#.##.###.##
            ..#####..#.#########
            ####################
            #.####....###.#.#.##
            ##.#################
            #####.##.###..####..
            ..######..##.#######
            ####.##.####...##..#
            .#####..#.######.###
            ##...#.##########...
            #.##########.#######
            .####.#.###.###.#.##
            ....##.##.###..#####
            .#.#.###########.###
            #.#.#.#####.####.###
            ###.##.####.##.#..##";


        let best_location = Location { x: 11, y: 13 };

        let locations = parse_locations(input);
        let unique_vectors = find_asteroids(&locations, &best_location);
        let (n_asteroids, found_location) = part1(input.clone());

        assert_eq!(unique_vectors.len(), 210);
        assert_eq!(n_asteroids, 210);
        assert_eq!(best_location, found_location);
    }

    #[test]
    fn test_location_parsing() {
        let input: &'static str = " ......#.#.
                                    #..#.#....
                                    ..#######.
                                    .#.#.###..
                                    .#..#.....
                                    ..#....#.#
                                    #..#....#.
                                    .##.#..###
                                    ##...#..#.
                                    .#....####";

        let locations = parse_locations(input);
        assert_eq!(locations.len(), 40);
    }

    #[test]
    fn test_line_of_sight() {
        let input: &'static str = " ......#.#.
                                    #..#.#....
                                    ..#######.
                                    .#.#.###..
                                    .#..#.....
                                    ..#....#.#
                                    #..#....#.
                                    .##.#..###
                                    ##...#..#.
                                    .#....####";

        let locations = parse_locations(input);
        let l = Location { x: 5, y: 8 };
        let n_asteroids = find_los_asteroids(&locations, &l);
        assert_eq!(n_asteroids, 33);
    }
}
