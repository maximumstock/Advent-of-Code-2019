fn main() {
    let input = vec![
        149579, 95962, 97899, 149552, 65085, 111896, 127591, 115128, 64630, 120430, 81173, 136775,
        137806, 132042, 65902, 87894, 97174, 126829, 88716, 85284, 61178, 106423, 89821, 51123,
        85350, 53905, 74259, 59710, 80358, 111938, 129027, 144036, 68717, 69382, 64163, 65114,
        58548, 74559, 142855, 115617, 107847, 133264, 111657, 125402, 129254, 67275, 120955,
        110940, 139146, 96810, 147085, 103471, 89560, 111940, 120332, 55717, 73498, 133817, 102095,
        57518, 57725, 58673, 84918, 143693, 149361, 74432, 51048, 99136, 128220, 141591, 79477,
        116798, 93622, 113316, 143888, 143155, 57861, 112833, 70928, 116310, 126836, 93835, 101281,
        116599, 107776, 138215, 107034, 74826, 73372, 127785, 105051, 124720, 147682, 97320, 74957,
        113446, 101566, 96278, 144766, 55755,
    ];

    part1(&input);
    part2(&input);
}

fn part1(input: &Vec<i32>) {
    let total_fuel = input.iter()
        .map(mass_to_fuel)
        .fold(0, |left, right| left + right);
    println!("Case 1 - Total fuel: {:?}", total_fuel);
}

fn mass_to_fuel(mass: &i32) -> i32 {
    (mass / 3) - 2
}

fn part2(input: &Vec<i32>) {
    let total_fuel = input.iter()
        .map(mass_and_fuel_to_fuel)
        .fold(0, |left, right| left + right);

    println!("Case 2 - Total fuel: {:?}", total_fuel);
}

fn mass_and_fuel_to_fuel(module_mass: &i32) -> i32 {
    // Calculate fuel for module mass
    let module_fule = mass_to_fuel(module_mass);

    let mut total_fuel = module_fule;
    let mut current_mass = module_fule;

    loop {
        let fuel = mass_to_fuel(&current_mass);
        if fuel <= 0 { break }
        total_fuel += fuel;
        current_mass = fuel;
        println!("{:?}", fuel);
    }

    total_fuel
}

#[test]
fn test_add() {
    assert_eq!(mass_and_fuel_to_fuel(&14), 2);
    assert_eq!(mass_and_fuel_to_fuel(&1969), 966);
    assert_eq!(mass_and_fuel_to_fuel(&100756), 50346);
}
