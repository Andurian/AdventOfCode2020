use std::collections::{HashMap, HashSet};

fn solve(filename: &str) -> (i32, String) {
    let mut all_ingredients = HashSet::<String>::new();
    let mut all_allergenes = HashMap::<String, HashSet<String>>::new();
    let mut occurrences = HashMap::<String, i32>::new();

    for line in common::read_file_linewise(filename) {
        let tokens = line.split(" (").collect::<Vec<&str>>();
        let ingredients = tokens[0]
            .split(" ")
            .map(|s| String::from(s.trim()))
            .collect::<HashSet<String>>();
        for ingredient in &ingredients {
            match occurrences.get_mut(ingredient) {
                Some(v) => {
                    *v += 1;
                }
                None => {
                    occurrences.insert(ingredient.clone(), 1);
                }
            }
        }
        all_ingredients = all_ingredients
            .union(&ingredients.clone())
            .cloned()
            .collect::<HashSet<String>>();
        let allergenes = tokens[1][9..tokens[1].len() - 1]
            .split(", ")
            .map(|s| String::from(s.trim()))
            .collect::<Vec<String>>();
        for allergen in allergenes {
            match all_allergenes.get_mut(&allergen) {
                Some(existing_ingredients) => {
                    *existing_ingredients = existing_ingredients
                        .intersection(&ingredients.clone())
                        .cloned()
                        .collect::<HashSet<String>>();
                }
                None => {
                    all_allergenes.insert(allergen, ingredients.clone());
                }
            }
        }
    }

    loop {
        let mut cleaned_allergenes = all_allergenes.clone();

        for (k, v) in &all_allergenes {
            if v.len() == 1 {
                for (ko, vo) in cleaned_allergenes.iter_mut() {
                    if k != ko {
                        *vo = vo.difference(&v).cloned().collect();
                    }
                }
            }
        }

        if cleaned_allergenes == all_allergenes {
            break;
        }

        all_allergenes = cleaned_allergenes;
    }

    let known_ingredients = all_allergenes
        .iter()
        .fold(HashSet::<String>::new(), |acc, s| {
            acc.union(s.1).cloned().collect()
        });
    let safe_ingredients = all_ingredients
        .difference(&known_ingredients)
        .cloned()
        .collect::<HashSet<String>>();

    let mut canonical_list = all_allergenes.keys().cloned().collect::<Vec<String>>();
    canonical_list.sort();

    let mut canoncial_list_translated = String::new();
    let mut first = true;

    for item in canonical_list {
        if !first {
            canoncial_list_translated.push(',');
        } else {
            first = false;
        }
        canoncial_list_translated.push_str(
            &all_allergenes
                .get(&item)
                .unwrap()
                .iter()
                .next()
                .unwrap()
                .clone(),
        );
    }

    (
        safe_ingredients
            .iter()
            .fold(0, |acc, s| acc + occurrences.get(s).unwrap()),
        canoncial_list_translated,
    )
}

fn main() {
    let solution = solve("src/day21/input.txt");
    println!("Solution 1: {}", solution.0);
    println!("Solution 2: {}", solution.1);
}
