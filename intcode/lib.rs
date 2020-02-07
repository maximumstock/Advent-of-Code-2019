use crate::Parameter::Reference;

#[derive(Clone, Debug, PartialEq)]
pub enum Parameter {
    Value { value: MemoryValue },
    Reference { address: MemoryIndex },
}

impl Parameter {
    fn eval(&self, memory: &Memory) -> MemoryValue {
        //        println!("{:?}", self);
        match self {
            Parameter::Value { value: v } => v.clone(),
            Parameter::Reference { address: a } => memory.get(*a).unwrap().clone(),
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
    AdjustRelativeBase(Parameter),
    Halt,
}

impl Operation {
    fn size(&self) -> u8 {
        match self {
            Self::Add(..) => 4,
            Self::Mul(..) => 4,
            Self::LessThan(..) => 4,
            Self::Equal(..) => 4,
            Self::JumpTrue(..) => 3,
            Self::JumpFalse(..) => 3,
            Self::Input(..) => 2,
            Self::Output(..) => 2,
            Self::AdjustRelativeBase(..) => 2,
            Self::Halt => 1,
        }
    }
}

pub type MemoryValue = isize;
pub type MemoryIndex = usize;
pub type Memory = Vec<MemoryValue>;
pub type Output = Vec<String>;

#[derive(Clone, Eq, PartialEq, Debug)]
enum Mode {
    Position,
    Immediate,
    Relative,
}

impl Mode {
    fn from_isize(input: MemoryValue) -> Self {
        match input {
            0 => Mode::Position,
            1 => Mode::Immediate,
            2 => Mode::Relative,
            _ => unreachable!(),
        }
    }
}

type ModeSet = (Mode, Mode, Mode);

#[derive(Debug, Clone)]
pub struct IntCodeComputer {
    memory: Memory,
    input: Memory,
    output: Memory,
    pc: MemoryIndex,
    relative_base: MemoryIndex,
}

impl IntCodeComputer {
    pub fn new(memory: Memory) -> Self {
        let len = memory.len();
        let mut cpu = IntCodeComputer {
            memory,
            input: vec![],
            output: vec![],
            pc: 0,
            relative_base: 0,
        };
        cpu.memory.extend((0..len * 100).into_iter().map(|_| 0));
        cpu
    }

    pub fn get_memory(&self) -> Memory {
        self.memory.clone()
    }

    pub fn get_output(&self) -> Memory {
        self.output.clone()
    }

    fn tick(&mut self) -> State {
        let operation = self.next_operation();
        let new_state = self.execute_command(&operation);

        match new_state {
            State::Jump | State::Halt | State::WaitingForInput => (),
            _ => self.pc += operation.size() as usize,
        }

        new_state
    }

    fn next_operation(&self) -> Operation {
        let position = self.pc;
        let raw_op_code = self.memory[position];
        let (op_code, mode_set) = Self::decode_opcode(raw_op_code);

        match op_code {
            1 => Operation::Add(
                self.get_parameter_for_mode(position + 1, mode_set.0),
                self.get_parameter_for_mode(position + 2, mode_set.1),
                self.get_parameter_for_mode(position + 3, mode_set.2),
            ),
            2 => Operation::Mul(
                self.get_parameter_for_mode(position + 1, mode_set.0),
                self.get_parameter_for_mode(position + 2, mode_set.1),
                self.get_parameter_for_mode(position + 3, mode_set.2),
            ),
            3 => Operation::Input(self.get_parameter_for_mode(position + 1, mode_set.0)),
            4 => Operation::Output(self.get_parameter_for_mode(position + 1, mode_set.0)),
            5 => Operation::JumpTrue(
                self.get_parameter_for_mode(position + 1, mode_set.0),
                self.get_parameter_for_mode(position + 2, mode_set.1),
            ),
            6 => Operation::JumpFalse(
                self.get_parameter_for_mode(position + 1, mode_set.0),
                self.get_parameter_for_mode(position + 2, mode_set.1),
            ),
            7 => Operation::LessThan(
                self.get_parameter_for_mode(position + 1, mode_set.0),
                self.get_parameter_for_mode(position + 2, mode_set.1),
                self.get_parameter_for_mode(position + 3, mode_set.2),
            ),
            8 => Operation::Equal(
                self.get_parameter_for_mode(position + 1, mode_set.0),
                self.get_parameter_for_mode(position + 2, mode_set.1),
                self.get_parameter_for_mode(position + 3, mode_set.2),
            ),
            9 => {
                Operation::AdjustRelativeBase(self.get_parameter_for_mode(position + 1, mode_set.0))
            }
            99 => Operation::Halt,
            _ => unreachable!(),
        }
    }

    fn decode_opcode(input: MemoryValue) -> (u32, ModeSet) {
        let opcode = input % 100;
        let c = (input / 10_000) % 10;
        let b = (input / 1_000) % 10;
        let a = (input / 100) % 10;

        let mode_a = Mode::from_isize(a);
        let mode_b = Mode::from_isize(b);
        let mode_c = Mode::from_isize(c);

        (opcode as u32, (mode_a, mode_b, mode_c))
    }

    fn get_parameter_for_mode(&self, index: MemoryIndex, mode: Mode) -> Parameter {
        match mode {
            Mode::Position => Self::get_reference(&self, index),
            Mode::Immediate => Self::get_value(&self, index),
            Mode::Relative => {
                let value_at_index = Self::get_parameter(&self, index);
                let relative_reference_address =
                    (self.relative_base as isize + value_at_index) as usize;
                Reference {
                    address: relative_reference_address,
                }
            }
        }
    }

    fn get_parameter(&self, index: MemoryIndex) -> MemoryValue {
        self.memory.get(index as usize).unwrap().clone()
    }

    fn get_reference(&self, index: MemoryIndex) -> Parameter {
        Parameter::Reference {
            address: self.get_parameter(index) as usize,
        }
    }

    fn get_value(&self, index: MemoryIndex) -> Parameter {
        Parameter::Value {
            value: self.get_parameter(index),
        }
    }

    pub fn run(&mut self, input: Memory) -> Output {
        self.input = input.clone();
        loop {
            let state = self.tick();
            match state {
                State::Halt => break,
                _ => continue,
            }
        }

        self.output
            .clone()
            .iter()
            .map(|x| format!("{:?}", x))
            .collect()
    }

    pub fn step(&mut self) -> State {
        self.tick()
    }

    pub fn read_input(&mut self, input: MemoryValue) {
        self.input.push(input);
    }

    fn execute_command(&mut self, operation: &Operation) -> State {
        match operation {
            Operation::Input(Parameter::Reference { address: v }) => match self.input.pop() {
                Some(value) => {
                    let cell = self.memory.get_mut(*v).unwrap();
                    *cell = value;
                    State::Running
                }
                None => State::WaitingForInput,
            },
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
                let pointer = b.eval(&self.memory) as MemoryIndex;
                if val != 0 {
                    self.pc = pointer;
                    State::Jump
                } else {
                    State::Running
                }
            }
            Operation::JumpFalse(a, b) => {
                let val = a.eval(&self.memory);
                let pointer = b.eval(&self.memory) as MemoryIndex;
                if val == 0 {
                    self.pc = pointer;
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
                self.output.push(value.clone());
                State::Output(value)
            }
            Operation::Halt => State::Halt,
            Operation::AdjustRelativeBase(a) => {
                let value = a.eval(&self.memory);
                self.relative_base = (self.relative_base as isize + value) as usize;
                State::Running
            }
            x => panic!("Unknown Operation: {:?}", x),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum State {
    Halt,
    Running,
    Jump,
    Output(MemoryValue),
    WaitingForInput,
}

#[cfg(test)]
mod test {
    use crate::{IntCodeComputer, Memory, Mode, Output};

    #[test]
    fn test_parse_instruction_code() {
        assert_eq!(
            IntCodeComputer::decode_opcode(102).1,
            (Mode::Immediate, Mode::Position, Mode::Position)
        );
        assert_eq!(
            IntCodeComputer::decode_opcode(01).1,
            (Mode::Position, Mode::Position, Mode::Position)
        );
        assert_eq!(
            IntCodeComputer::decode_opcode(11101).1,
            (Mode::Immediate, Mode::Immediate, Mode::Immediate)
        );
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
                let mut comp = IntCodeComputer::new(p.clone());
                let output = comp.run(spec.0.clone());
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

        programs.iter().zip(&specs).for_each(|(p, spec)| {
            let mut comp = IntCodeComputer::new(p.clone());
            let output = comp.run(spec.0.clone());
            assert_eq!(output, spec.1);
        });
    }

    #[test]
    fn test_relative_mode() {
        let program = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let input: Memory = vec![];
        let output = program.clone();

        let mut comp = IntCodeComputer::new(program);
        comp.run(input);
        assert_eq!(output, comp.output);
    }

    #[test]
    fn test_large_numbers() {
        let program = vec![104, 1125899906842624, 99];
        let input: Memory = vec![];
        let output = vec![1125899906842624];

        let mut comp = IntCodeComputer::new(program);
        comp.run(input);
        assert_eq!(output, comp.output);
    }

    #[test]
    fn test_large_number_output() {
        let program = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        let input: Memory = vec![];

        let mut comp = IntCodeComputer::new(program);
        comp.run(input);
        let last_output = comp.output.first().unwrap().to_string();
        assert_eq!(last_output.len(), 16);
    }
}
