    #[derive(Clone, Debug, PartialEq)]
    pub enum Parameter {
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
    pub type Output = Vec<String>;

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
    pub struct IntCodeComputer {
        memory: Memory,
        input: Memory,
        output: Memory,
        pc: usize,
    }

    impl IntCodeComputer {
        pub fn new(memory: Memory) -> Self {
            IntCodeComputer { memory, input: vec![],  output: vec![], pc: 0 }
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

        pub fn run(&mut self, input: Memory) -> Output {
            self.input = input;
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


    #[cfg(test)]
    mod test {
        use crate::{IntCodeComputer, Mode, Memory, Output};

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

            programs.iter()
                .zip(&specs)
                .for_each(|(p, spec)| {
                    let mut comp = IntCodeComputer::new(p.clone());
                    let output = comp.run(spec.0.clone());
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
