fn main() {
    let input = input();
    // Part 1
    let part1_input = input_for_parameters(&input, 12, 2);
    let part1_output = part1(&part1_input);
    let part1_result = part1_output.get(0).unwrap();
    println!("Part 1 Result: {:?}", part1_result);
    assert_eq!(*part1_result, 5110675);

    // Part 2
    let (noun, verb) = part2(&input, 19690720);
    println!("Part 2 Result: {:?}", 100 * noun + verb);
}

fn part1(input: &Vec<usize>) -> Vec<usize> {
    let mut input = input.clone();

    let mut pos = 0;
    loop {
        let next = input[pos];
        match next {
            1 => {
                let op1 = input[pos+1];
                let op2 = input[pos+2];
                let target_idx = input[pos+3];
                input[target_idx] = input[op1] + input[op2];
                pos += 4;
            },
            2 => {
                let op1 = input[pos+1];
                let op2 = input[pos+2];
                let target_idx = input[pos+3];
                input[target_idx] = input[op1] * input[op2];
                pos += 4;
            },
            99 => break,
            _ => unreachable!()
        }
    }

    input
}

fn input_for_parameters(input: &Vec<usize>, p1: usize, p2: usize) -> Vec<usize> {
    let mut input = input.clone();
    input[1] = p1;
    input[2] = p2;
    input
}

fn input() -> Vec<usize> {
    vec![
        1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 9, 1, 19, 1, 5, 19, 23, 2, 9, 23, 27, 1,
        27, 5, 31, 2, 31, 13, 35, 1, 35, 9, 39, 1, 39, 10, 43, 2, 43, 9, 47, 1, 47, 5, 51, 2, 13,
        51, 55, 1, 9, 55, 59, 1, 5, 59, 63, 2, 6, 63, 67, 1, 5, 67, 71, 1, 6, 71, 75, 2, 9, 75, 79,
        1, 79, 13, 83, 1, 83, 13, 87, 1, 87, 5, 91, 1, 6, 91, 95, 2, 95, 13, 99, 2, 13, 99, 103, 1,
        5, 103, 107, 1, 107, 10, 111, 1, 111, 13, 115, 1, 10, 115, 119, 1, 9, 119, 123, 2, 6, 123,
        127, 1, 5, 127, 131, 2, 6, 131, 135, 1, 135, 2, 139, 1, 139, 9, 0, 99, 2, 14, 0, 0,
    ]
}

fn part2(base_input: &Vec<usize>, solution: i32) -> (usize, usize) {
    let mut params = (0usize, 0usize);
    for x in 0..100usize {
        for y in 0..100usize {
            let input = input_for_parameters(base_input, x, y);
            let output = part1(&input);
            let result = output.get(0).unwrap();
            if *result == solution as usize {
                params = (x, y);
                break
            }
        }
    }
    params
}

#[test]
fn test_part1() {
    assert_eq!(part1(&vec![1, 0, 0, 0, 99]), vec![2, 0, 0, 0, 99]);
    assert_eq!(part1(&vec![2, 3, 0, 3, 99]), vec![2, 3, 0, 6, 99]);
    assert_eq!(part1(&vec![2, 4, 4, 5, 99, 0]), vec![2, 4, 4, 5, 99, 9801]);
    assert_eq!(part1(&vec![1, 1, 1, 4, 99, 5, 6, 0, 99]), vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
}
