fn main() {
    part1();
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
            Parameter::Value{ value: v} => v.clone(),
            Parameter::Reference { address: a } => memory.get(*a).unwrap().clone()
        }
    }
}


#[derive(Clone, Debug, PartialEq)]
enum Operation {
    Add(Parameter, Parameter, Parameter),
    Mul(Parameter, Parameter, Parameter),
    ReadInt(Parameter),
    Print(Parameter),
    NoOp,
    Halt,
}

impl Operation {
    fn size(&self) -> usize {
        match self {
            Self::Add(..) => 4,
            Self::Mul(..) => 4,
            Self::ReadInt(..) => 2,
            Self::Print(..) => 2,
            Self::NoOp => 1,
            Self::Halt => 1
        }
    }
}


type RawProgram = Vec<isize>;
type Program = Vec<Operation>;
type Memory = Vec<isize>;

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

struct IntCodeComputer {
    memory: Memory,
    input: Memory,
    pc: usize,
}

impl IntCodeComputer {
    fn new(memory: Memory) -> Self {
        IntCodeComputer { memory: memory.clone(), input: memory.clone(), pc: 0 }
    }

    fn parse_program(&mut self) -> Program {
        let mut parsed_program: Program = vec![];
        while self.pc < self.memory.len() {
            let operation = Self::parse_operation(&self.memory, self.pc);
            let op_length = operation.size();
            self.pc = self.pc + op_length;
            parsed_program.push(operation);
        }
        parsed_program
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
            (mode_c, mode_b, mode_a),
        )
    }

    fn parse_operation(program: &RawProgram, position: usize) -> Operation {
        let raw_op_code = program.get(position).expect("Cannot find position in memory");

        let op_code = *raw_op_code % 100;

        match op_code {
            99 => Operation::Halt,
            4 => Operation::Print(Parameter::Reference { address: Self::get_reference(&program, position + 1) }),
            3 => Operation::ReadInt(Parameter::Reference { address: Self::get_reference(&program, position + 1) }),
            1 => {
                let (_, mode_set) = Self::decode_opcode(*raw_op_code);
                Operation::Add(
                    Self::get_parameter_for_mode(&program, position + 1, mode_set.2),
                    Self::get_parameter_for_mode(&program, position + 2, mode_set.1),
                    Self::get_parameter_for_mode(&program, position + 3, mode_set.0),
                )
            }
            2 => {
                let (_, mode_set) = Self::decode_opcode(*raw_op_code);
                Operation::Mul(
                    Self::get_parameter_for_mode(&program, position + 1, mode_set.2),
                    Self::get_parameter_for_mode(&program, position + 2, mode_set.1),
                    Self::get_parameter_for_mode(&program, position + 3, mode_set.0),
                )
            }
            _ => Operation::NoOp
        }
    }

    fn get_parameter_for_mode(program: &RawProgram, index: usize, mode: Mode) -> Parameter {
        match mode {
            Mode::Position => Parameter::Reference { address: Self::get_reference(&program, index) },
            Mode::Immediate => Parameter::Value { value: Self::get_value(&program, index) }
        }
    }

    fn get_parameter(program: &RawProgram, index: usize) -> isize {
        program.get(index).unwrap().clone()
    }

    fn get_reference(program: &RawProgram, index: usize) -> usize {
        Self::get_parameter(&program, index) as usize
    }

    fn get_value(program: &RawProgram, index: usize) -> isize {
        Self::get_parameter(&program, index)
    }

    fn run_program(&mut self) {
        let operations = self.parse_program();
        self.pc = 0;
        for operation in operations {
            let new_state = self.execute_command(&operation);
            if new_state == State::Halt {
                break
            }
            self.pc += operation.size();
        }
    }

    fn execute_command(&mut self, operation: &Operation) -> State {
//        println!("{:?}", operation);

        match operation {
            Operation::ReadInt(Parameter::Reference {address: v}) => {
                let cell = self.memory.get_mut(*v).unwrap();
                *cell = 1;
                println!("READ\t{:?}", v);
                State::Running
            },
            Operation::Add(a, b, Parameter::Reference { address: target}) => {
                let x = a.eval(&self.memory);
                let y = b.eval(&self.memory);
                self.memory[*target] = x + y;
                println!("ADD\t{:?} + {:?} = {:?}", x, y, self.memory[*target]);
                State::Running
            },
            Operation::Mul(a, b, Parameter::Reference { address: target}) => {
                let x = a.eval(&self.memory);
                let y = b.eval(&self.memory);
                self.memory[*target] = x * y;
                println!("MUL\t{:?} * {:?} = {:?}", x, y, self.memory[*target]);
                State::Running
            },
            Operation::Print(address) => {
                println!("PRINT\t{:?}", address.eval(&self.memory));
                State::Running
            },
            Operation::Halt => {
                println!("HALT");
                println!("{:?}", &self.memory);
                State::Halt
            },
            Operation::NoOp => State::Running,
            _ => panic!("Unknown Operation")
        }
//        println!("{:?}", self.memory);
    }
}

#[derive(Debug, PartialEq)]
enum State {
    Halt,
    Running
}

fn part1() {
    let input = input();
    let mut comp = IntCodeComputer::new(input);
    comp.run_program();
}


#[test]
fn test_parse_instruction_code() {
    assert_eq!(IntCodeComputer::decode_opcode(102).1, (Mode::Position, Mode::Position, Mode::Immediate));
    assert_eq!(IntCodeComputer::decode_opcode(01).1, (Mode::Position, Mode::Position, Mode::Position));
    assert_eq!(IntCodeComputer::decode_opcode(11101).1, (Mode::Immediate, Mode::Immediate, Mode::Immediate));
}

#[test]
fn test_parse_program() {
    let raw_programs: Vec<RawProgram> = vec![
        vec![3, 100, 1, 1, 0, 0],
        vec![3, 100, 11101, 1, 0, 0],
    ];

    let parsed_programs: Vec<Program> = raw_programs
        .iter()
        .map(|x| IntCodeComputer::parse_program(&x))
        .collect();

    let expected_programs = vec![
        vec![
            Operation::ReadInt(Parameter::Reference { address: 100 }),
            Operation::Add(
                Parameter::Reference { address: 1 },
                Parameter::Reference { address: 0 },
                Parameter::Reference { address: 0 },
            )],
        vec![
            Operation::ReadInt(Parameter::Reference { address: 100 }),
            Operation::Add(
                Parameter::Value { value: 1 },
                Parameter::Value { value: 0 },
                Parameter::Value { value: 0 },
            )]
    ];

    parsed_programs
        .iter()
        .zip(&expected_programs)
        .for_each(|(parsed, expected)| assert_eq!(parsed, expected));
}
