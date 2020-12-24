use std::collections::{HashMap, HashSet, VecDeque};
mod helpers;

fn main() {
    let filename: &str = "day21.txt";
    let input = &helpers::input_helpers::read_input(&filename).unwrap();
    task_1(input);
    task_2(input);
}

fn task_1(input: &[String]) -> u64 {
    let foods: Vec<Food> = input.iter().map(Food::parse).collect();
    let allergen_map = build_allergen_map(&foods);

    // Count ingredients not containing allergens
    let result = foods
        .iter()
        .map(|f| &f.ingredients)
        .flatten()
        .filter(|i| !allergen_map.contains_key(&i.to_string()))
        .count();

    println!("Task 1: {}", result);
    result as u64
}

fn task_2(input: &[String]) -> String {
    let foods: Vec<Food> = input.iter().map(Food::parse).collect();
    let allergen_map = build_allergen_map(&foods);

    let mut allergen_list: Vec<(String, String)> = allergen_map.into_iter().collect();
    allergen_list.sort_by(|a, b| a.1.cmp(&b.1));
    let result = allergen_list
        .into_iter()
        .map(|r| r.0)
        .collect::<Vec<String>>()
        .join(",");
    println!("Task 2: {}", result);
    result
}

fn build_allergen_map(input: &Vec<Food>) -> HashMap<String, String> {
    let allergens: HashSet<String> = input
        .iter()
        .map(|f| &f.allergens)
        .flatten()
        .cloned()
        .collect();
    let mut allergen_alternatives: HashMap<String, Vec<String>> = HashMap::new();
    for allergen in &allergens {
        let affected_ingredient_sets: Vec<&HashSet<String>> = input
            .iter()
            .filter(|f| f.allergens.contains(&allergen.to_string()))
            .map(|f| &f.ingredients)
            .collect();
        let mut potential_ingredients = affected_ingredient_sets[0].clone();
        for ingredient_set in affected_ingredient_sets {
            potential_ingredients = potential_ingredients
                .intersection(ingredient_set)
                .cloned()
                .collect();
        }
        // The set of potential ingredients may have more than one alternative, but since
        // each ingredient should have 0..1 allergens, they will be eliminated later
        allergen_alternatives.insert(
            allergen.to_string(),
            potential_ingredients.into_iter().collect(),
        );
    }

    let mut allergens_to_process: VecDeque<String> = allergens.into_iter().collect();
    // Mapping from ingredients to their guaranteed allergens
    let mut matches: HashMap<String, String> = HashMap::new();

    while let Some(allergen) = allergens_to_process.pop_front() {
        let potential_ingredients: Vec<&String> = allergen_alternatives
            .get(&allergen)
            .unwrap()
            .iter()
            .filter(|i| !matches.contains_key(&i.to_string()))
            .collect();
        if potential_ingredients.len() == 1 {
            matches.insert(potential_ingredients[0].to_string(), allergen);
        } else {
            allergens_to_process.push_back(allergen);
        }
    }

    matches
}

#[derive(Debug)]
struct Food {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}

impl Food {
    fn parse(input: &String) -> Food {
        let split: Vec<&str> = input
            .strip_suffix(')')
            .unwrap()
            .split(" (contains ")
            .collect();

        Food {
            ingredients: split[0]
                .split(' ')
                .map(str::to_string)
                .collect::<HashSet<String>>(),
            allergens: split[1]
                .split(", ")
                .map(str::to_string)
                .collect::<HashSet<String>>(),
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn verify_day21_example_task_1() {
        let input = vec![
            "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)".to_string(),
            "trh fvjkl sbzzf mxmxvkd (contains dairy)".to_string(),
            "sqjhc fvjkl (contains soy)".to_string(),
            "sqjhc mxmxvkd sbzzf (contains fish)".to_string(),
        ];
        assert_eq!(5, crate::task_1(&input));
    }

    #[test]
    fn verify_day21_example_task_2() {
        let input = vec![
            "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)".to_string(),
            "trh fvjkl sbzzf mxmxvkd (contains dairy)".to_string(),
            "sqjhc fvjkl (contains soy)".to_string(),
            "sqjhc mxmxvkd sbzzf (contains fish)".to_string(),
        ];
        assert_eq!("mxmxvkd,sqjhc,fvjkl".to_string(), crate::task_2(&input));
    }
}
