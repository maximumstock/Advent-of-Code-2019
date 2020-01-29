use std::fmt::{Display, Formatter, Error};
use std::ops::Add;
use std::cmp::Ordering;

fn main() {
    part1(input());
    part2(input());
}

fn part1(system: OrbitalSystem) {
    let mut system = system;
    for _ in 0..1000 {
        system.tick();
    }
    println!("Total energy is: {:?}", system.total_energy());
}

fn part2(initial_system: OrbitalSystem) {
    let system_x = OrbitalSystem {
        moons: initial_system.moons.iter().map(|m| {
            Satellite::new(Vector::new(m.position.x, 0, 0))
        }).collect()
    };

    let system_y = OrbitalSystem {
        moons: initial_system.moons.iter().map(|m| {
            Satellite::new(Vector::new(0, m.position.y, 0))
        }).collect()
    };

    let system_z = OrbitalSystem {
        moons: initial_system.moons.iter().map(|m| {
            Satellite::new(Vector::new(0, 0, m.position.z))
        }).collect()
    };

    let x_steps = find_steps_for_axis(system_x);
    let y_steps = find_steps_for_axis(system_y);
    let z_steps = find_steps_for_axis(system_z);

    println!("Done: kgV({}, {}, {})", x_steps, y_steps, z_steps);
}

fn find_steps_for_axis(mut system: OrbitalSystem) -> usize {
    let mut steps = 0 as usize;

    let initial_state = system.clone();

    loop {
        steps += 1;
        system.tick();

        if system.eq(&initial_state) {
            break;
        }
    }
    steps
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Vector {
    x: i32,
    y: i32,
    z: i32,
}

impl Vector {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Vector { x, y, z }
    }
}

impl Default for Vector {
    fn default() -> Self {
        Vector { x: 0, y: 0, z: 0 }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Satellite {
    position: Vector,
    velocity: Vector,
}

impl Display for Satellite {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let positions = format!("pos=<x=\t{}, y=\t{}, z=\t{}", self.position.x, self.position.y, self.position.z);
        let velocities = format!("vel=<x=\t{}, y=\t{}, z=\t{}", self.velocity.x, self.velocity.y, self.velocity.z);
        write!(f, "{}, {}", positions, velocities)
    }
}

impl Satellite {
    fn new(position: Vector) -> Self {
        Satellite {
            position,
            velocity: Vector::default(),
        }
    }

    fn kinetic_energy(&self) -> usize {
        (self.velocity.x.abs() + self.velocity.y.abs() + self.velocity.z.abs()) as usize
    }

    fn potential_energy(&self) -> usize {
        (self.position.x.abs() + self.position.y.abs() + self.position.z.abs()) as usize
    }

    fn total_energy(&self) -> usize {
        self.kinetic_energy() * self.potential_energy()
    }

    fn compare_axes(sat: i32, other: i32) -> i32 {
        match sat.cmp(&other) {
            Ordering::Equal => 0,
            Ordering::Greater => -1,
            Ordering::Less => 1
        }
    }

    fn apply_gravity(&mut self, other: &Satellite) {
        self.velocity.x = self.velocity.x + Satellite::compare_axes(self.position.x, other.position.x);
        self.velocity.y = self.velocity.y + Satellite::compare_axes(self.position.y, other.position.y);
        self.velocity.z = self.velocity.z + Satellite::compare_axes(self.position.z, other.position.z);
    }

    fn apply_velocity(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
        self.position.z += self.velocity.z;
    }
}

#[derive(Hash, Eq, PartialEq, Clone)]
struct OrbitalSystem {
    moons: Vec<Satellite>
}

impl Display for OrbitalSystem {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let mut output = String::new();
        for moon in &self.moons {
            output = output.add(format!("{}\n", moon).as_ref());
        }
        write!(f, "{}", output)
    }
}

impl OrbitalSystem {
    fn total_energy(&self) -> usize {
        self.moons.iter()
            .map(|moon| moon.total_energy())
            .sum()
    }

    fn tick(&mut self) {
        let mut idx_pairs = vec![];
        for i in 0..self.moons.len() {
            for k in 0..self.moons.len() {
                if k != i {
                    idx_pairs.push((i, k))
                }
            }
        }

        for (left_idx, right_idx) in idx_pairs {
            let mut left = self.moons.get(left_idx).unwrap().clone();
            let right = self.moons.get(right_idx).unwrap();
            left.apply_gravity(right);
            self.moons.remove(left_idx);
            self.moons.insert(left_idx, left);
        }

        for moon in &mut self.moons {
            moon.apply_velocity()
        }
    }
}

fn input() -> OrbitalSystem {
    OrbitalSystem {
        moons: vec![
            Satellite::new(Vector::new(-15, 1, 4)),
            Satellite::new(Vector::new(1, -10, -8)),
            Satellite::new(Vector::new(-5, 4, 9)),
            Satellite::new(Vector::new(4, 6, -2)),
        ]
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_energy() {
        let mut system = OrbitalSystem {
            moons: vec![
                Satellite::new(Vector::new(-1, 0, 2)),
                Satellite::new(Vector::new(2, -10, -7)),
                Satellite::new(Vector::new(4, -8, 8)),
                Satellite::new(Vector::new(3, 5, -1)),
            ]
        };

        println!("{}", system);
        for _ in 0..10 {
            system.tick();
            println!("{}", system);
        }

        assert_eq!(system.total_energy(), 179);
    }
}
