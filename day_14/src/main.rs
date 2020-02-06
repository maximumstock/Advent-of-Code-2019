use std::collections::HashMap;

fn main() {
    let hash_map = parse_input(input());
    let n_ore = part1(hash_map.clone());
    println!("Need {} ore to create one fuel", n_ore);
    let n_fuel = part2(hash_map.clone(), 1_000_000_000_000);
    println!("Can produce {} fuel", n_fuel);
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Item {
    quantity: usize,
    name: String,
}

impl Item {
    fn is_ore(&self) -> bool {
        self.name.eq(&String::from("ORE"))
    }
}

impl std::ops::Mul<usize> for Item {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self::Output {
        Item {
            quantity: self.quantity * rhs,
            name: self.name,
        }
    }
}

type IngredientMap = HashMap<Item, Vec<Item>>;

fn part1(input: IngredientMap) -> usize {
    let start_item = Item {
        quantity: 1,
        name: String::from("FUEL"),
    };
    resolve(&input, &start_item)
}

fn part2(input: IngredientMap, target: usize) -> usize {
    let x = move |quantity: usize| {
        let fuel_item = Item { name: String::from("FUEL"), quantity };
        let required_ore = resolve(&input, &fuel_item);
//        println!("Producing {} fuel with {} ore", quantity, required_ore);
        required_ore as usize
    };

    let cost_one = x(1);
    let mut fuel_left = target / cost_one;
    let mut fuel_right = fuel_left * 2;

    while fuel_right - fuel_left > 1 {
        let fuel = (fuel_left + fuel_right) / 2;
        let required_ore = x(fuel);

        if required_ore < target {
            fuel_left = fuel;
        } else {
            fuel_right = fuel;
        }
    }

    fuel_left
}

fn try_leftovers(leftovers: &mut HashMap<String, usize>, item: &Item) -> usize {
    let required = item.quantity;
    if let Some(leftover) = leftovers.get_mut(&item.name) {
        if *leftover >= required {
            *leftover -= required;
            return 0;
        } else {
            let remainder = required - *leftover;
            leftovers.remove(&item.name);
            return remainder;
        }
    } else {
        return required;
    }
}

fn resolve(ingredient_map: &IngredientMap, item: &Item) -> usize {
    let mut queue: Vec<Item> = vec![item.clone()];
    let mut leftovers: HashMap<String, usize> = HashMap::new();
    let mut ore: usize = 0;

//    println!("{:?}", queue);
    while let Some(next) = queue.pop() {
//        println!("Next item {:?}", next);
        if next.is_ore() {
            ore += next.quantity;
            continue;
        }

        let remaining = try_leftovers(&mut leftovers, &next);
//        println!("\t\tNeed to produce: {:?} of item: {:?}", remaining, next);
        if remaining > 0 {
            let minimum_production_size = ingredient_map
                .keys()
                .filter(|i| i.name.eq(&next.name))
                .cloned()
                .collect::<Vec<Item>>()
                .first()
                .unwrap()
                .quantity;

//            println!("\t\tMinimum production size {:?}", minimum_production_size);

            let factor = (remaining as f64 / minimum_production_size as f64).ceil() as usize;

            let n_produced = minimum_production_size * factor;
//            println!("\t\tProducing {} of {:?}", n_produced, next);

            // Save leftovers
            if n_produced > remaining {
                let leftover = n_produced - remaining;
//                println!("\t\tLeftovers: {:?}", leftover);
                *leftovers.entry(next.name.clone()).or_insert(0) += leftover;
            }

            // Schedule production for ingredients in quantities based on factor
            let ingredients = get_ingredients(&ingredient_map, &next);
            for i in &ingredients {
//                println!("\t\tItem: {:?}, factor: {:?}", i, factor);
                queue.push((i.clone() * factor));
            }
        }
    }

    ore
}

fn get_ingredients(ingredient_map: &IngredientMap, item: &Item) -> Vec<Item> {
    let ancestor_item = ingredient_map
        .keys()
        .filter(|i| i.name.eq(&item.name))
        .collect::<Vec<&Item>>()
        .first()
        .unwrap()
        .clone();

    ingredient_map.get(&ancestor_item).unwrap().clone()
}

fn parse_token(input: &str) -> Item {
    let tokens: Vec<String> = input
        .trim()
        .split(" ")
        .into_iter()
        .map(|x| String::from(x))
        .collect();
    let n: usize = tokens.get(0).unwrap().parse().unwrap();
    let id = tokens.get(1).unwrap();
    Item {
        quantity: n,
        name: id.clone(),
    }
}

fn parse_input(input: &'static str) -> HashMap<Item, Vec<Item>> {
    input
        .lines()
        .into_iter()
        .map(|line| {
            let tokens: Vec<String> = line
                .split("=>")
                .into_iter()
                .map(|x| String::from(x))
                .collect();
            let produced_item = parse_token(tokens.get(1).unwrap());
            let ingredients: Vec<Item> =
                tokens.get(0).unwrap().split(",").map(parse_token).collect();
            (produced_item, ingredients)
        })
        .collect::<HashMap<Item, Vec<Item>>>()
}

fn input() -> &'static str {
    "3 JQFM, 5 QMQB, 20 WQCT => 8 PHBMP
    2 XJFQR => 1 WQCT
    133 ORE => 3 KFKWH
    1 QGVJV, 9 TNRGW, 9 NSWDH => 5 HJPD
    4 QMQB, 2 QZMZ, 3 DQPX, 1 HJFV, 5 SLQN, 4 XHKG, 23 DBKL => 5 CVZLJ
    6 GFDP, 1 MXQF => 7 TDPN
    19 BWHKF, 2 KXHQW, 8 GHNG, 8 CSNS, 8 JVRQ, 1 PHBMP, 20 LZWR, 7 JKRZH => 5 PZRSQ
    1 JQFM => 1 QGVJV
    8 KFKWH => 7 ZJKB
    3 VMDSG, 2 BMSNV => 9 XJFQR
    7 ZKZHV => 6 DVRS
    1 WKFTZ, 5 SVTK, 1 QDJD => 7 JQFM
    19 FRTK => 2 QMTMN
    23 DVRS, 3 XNGTQ => 9 MCWF
    188 ORE => 3 HDRMK
    3 MCWF => 5 LHXN
    12 KFKWH, 2 DWBL => 8 ZKZHV
    2 GHNG => 8 SVTK
    4 MLJN, 9 CSNS => 6 DQPX
    2 NDNP, 1 LWGNJ, 6 DBKL, 3 RLKDF, 9 DQPX, 1 BWHKF => 3 JVGRC
    4 TNRGW => 2 CFBP
    2 KXHQW => 1 BWHKF
    7 HJFV => 4 PDKZ
    2 QZMZ => 5 BMSNV
    1 SVTK, 1 LZWR, 1 WQCT => 3 SLQN
    1 TDPN, 1 VMDSG => 7 NHVQD
    1 SVTK => 2 LZWR
    149 ORE => 9 DWBL
    1 XMHN, 1 PDKZ => 5 LWGNJ
    6 WCMV => 6 XNGTQ
    7 MCWF, 2 VCMPS => 1 HJFV
    11 BRTX, 37 CFBP, 2 HJPD, 72 HDRMK, 5 LWGNJ, 7 JVGRC, 3 CVZLJ, 8 PZRSQ, 3 LQBJP => 1 FUEL
    9 QMTMN, 14 FRTK, 14 HJFV => 9 NDNP
    1 KFKWH, 3 ZJKB => 9 MXQF
    1 HJFV, 1 XJFQR => 9 TNRGW
    1 DVRS => 2 BRTX
    4 QZMZ, 3 BMSNV, 3 GFDP => 6 VMDSG
    3 NHVQD => 6 WKFTZ
    1 BWHKF => 6 DBKL
    8 DWBL => 8 QZMZ
    4 MLJN, 16 NSWDH, 4 XHKG => 8 JVRQ
    2 DVRS, 32 XNGTQ, 9 MXQF => 7 GHNG
    1 DWBL => 8 WCMV
    8 SLQN, 1 CFBP => 9 MLJN
    1 QDJD => 4 XMHN
    3 BWHKF, 2 TNRGW => 9 XHKG
    1 WGLN => 8 GFDP
    1 MCWF, 1 XJFQR => 2 CSNS
    3 XNGTQ => 1 QDJD
    15 KXHQW, 3 WQCT, 2 QMTMN => 8 NSWDH
    9 XCMJ, 1 QMTMN => 2 JKRZH
    4 HDRMK => 4 WGLN
    9 NSWDH, 12 LHXN, 16 NDNP => 1 QMQB
    16 NHVQD, 3 DWBL, 1 WKFTZ => 4 FRTK
    1 GFDP => 2 VCMPS
    2 JQFM, 2 XMHN => 6 XCMJ
    1 DVRS, 19 QZMZ, 1 DWBL => 5 KXHQW
    1 QGVJV, 8 NDNP, 5 PDKZ => 1 RLKDF
    29 HJFV, 2 WKFTZ, 4 GFDP => 2 LQBJP"
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, resolve, Item};

    fn test(input: &'static str, fuel: usize) {
        let parsed = parse_input(input);
        let result = resolve(
            &parse_input(input),
            &Item {
                quantity: 1,
                name: String::from("FUEL"),
            }
        );
        assert_eq!(result, fuel);
    }

    #[test]
    fn test_simple() {
        let input = "\
            9 ORE => 2 A
            8 ORE => 3 B
            7 ORE => 5 C
            3 A, 4 B => 1 AB
            5 B, 7 C => 1 BC
            4 C, 1 A => 1 CA
            2 AB, 3 BC, 4 CA => 1 FUEL";
        test(input, 165);
    }

    #[test]
    fn test_medium() {
        let input = "\
            157 ORE => 5 NZVS
            165 ORE => 6 DCFZ
            44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
            12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
            179 ORE => 7 PSHF
            177 ORE => 5 HKGWZ
            7 DCFZ, 7 PSHF => 2 XJWVT
            165 ORE => 2 GPVTF
            3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";
        test(input, 13312);
    }

    #[test]
    fn test_large() {
        let input = "\
            145 ORE => 6 MNCFX
            176 ORE => 6 VJHF
            139 ORE => 4 NVRVD
            144 ORE => 7 JNWZP
            1 NVRVD => 8 CXFTF
            17 NVRVD, 3 JNWZP => 8 VPVL
            22 VJHF, 37 MNCFX => 5 FWMGM
            1 VJHF, 6 MNCFX => 4 RFSQX
            2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
            5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
            5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
            53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL";
        test(input, 180697);
    }
}
