fn main() {
    let output = part1();
    println!("Part 1: {:?}", output);
    let output = part2();
    println!("Part 2: {:?}", output);
}

fn input() -> Vec<isize> {
    vec![
        3, 225, 1, 225, 6, 6, 1100, 1, 238, 225, 104, 0, 2, 218, 57, 224, 101, -3828, 224, 224, 4, 224, 102, 8, 223, 223, 1001, 224, 2, 224, 1, 223, 224, 223, 1102, 26, 25, 224, 1001, 224, -650, 224, 4, 224, 1002, 223, 8, 223, 101, 7, 224, 224, 1, 223, 224, 223, 1102, 44, 37, 225, 1102, 51, 26, 225, 1102, 70, 94, 225, 1002, 188, 7, 224, 1001, 224, -70, 224, 4, 224, 1002, 223, 8, 223, 1001, 224, 1, 224, 1, 223, 224, 223, 1101, 86, 70, 225, 1101, 80, 25, 224, 101, -105, 224, 224, 4, 224, 102, 8, 223, 223, 101, 1, 224, 224, 1, 224, 223, 223, 101, 6, 91, 224, 1001, 224, -92, 224, 4, 224, 102, 8, 223, 223, 101, 6, 224, 224, 1, 224, 223, 223, 1102, 61, 60, 225, 1001, 139, 81, 224, 101, -142, 224, 224, 4, 224, 102, 8, 223, 223, 101, 1, 224, 224, 1, 223, 224, 223, 102, 40, 65, 224, 1001, 224, -2800, 224, 4, 224, 1002, 223, 8, 223, 1001, 224, 3, 224, 1, 224, 223, 223, 1102, 72, 10, 225, 1101, 71, 21, 225, 1, 62, 192, 224, 1001, 224, -47, 224, 4, 224, 1002, 223, 8, 223, 101, 7, 224, 224, 1, 224, 223, 223, 1101, 76, 87, 225, 4, 223, 99, 0, 0, 0, 677, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1105, 0, 99999, 1105, 227, 247, 1105, 1, 99999, 1005, 227, 99999, 1005, 0, 256, 1105, 1, 99999, 1106, 227, 99999, 1106, 0, 265, 1105, 1, 99999, 1006, 0, 99999, 1006, 227, 274, 1105, 1, 99999, 1105, 1, 280, 1105, 1, 99999, 1, 225, 225, 225, 1101, 294, 0, 0, 105, 1, 0, 1105, 1, 99999, 1106, 0, 300, 1105, 1, 99999, 1, 225, 225, 225, 1101, 314, 0, 0, 106, 0, 0, 1105, 1, 99999, 108, 226, 677, 224, 102, 2, 223, 223, 1005, 224, 329, 1001, 223, 1, 223, 1107, 677, 226, 224, 102, 2, 223, 223, 1006, 224, 344, 1001, 223, 1, 223, 7, 226, 677, 224, 1002, 223, 2, 223, 1005, 224, 359, 101, 1, 223, 223, 1007, 226, 226, 224, 102, 2, 223, 223, 1005, 224, 374, 101, 1, 223, 223, 108, 677, 677, 224, 102, 2, 223, 223, 1006, 224, 389, 1001, 223, 1, 223, 107, 677, 226, 224, 102, 2, 223, 223, 1006, 224, 404, 101, 1, 223, 223, 1108, 677, 226, 224, 102, 2, 223, 223, 1006, 224, 419, 1001, 223, 1, 223, 1107, 677, 677, 224, 1002, 223, 2, 223, 1006, 224, 434, 101, 1, 223, 223, 1007, 677, 677, 224, 102, 2, 223, 223, 1006, 224, 449, 1001, 223, 1, 223, 1108, 226, 677, 224, 1002, 223, 2, 223, 1006, 224, 464, 101, 1, 223, 223, 7, 677, 226, 224, 102, 2, 223, 223, 1006, 224, 479, 101, 1, 223, 223, 1008, 226, 226, 224, 102, 2, 223, 223, 1006, 224, 494, 101, 1, 223, 223, 1008, 226, 677, 224, 1002, 223, 2, 223, 1005, 224, 509, 1001, 223, 1, 223, 1007, 677, 226, 224, 102, 2, 223, 223, 1005, 224, 524, 1001, 223, 1, 223, 8, 226, 226, 224, 102, 2, 223, 223, 1006, 224, 539, 101, 1, 223, 223, 1108, 226, 226, 224, 1002, 223, 2, 223, 1006, 224, 554, 101, 1, 223, 223, 107, 226, 226, 224, 1002, 223, 2, 223, 1005, 224, 569, 1001, 223, 1, 223, 7, 226, 226, 224, 102, 2, 223, 223, 1005, 224, 584, 101, 1, 223, 223, 1008, 677, 677, 224, 1002, 223, 2, 223, 1006, 224, 599, 1001, 223, 1, 223, 8, 226, 677, 224, 1002, 223, 2, 223, 1006, 224, 614, 1001, 223, 1, 223, 108, 226, 226, 224, 1002, 223, 2, 223, 1006, 224, 629, 101, 1, 223, 223, 107, 677, 677, 224, 102, 2, 223, 223, 1005, 224, 644, 1001, 223, 1, 223, 8, 677, 226, 224, 1002, 223, 2, 223, 1005, 224, 659, 1001, 223, 1, 223, 1107, 226, 677, 224, 102, 2, 223, 223, 1005, 224, 674, 1001, 223, 1, 223, 4, 223, 99, 226
    ]
}


#[derive(Clone, Debug, PartialEq)]
enum Parameter {
    Value { value: isize },
    Reference { address: usize },
}

impl Parameter {
    fn eval(&self, memory: &Memory) -> isize {
        match self {
            Parameter::Value { value: v } => v.clone(),
            Parameter::Reference { address: a } => memory.get(*a).unwrap().clone()
        }
    }
}


#[derive(Clone, Debug, PartialEq)]
enum Operation {
    Add(Parameter, Parameter, Parameter),
    Mul(Parameter, Parameter, Parameter),
    LessThan(Parameter, Parameter, Parameter),
    Equal(Parameter, Parameter, Parameter),
    JumpTrue(Parameter, Parameter),
    JumpFalse(Parameter, Parameter),
    Input(Parameter),
    Output(Parameter),
    NoOp,
    Halt,
}

impl Operation {
    fn size(&self) -> usize {
        match self {
            Self::Add(..) => 4,
            Self::Mul(..) => 4,
            Self::LessThan(..) => 4,
            Self::Equal(..) => 4,
            Self::JumpTrue(..) => 3,
            Self::JumpFalse(..) => 3,
            Self::Input(..) => 2,
            Self::Output(..) => 2,
            Self::NoOp => 1,
            Self::Halt => 1
        }
    }
}


type Memory = Vec<isize>;
type Output = Vec<String>;

#[derive(Clone, Eq, PartialEq, Debug)]
enum Mode {
    Position,
    Immediate,
}

impl Mode {
    fn from_isize(input: isize) -> Self {
        match input {
            0 => Mode::Position,
            1 => Mode::Immediate,
            _ => unreachable!()
        }
    }
}

type ModeSet = (Mode, Mode, Mode);

#[derive(Debug)]
struct IntCodeComputer {
    memory: Memory,
    input: Memory,
    output: Memory,
    pc: usize,
}

impl IntCodeComputer {
    fn new(memory: Memory, input: Memory) -> Self {
        IntCodeComputer { memory, input, output: vec![], pc: 0 }
    }

    fn tick(&mut self) -> State {
        let operation = Self::next_operation(&self.memory, self.pc);
        let new_state = self.execute_command(&operation);

        match new_state {
            State::Running => self.pc += operation.size(),
            _ => ()
        }

        new_state
    }

    fn next_operation(memory: &Memory, position: usize) -> Operation {
        let raw_op_code = memory[position];
        let (op_code, mode_set) = Self::decode_opcode(raw_op_code);

        match op_code {
            1 => {
                Operation::Add(
                    Self::get_parameter_for_mode(&memory, position + 1, mode_set.0),
                    Self::get_parameter_for_mode(&memory, position + 2, mode_set.1),
                    Self::get_parameter_for_mode(&memory, position + 3, mode_set.2),
                )
            }
            2 => {
                Operation::Mul(
                    Self::get_parameter_for_mode(&memory, position + 1, mode_set.0),
                    Self::get_parameter_for_mode(&memory, position + 2, mode_set.1),
                    Self::get_parameter_for_mode(&memory, position + 3, mode_set.2),
                )
            }
            3 => {
                Operation::Input(Self::get_parameter_for_mode(&memory, position + 1, mode_set.0))
            }
            4 => {
                Operation::Output(Self::get_parameter_for_mode(&memory, position + 1, mode_set.0))
            }
            5 => {
                Operation::JumpTrue(
                    Self::get_parameter_for_mode(&memory, position + 1, mode_set.0),
                    Self::get_parameter_for_mode(&memory, position + 2, mode_set.1),
                )
            }
            6 => {
                Operation::JumpFalse(
                    Self::get_parameter_for_mode(&memory, position + 1, mode_set.0),
                    Self::get_parameter_for_mode(&memory, position + 2, mode_set.1),
                )
            }
            7 => {
                Operation::LessThan(
                    Self::get_parameter_for_mode(&memory, position + 1, mode_set.0),
                    Self::get_parameter_for_mode(&memory, position + 2, mode_set.1),
                    Self::get_reference(&memory, position + 3),
                )
            }
            8 => {
                Operation::Equal(
                    Self::get_parameter_for_mode(&memory, position + 1, mode_set.0),
                    Self::get_parameter_for_mode(&memory, position + 2, mode_set.1),
                    Self::get_reference(&memory, position + 3),
                )
            }
            99 => Operation::Halt,
            _ => Operation::NoOp
        }
    }

    fn decode_opcode(input: isize) -> (usize, ModeSet) {
        let opcode = input % 100;
        let c = (input / 10_000) % 10;
        let b = (input / 1_000) % 10;
        let a = (input / 100) % 10;

        let mode_a = Mode::from_isize(a);
        let mode_b = Mode::from_isize(b);
        let mode_c = Mode::from_isize(c);

        (
            opcode as usize,
            (mode_a, mode_b, mode_c),
        )
    }

    fn get_parameter_for_mode(program: &Memory, index: usize, mode: Mode) -> Parameter {
        match mode {
            Mode::Position => Self::get_reference(&program, index),
            Mode::Immediate => Self::get_value(&program, index),
        }
    }

    fn get_parameter(program: &Memory, index: usize) -> isize {
        program.get(index).unwrap().clone()
    }

    fn get_reference(program: &Memory, index: usize) -> Parameter {
        Parameter::Reference { address: Self::get_parameter(&program, index) as usize }
    }

    fn get_value(program: &Memory, index: usize) -> Parameter {
        Parameter::Value { value: Self::get_parameter(&program, index) }
    }

    fn run(&mut self) -> Output {
        loop {
            let state = self.tick();
            match state {
                State::Halt => break,
                _ => continue
            }
        }

        self.output.clone().iter()
            .map(|x| format!("{:?}", x))
            .collect()
    }

    fn execute_command(&mut self, operation: &Operation) -> State {
        match operation {
            Operation::Input(Parameter::Reference { address: v }) => {
                let cell = self.memory.get_mut(*v).unwrap();
                *cell = self.input.pop().unwrap();
                State::Running
            }
            Operation::Add(a, b, Parameter::Reference { address: target }) => {
                let x = a.eval(&self.memory);
                let y = b.eval(&self.memory);
                self.memory[*target] = x + y;
                State::Running
            }
            Operation::Mul(a, b, Parameter::Reference { address: target }) => {
                let x = a.eval(&self.memory);
                let y = b.eval(&self.memory);
                self.memory[*target] = x * y;
                State::Running
            }
            Operation::JumpTrue(a, b) => {
                let val = a.eval(&self.memory);
                let pointer = b.eval(&self.memory) as usize;
                if val != 0 {
                    self.pc = pointer as usize;
                    State::Jump
                } else {
                    State::Running
                }
            }
            Operation::JumpFalse(a, b) => {
                let val = a.eval(&self.memory);
                let pointer = b.eval(&self.memory) as usize;
                if val == 0 {
                    self.pc = pointer as usize;
                    State::Jump
                } else {
                    State::Running
                }
            }
            Operation::LessThan(a, b, Parameter::Reference { address: target }) => {
                if a.eval(&self.memory) < b.eval(&self.memory) {
                    self.memory[*target] = 1;
                } else {
                    self.memory[*target] = 0;
                }
                State::Running
            }
            Operation::Equal(a, b, Parameter::Reference { address: target }) => {
                if a.eval(&self.memory) == b.eval(&self.memory) {
                    self.memory[*target] = 1;
                } else {
                    self.memory[*target] = 0;
                }
                State::Running
            }
            Operation::Output(address) => {
                let value = address.eval(&self.memory);
                self.output.push(value);
                State::Running
            }
            Operation::Halt => {
                State::Halt
            }
            Operation::NoOp => State::Running,
            _ => panic!("Unknown Operation")
        }
    }
}

#[derive(Debug, PartialEq)]
enum State {
    Halt,
    Running,
    Jump,
}

fn part1() -> Output {
    let memory = input();
    let mut comp = IntCodeComputer::new(memory, vec![1]);
    comp.run()
}

fn part2() -> Output {
    let memory = input();
    let mut comp = IntCodeComputer::new(memory, vec![5]);
    comp.run()
}


#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_parse_instruction_code() {
        assert_eq!(IntCodeComputer::decode_opcode(102).1, (Mode::Immediate, Mode::Position, Mode::Position));
        assert_eq!(IntCodeComputer::decode_opcode(01).1, (Mode::Position, Mode::Position, Mode::Position));
        assert_eq!(IntCodeComputer::decode_opcode(11101).1, (Mode::Immediate, Mode::Immediate, Mode::Immediate));
    }

    #[test]
    fn test_lt_eq_operations() {
        let programs: Vec<Memory> = vec![
            vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8],
            vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8],
            vec![3, 3, 1108, -1, 8, 3, 4, 3, 99],
            vec![3, 3, 1107, -1, 8, 3, 4, 3, 99],
        ];

        let specs: Vec<Vec<(Memory, Output)>> = vec![
            vec![
                (vec![8], vec![String::from("1")]),
                (vec![9], vec![String::from("0")]),
            ],
            vec![
                (vec![7], vec![String::from("1")]),
                (vec![9], vec![String::from("0")]),
            ],
            vec![
                (vec![8], vec![String::from("1")]),
                (vec![9], vec![String::from("0")]),
            ],
            vec![
                (vec![7], vec![String::from("1")]),
                (vec![9], vec![String::from("0")]),
            ],
        ];

        programs.iter().zip(&specs).for_each(|(p, specs)| {
            for spec in specs {
                let mut comp = IntCodeComputer::new(p.clone(), spec.0.clone());
                let output = comp.run();
                assert_eq!(output, spec.1);
            }
        });
    }

    #[test]
    fn test_jump_operations() {
        let programs: Vec<Memory> = vec![
            vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
            vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
        ];

        let specs = vec![
            (vec![0], vec![String::from("0")]),
            (vec![1], vec![String::from("1")]),
            (vec![5], vec![String::from("1")]),
        ];

        programs.iter()
            .zip(&specs)
            .for_each(|(p, spec)| {
                let mut comp = IntCodeComputer::new(p.clone(), spec.0.clone());
                let output = comp.run();
                assert_eq!(output, spec.1);
            });
    }

//    #[test]
//    fn test_larger_program() {
//        let memory = vec![
//            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31,
//            1106, 0, 36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104,
//            999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99
//        ];
//        let input: Memory = vec![100];
//        let output = vec!["1000".to_string()];
//
//        let mut comp = IntCodeComputer::new(memory, input);
//        assert_eq!(output, comp.run());
//    }
}


