use intcode::{Memory, IntCodeComputer, State};
use itertools::Itertools;

fn main() {
    let (highscore, best_permutation) = part1(input());
    println!("Highscore: {:?} with permutation: {:?}", highscore, best_permutation);

    let output = part2();
    println!("{:?}", output);
}

fn part2() -> isize {
    let program = input();
    let alphabet = vec![5, 6, 7, 8, 9];

    let mut max_output = 0 as isize;
    for permutation in alphabet.iter().permutations(alphabet.len()) {
        let perm = permutation.iter().map(|x| **x).collect::<Vec<isize>>();
        let output = run_amplifier(program.clone(), perm.clone());
        max_output = max_output.max(output);
    }
    max_output
}

fn run_amplifier(program: Memory, first_inputs: Memory) -> isize {
    let mut amplifiers = (0..first_inputs.len())
        .into_iter()
        .map(|idx| {
            let mut amp = IntCodeComputer::new(program.clone());
            amp.read_input(first_inputs.get(idx).unwrap().clone());
            amp
        })
        .collect::<Vec<IntCodeComputer>>();

    let mut last_output = 0 as isize;

    for idx in (0..amplifiers.len()).into_iter().cycle() {
        let amplifier = amplifiers.get_mut(idx).unwrap();
        println!("Running amplifier: {:?}", idx);
        println!("State: {:?}", amplifier);

        let mut state = State::Running;

        loop {
            state = amplifier.step();
            println!("State: {:?}", amplifier);
            match state {
                State::Output(output) => {
                    last_output = output;
                    break
                },
                State::WaitingForInput => amplifier.read_input(last_output.to_owned()),
                State::Halt => break,
                _ => continue
            }
        }

        if state == State::Halt && idx == 4 {
            break
        }
    }

    last_output
}


fn part1(program: Memory) -> (isize, Vec<isize>) {
    let mut input: Memory = vec![];
    let phases: Vec<isize> = vec![0, 1, 2, 3, 4];

    let mut highscore = 0;
    let mut best_permutation: Vec<isize> = vec![];
    let mut last_output = 0;

    for permutation in phases.iter().permutations(5) {
        println!("\tCurrent Permutation: {:?}", permutation);
        for (phase_idx, next_phase) in permutation.iter().enumerate() {
            // Setup inputs for this iteration
            input.clear();
            input.push(last_output);
            input.push(**next_phase);

//            println!("\t\tPhase Idx: {:?}, next_phase: {:?}, Input: {:?}", phase_idx, next_phase, input);
//            println!("\t\tHighscore: {:?}", highscore);

            // Determine the current's phase output
            let mut computer = IntCodeComputer::new(program.clone());
            let output = computer.run(input.clone());
            last_output = output.last().unwrap().parse().unwrap();

            // During the last phase, we can check if our fifth amplifier returns an output
            // greater than the last one
            if phase_idx == 4 {
                if last_output > highscore {
                    highscore = last_output;
                    best_permutation.clear();
                    for x in &permutation {
                        best_permutation.push(*x.clone());
                    }
                }
                // Either way, we must reset the last output value for the next permutation
                last_output = 0;
            }
        }
    }
    (highscore, best_permutation.clone())
}

fn input() -> Memory {
    vec![
        3, 8, 1001, 8, 10, 8, 105, 1, 0, 0, 21, 42, 67, 84, 109, 126, 207, 288, 369, 450, 99999, 3, 9, 102, 4, 9, 9, 1001, 9, 4, 9, 102, 2, 9, 9, 101, 2, 9, 9, 4, 9, 99, 3, 9, 1001, 9, 5, 9, 1002, 9, 5, 9, 1001, 9, 5, 9, 1002, 9, 5, 9, 101, 5, 9, 9, 4, 9, 99, 3, 9, 101, 5, 9, 9, 1002, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 99, 3, 9, 1001, 9, 2, 9, 102, 4, 9, 9, 101, 2, 9, 9, 102, 4, 9, 9, 1001, 9, 2, 9, 4, 9, 99, 3, 9, 102, 2, 9, 9, 101, 5, 9, 9, 1002, 9, 2, 9, 4, 9, 99, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 99, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 99, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 99, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 99, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 99
    ]
}

#[cfg(test)]
mod test {
    use intcode::Memory;
    use crate::run_amplifier;

    #[test]
    fn test_part2() {
        let specs: Vec<(Memory, Vec<isize>, isize)> = vec![
            (
                vec![
                    3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26,
                    27, 4, 27, 1001, 28, -1, 28, 1005, 28, 6, 99, 0, 0, 5
                ], vec![9, 8, 7, 6, 5], 139629729
            ),
            (
                vec![
                    3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
                    -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
                    53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10
                ], vec![9, 7, 8, 5, 6], 18216
            )
        ];

        for (input, settings, thruster_signal) in specs {
            let output = run_amplifier(input, settings);
            assert_eq!(output, thruster_signal);
        }
    }
}
