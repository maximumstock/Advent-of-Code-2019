use intcode::{IntCodeComputer, Memory, MemoryValue, State};
use std::collections::HashSet;
use std::fmt::{Debug, Display, Error, Formatter};
use std::ops::Add;

fn main() {
    let program = input();
    let mut robot = Robot::new(program);
    robot.run();
    let n_unique_panels_painted = robot.unique_panels.len();
    println!("Painted {} unique panels", n_unique_panels_painted);
}

#[derive(Copy, Clone, Debug)]
enum Panel {
    Black,
    White,
}

impl Display for Panel {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Panel::Black => write!(f, "."),
            Panel::White => write!(f, "#"),
        }
    }
}

impl Panel {
    fn to_input(&self) -> MemoryValue {
        match self {
            Panel::Black => 0,
            Panel::White => 1,
        }
    }

    fn from_isize(input: isize) -> Self {
        match input {
            0 => Panel::Black,
            1 => Panel::White,
            _ => unreachable!(),
        }
    }
}

impl Default for Panel {
    fn default() -> Self {
        Panel::Black
    }
}

#[derive(Debug)]
struct Grid<T: Debug> {
    grid: Vec<T>,
    grid_size: (usize, usize),
    offset: usize,
}

impl<T: Display + Debug> Display for Grid<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let mut output = String::from("");
        let (x, y) = self.grid_size;

        for row in (0..y).into_iter() {
            for col in (0..x).into_iter() {
                let item = self.grid.get(col * y + row).unwrap();
                output = output.add(format!("{}", item).as_str());
            }
            output = output.add("\n");
        }

        write!(f, "\n{}", output)
    }
}

impl<T: Default + Debug> Grid<T> {
    fn new(width: usize, height: usize, x_offset: usize, y_offset: usize) -> Grid<T> {
        let mut data = Vec::with_capacity(width * height);

        for _ in 0..width * height {
            data.insert(0, T::default())
        }

        Grid {
            grid: data,
            grid_size: (width, height),
            offset: y_offset * height + x_offset,
        }
    }
}

impl<T: Debug> Grid<T> {
    fn get(&self, x: isize, y: isize) -> Result<&T, ()> {
        let (_, dim_y) = self.grid_size;
        let index = ((y * dim_y as isize + x) + self.offset as isize) as usize;
        let item = self.grid.get(index).unwrap();
        Ok(item)
    }

    fn set(&mut self, x: isize, y: isize, item: T) -> Result<(), ()> {
        let (_, dim_y) = self.grid_size;
        let index = ((y * dim_y as isize + x) + self.offset as isize) as usize;
        let old_item = self.grid.get_mut(index).unwrap();
        *old_item = item;
        Ok(())
    }
}

struct Robot {
    cpu: intcode::IntCodeComputer,
    grid: Grid<Panel>,
    unique_panels: HashSet<(isize, isize)>,
    position: (isize, isize),
    direction: i32,
}

impl Robot {
    fn new(memory: Memory) -> Self {
        Robot {
            cpu: IntCodeComputer::new(memory),
            grid: Grid::new(100, 100, 40, 40),
            unique_panels: HashSet::new(),
            position: (0, 0),
            direction: 0,
        }
    }

    fn paint(&mut self, color_to_paint: MemoryValue) {
        let (x, y) = self.position;
        self.grid
            .set(x, y, Panel::from_isize(color_to_paint))
            .unwrap();
        self.unique_panels.insert((x, y));
    }

    fn turn(&mut self, direction: MemoryValue) {
        match direction {
            0 => self.direction = (self.direction - 90) % 360,
            1 => self.direction = (self.direction + 90) % 360,
            _ => unreachable!(),
        }
    }

    fn step(&mut self) {
        let (x, y) = self.position;
        match self.direction {
            0 => self.position = (x, y + 1),
            -90 | 270 => self.position = (x - 1, y),
            90 | -270 => self.position = (x + 1, y),
            180 | -180 => self.position = (x, y - 1),
            _ => unreachable!(),
        }
    }

    fn run(&mut self) {
        let mut waiting_for_output = true;
        self.grid.set(0, 0, Panel::White).unwrap();
        loop {
            let new_state = self.cpu.step();
            match new_state {
                State::WaitingForInput => {
                    let (x, y) = self.position;
                    let current_panel = self.grid.get(x, y).unwrap().clone();
                    self.cpu.read_input(current_panel.to_input());
                }
                State::Output(_) => {
                    if waiting_for_output == true {
                        waiting_for_output = false;
                        continue;
                    } else {
                        waiting_for_output = true;
                        let mut output = self.cpu.get_output();
                        let direction = output.pop().unwrap();
                        let color_to_paint = output.pop().unwrap();
                        self.paint(color_to_paint);
                        self.turn(direction);
                        self.step();
                    }
                }
                State::Halt => break,
                _ => (),
            }
        }
        println!("{}", self.grid);
    }
}

fn input() -> Memory {
    vec![
        3,
        8,
        1005,
        8,
        321,
        1106,
        0,
        11,
        0,
        0,
        0,
        104,
        1,
        104,
        0,
        3,
        8,
        102,
        -1,
        8,
        10,
        1001,
        10,
        1,
        10,
        4,
        10,
        1008,
        8,
        1,
        10,
        4,
        10,
        1002,
        8,
        1,
        29,
        3,
        8,
        1002,
        8,
        -1,
        10,
        101,
        1,
        10,
        10,
        4,
        10,
        108,
        0,
        8,
        10,
        4,
        10,
        1002,
        8,
        1,
        50,
        3,
        8,
        102,
        -1,
        8,
        10,
        1001,
        10,
        1,
        10,
        4,
        10,
        1008,
        8,
        0,
        10,
        4,
        10,
        1001,
        8,
        0,
        73,
        1,
        1105,
        16,
        10,
        2,
        1004,
        8,
        10,
        3,
        8,
        1002,
        8,
        -1,
        10,
        1001,
        10,
        1,
        10,
        4,
        10,
        1008,
        8,
        0,
        10,
        4,
        10,
        1002,
        8,
        1,
        103,
        1006,
        0,
        18,
        1,
        105,
        14,
        10,
        3,
        8,
        102,
        -1,
        8,
        10,
        101,
        1,
        10,
        10,
        4,
        10,
        108,
        0,
        8,
        10,
        4,
        10,
        102,
        1,
        8,
        131,
        1006,
        0,
        85,
        1,
        1008,
        0,
        10,
        1006,
        0,
        55,
        2,
        104,
        4,
        10,
        3,
        8,
        102,
        -1,
        8,
        10,
        1001,
        10,
        1,
        10,
        4,
        10,
        1008,
        8,
        1,
        10,
        4,
        10,
        1001,
        8,
        0,
        168,
        2,
        1101,
        1,
        10,
        1006,
        0,
        14,
        3,
        8,
        102,
        -1,
        8,
        10,
        101,
        1,
        10,
        10,
        4,
        10,
        108,
        1,
        8,
        10,
        4,
        10,
        102,
        1,
        8,
        196,
        1006,
        0,
        87,
        1006,
        0,
        9,
        1,
        102,
        20,
        10,
        3,
        8,
        1002,
        8,
        -1,
        10,
        101,
        1,
        10,
        10,
        4,
        10,
        108,
        1,
        8,
        10,
        4,
        10,
        1001,
        8,
        0,
        228,
        3,
        8,
        1002,
        8,
        -1,
        10,
        101,
        1,
        10,
        10,
        4,
        10,
        108,
        0,
        8,
        10,
        4,
        10,
        1002,
        8,
        1,
        250,
        2,
        5,
        0,
        10,
        2,
        1009,
        9,
        10,
        2,
        107,
        17,
        10,
        1006,
        0,
        42,
        3,
        8,
        102,
        -1,
        8,
        10,
        101,
        1,
        10,
        10,
        4,
        10,
        108,
        1,
        8,
        10,
        4,
        10,
        1001,
        8,
        0,
        287,
        2,
        102,
        8,
        10,
        1006,
        0,
        73,
        1006,
        0,
        88,
        1006,
        0,
        21,
        101,
        1,
        9,
        9,
        1007,
        9,
        925,
        10,
        1005,
        10,
        15,
        99,
        109,
        643,
        104,
        0,
        104,
        1,
        21102,
        1,
        387353256856,
        1,
        21101,
        0,
        338,
        0,
        1105,
        1,
        442,
        21101,
        936332866452,
        0,
        1,
        21101,
        349,
        0,
        0,
        1105,
        1,
        442,
        3,
        10,
        104,
        0,
        104,
        1,
        3,
        10,
        104,
        0,
        104,
        0,
        3,
        10,
        104,
        0,
        104,
        1,
        3,
        10,
        104,
        0,
        104,
        1,
        3,
        10,
        104,
        0,
        104,
        0,
        3,
        10,
        104,
        0,
        104,
        1,
        21101,
        0,
        179357024347,
        1,
        21101,
        0,
        396,
        0,
        1105,
        1,
        442,
        21102,
        1,
        29166144659,
        1,
        21102,
        407,
        1,
        0,
        1105,
        1,
        442,
        3,
        10,
        104,
        0,
        104,
        0,
        3,
        10,
        104,
        0,
        104,
        0,
        21102,
        1,
        718170641252,
        1,
        21102,
        430,
        1,
        0,
        1106,
        0,
        442,
        21101,
        825012151040,
        0,
        1,
        21102,
        441,
        1,
        0,
        1106,
        0,
        442,
        99,
        109,
        2,
        21202,
        -1,
        1,
        1,
        21102,
        1,
        40,
        2,
        21102,
        1,
        473,
        3,
        21102,
        463,
        1,
        0,
        1105,
        1,
        506,
        109,
        -2,
        2106,
        0,
        0,
        0,
        1,
        0,
        0,
        1,
        109,
        2,
        3,
        10,
        204,
        -1,
        1001,
        468,
        469,
        484,
        4,
        0,
        1001,
        468,
        1,
        468,
        108,
        4,
        468,
        10,
        1006,
        10,
        500,
        1102,
        1,
        0,
        468,
        109,
        -2,
        2105,
        1,
        0,
        0,
        109,
        4,
        1202,
        -1,
        1,
        505,
        1207,
        -3,
        0,
        10,
        1006,
        10,
        523,
        21101,
        0,
        0,
        -3,
        22101,
        0,
        -3,
        1,
        21202,
        -2,
        1,
        2,
        21102,
        1,
        1,
        3,
        21102,
        1,
        542,
        0,
        1105,
        1,
        547,
        109,
        -4,
        2106,
        0,
        0,
        109,
        5,
        1207,
        -3,
        1,
        10,
        1006,
        10,
        570,
        2207,
        -4,
        -2,
        10,
        1006,
        10,
        570,
        22102,
        1,
        -4,
        -4,
        1105,
        1,
        638,
        22102,
        1,
        -4,
        1,
        21201,
        -3,
        -1,
        2,
        21202,
        -2,
        2,
        3,
        21101,
        0,
        589,
        0,
        1106,
        0,
        547,
        22102,
        1,
        1,
        -4,
        21101,
        1,
        0,
        -1,
        2207,
        -4,
        -2,
        10,
        1006,
        10,
        608,
        21102,
        0,
        1,
        -1,
        22202,
        -2,
        -1,
        -2,
        2107,
        0,
        -3,
        10,
        1006,
        10,
        630,
        21202,
        -1,
        1,
        1,
        21102,
        630,
        1,
        0,
        105,
        1,
        505,
        21202,
        -2,
        -1,
        -2,
        22201,
        -4,
        -2,
        -4,
        109,
        -5,
        2106,
        0,
        0,
    ]
}

#[cfg(test)]
mod tests {
    use crate::Grid;

    #[test]
    fn test_simple_grid() {
        let mut grid: Grid<bool> = Grid::new(5, 5, 0, 0);
        assert_eq!(&false, grid.get(2, 3).unwrap());
        assert_eq!(&false, grid.get(0, 0).unwrap());
        assert_eq!(&false, grid.get(4, 4).unwrap());

        grid.set(2, 2, true).unwrap();
        assert_eq!(&true, grid.get(2, 2).unwrap());
    }

    #[test]
    fn test_offset_grid() {
        let mut grid: Grid<bool> = Grid::new(5, 5, 2, 2);
        assert_eq!(&false, grid.get(-1, -1).unwrap());
        assert_eq!(&false, grid.get(-2, -2).unwrap());

        grid.set(-2, -2, true).unwrap();
        assert_eq!(&true, grid.get(-2, -2).unwrap());
    }
}
