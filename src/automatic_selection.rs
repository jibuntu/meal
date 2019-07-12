use std::fs::File;
use std::io::BufReader;
use std::iter::Iterator;

use clap::ArgMatches;
use serde_json::Value;

use crate::food_table::FoodTable;
use crate::food::Food;
use crate::food::food_data::FoodData;
use crate::parse_json::{Body, parse_body, parse_json};
use crate::kijun::Kijun;
use crate::combination::Combination;

fn color(text: &str, style: &str) -> String {
    let mut colored_text = String::new();
    let unset_style = "\x1b[0m";

    for c in style.chars() {
        let color_style = match c {
            'd' => "\x1b[30m",
            'r' => "\x1b[31m",
            'g' => "\x1b[32m",
            'y' => "\x1b[33m",
            'b' => "\x1b[34m",
            'm' => "\x1b[35m",
            'c' => "\x1b[36m",
            'w' => "\x1b[37m",
            'D' => "\x1b[90m",
            'R' => "\x1b[91m",
            'G' => "\x1b[92m",
            'Y' => "\x1b[93m",
            'B' => "\x1b[94m",
            'M' => "\x1b[95m",
            'C' => "\x1b[96m",
            'W' => "\x1b[97m",
            '+' => "\x1b[1m",
            _ => continue
        };

        colored_text += color_style;
    }

    colored_text + text + unset_style
}

pub fn automatic_selection(matches: &ArgMatches) -> Result<(), String> {
    let path = match matches.value_of("file") {
        Some(path) => path,
        None => return Err("ファイルを指定してください".to_string())
    };

    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(e) => return Err(e.to_string())
    };

    let parsed_data = match parse_json(BufReader::new(file)) {
        Ok(parsed_data) => parsed_data,
        Err(e) => return Err(e)
    };

    let food_table_path = "/home/jibuntu/programming_language/rust/project/meal/data/foods.json";
    let food_table = match FoodTable::from_json(food_table_path) {
        Ok(food_table) => food_table,
        Err(e) => return Err(e.to_string())
    };

    let mut inputted_food_table = FoodTable::new();
    for parsed_food in parsed_data.foods {
        let mut food = match food_table.get(&parsed_food.number) {
            Some(food) => food.clone(),
            None => return Err(format!("{}番の食材はありません。JSONの値が間違っています", &parsed_food.number))
        };

        food = food.change_weight(parsed_food.weight.unwrap_or(100.0)).unwrap();
        inputted_food_table.add(food);
    }

    let list: Vec<_> = parsed_data.name_list.iter().map(|name| name.as_str()).collect();

    let kijun = Kijun::new(parsed_data.body.age,
                           parsed_data.body.weight,
                           parsed_data.body.height,
                           parsed_data.body.gender,
                           parsed_data.body.pal);

    let mut food_table_list: Vec<(f32, FoodTable)> = Vec::new();
    let list_of_length_of_combination = parsed_data.comb.unwrap_or(vec![5]);

    for length_of_combination in list_of_length_of_combination {
        let keys: Vec<_> = inputted_food_table.iter().map(|(key, food)| key.to_string()).collect();
        let comb = Combination::new(keys, length_of_combination);

        for key_list in comb.iter() {
            let keys: Vec<&str> = key_list.iter().map(|key| key.as_str()).collect();
            let ft = inputted_food_table.get_list(&keys);
            let percentage = ft.percentage_of_kijun(&kijun).unwrap();
            food_table_list.push((percentage, ft));
        }
    }

    food_table_list.sort_by(|(p, f), (p2, f2)| p2.partial_cmp(&p).unwrap());

    for (index, (percentage, ft)) in food_table_list.iter().take(5).enumerate() {
        println!("{}", color(&format!("[{}] 摂取基準の達成率: {}", index+1, percentage), "g+"));
        ft.print_with_sum_and_kijun(&list, &kijun);
        println!();
    }

    println!("[automatic selection]");

    Ok(())
}


