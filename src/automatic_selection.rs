use std::fs::File;
use std::io::BufReader;
use std::iter::Iterator;

use clap::ArgMatches;
use serde_json::Value;

use crate::food_table::FoodTable;
use crate::food::{FoodData, Food};
use crate::parse_json::{Body, parse_body, parse_json};
use crate::kijun::Kijun;
use crate::combination::Combination;

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

    let path = "/home/jibuntu/programming_language/rust/project/meal/data/foods.json";
    let mut foods = match FoodTable::from_json(path) {
        Ok(foods) => foods,
        Err(e) => return Err(e.to_string())
    };

    let mut food_table = FoodTable::new();
    for parsed_food in parsed_data.foods {
        let mut food = match foods.get(&parsed_food.number) {
            Some(food) => food.clone(),
            None => return Err(format!("{}番の食材はありません。JSONの値が間違っています", &parsed_food.number))
        };

        food.set_weight(parsed_food.weight.unwrap_or(100.0));
        food.set("重量", FoodData::String(parsed_food.weight.unwrap_or(100.0).to_string()));
        food_table.add(food);
    }

    let list: Vec<_> = parsed_data.name_list.iter().map(|name| name.as_str()).collect();

    let kijun = Kijun::new(parsed_data.body.age,
                           parsed_data.body.weight,
                           parsed_data.body.height,
                           parsed_data.body.gender,
                           parsed_data.body.pal);

    // まずは、総当たりの組み合わせを表示する
    let mut keys: Vec<_> = food_table.iter().map(|(key, food)| key.to_string()).collect();

    //all_food_comb(keys, next_result, 0);
    //all_food_comb(keys, 2, 0);
    //all_food_comb_test(vec![1, 2, 3, 4, 5], vec![], 2, 0);


    let mut comb = Combination::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20], 5);

    let mut iter = comb.iter();

    println!("[automatic selection]");

    Ok(())
}


