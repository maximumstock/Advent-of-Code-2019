fn main() {
    //    let result = part1(input(), 100);
    //    println!("{:?}", &result[0..8]);

    let result = part2(input());
    println!("Message is {:?}", result);
}

fn part1(mut input: Vec<u32>, n_phases: usize) -> Vec<u32> {
    for _ in 0..n_phases {
        input = calculate_fft(input);
    }
    input
}

// Credits to https://github.com/prscoelho/aoc2019/blob/master/src/aoc16/mod.rs
fn part2(mut input: Vec<u32>) -> usize {
    let offset_message: &[u32] = &input[0..7];
    let offset = vec_to_num(offset_message);

    let start = offset;
    let end = input.len() * 10_000;

    let mut current = Vec::new();
    for i in start..end {
        current.push(input[i % input.len()]);
    }

    for _ in 0..100 {
        let mut sums = Vec::new();
        let mut total = 0;
        sums.push(0);
        for i in 0..current.len() {
            total += current[i];
            sums.push(total);
        }
        for i in 0..current.len() {
            let value = sums.last().unwrap() - sums[i];
            current[i] = value % 10;
        }
    }

    vec_to_num(&current[0..8])
}

fn vec_to_num(vec: &[u32]) -> usize {
    vec.iter().fold(0u32, |acc, x| acc * 10 + *x) as usize
}

fn calculate_fft(input: Vec<u32>) -> Vec<u32> {
    let mut output: Vec<u32> = Vec::with_capacity(input.len());

    for idx in 0..input.len() {
        let v = calculate_fft_idx(input.as_slice(), idx);
        output.push(v);
    }

    output
}

fn calculate_fft_idx(input: &[u32], index: usize) -> u32 {
    let base_pattern: Vec<i32> = vec![0, 1, 0, -1];
    let mut pattern: Vec<i32> = vec![];

    for base_element in &base_pattern {
        for _ in 0..(index + 1) {
            pattern.push(*base_element);
        }
    }

    let pattern_length = input.len() + 1;

    let x: i32 = input
        .iter()
        .zip(pattern.iter().cycle().take(pattern_length).skip(1))
        .map(|(left, right)| *left as i32 * *right)
        .map(|n| n as i32)
        .sum();

    (x % 10).abs() as u32
}

fn input() -> Vec<u32> {
    let raw = "59727310424796235189476878806940387435291429226818921130171187957262146115559932358924341808253400617220924411865224341744614706346865536561788244183609411225788501102400269978290670307147139438239865673058478091682748114942700860895620690690625512670966265975462089087644554004423208369517716075591723905075838513598360188150158989179151879406086757964381549720210763972463291801513250953430219653258827586382953297392567981587028568433943223260723561880121205475323894070000380258122357270847092900809245133752093782889315244091880516672127950518799757198383131025701009960944008679555864631340867924665650332161673274408001712152664733237178121872";
    raw.chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>()
}

fn real_input() -> Vec<u32> {
    let input = input();
    input
        .iter()
        .cycle()
        .take(input.len() * 10_000)
        .cloned()
        .collect::<Vec<u32>>()
}

#[cfg(test)]
mod tests {
    use crate::{part1, vec_to_num};

    #[test]
    fn test_phase_1() {
        let input = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let expected_phase1 = vec![4, 8, 2, 2, 6, 1, 5, 8];
        assert_eq!(part1(input, 1), expected_phase1);

        let expected_phase2 = vec![3, 4, 0, 4, 0, 4, 3, 8];
        assert_eq!(part1(expected_phase1, 1), expected_phase2);
    }

    #[test]
    fn test_vec_to_num() {
        let input = vec![4, 3, 2, 1];
        let expected: u32 = 4321;
        assert_eq!(vec_to_num(&input), expected)
    }
}
