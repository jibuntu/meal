use std::fs::File;
use std::io::BufReader;
use std::iter::Iterator;

use clap::ArgMatches;
use serde_json::Value;

use crate::food_table::FoodTable;
use crate::food::{FoodData, Food};
use crate::parse_json::{Body, parse_body, parse_json};
use crate::kijun::Kijun;

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

    println!("[automatic selection]");

    Ok(())
}

struct Combination<T> {
    data_list: Vec<T>,
    length_of_combination: usize
}

impl<T> Combination<T> {
    fn new(data_list: Vec<T>, length_of_combination: usize) -> Combination<T> {
        Combination {
            data_list,
            length_of_combination
        }
    }

    fn iter(&self) -> CombIterator<T> {
        CombIterator::new(self)
    }
}

struct CombIterator<'a, T> {
    comb: &'a Combination<T>,
    keys_list: Vec<Vec<usize>>
}

impl<'a, T> CombIterator<'a, T> {
    fn new(comb: &'a Combination<T>) -> CombIterator<'a, T> {
        let mut keys: Vec<usize> = (0..comb.data_list.len()).collect();
        let mut keys_list = Vec::new();

        for _i in 0..comb.length_of_combination {
            keys_list.push(keys.clone());
            keys.pop();
        }

        CombIterator {
            comb,
            keys_list
        }
    }

    fn change_keys_list(&mut self) {
        self.keys_list.last_mut().unwrap().pop();

        for i in 1..self.keys_list.len() {
            let digit = self.keys_list.len() - i;
            if self.keys_list[digit].len() < self.keys_list.len() - digit {
                // 親の値を１つ消す
                self.keys_list[digit - 1].pop();
                let mut parent = self.keys_list[digit - 1].clone();
                parent.pop();

                self.keys_list[digit] = parent;

                // 子にも伝える
                for digit in digit+1..self.keys_list.len() {
                    if self.keys_list[digit].len() < self.keys_list.len() - digit {
                        let mut parent = self.keys_list[digit - 1].clone();
                        parent.pop();
                        self.keys_list[digit] = parent;
                    }
                }
            }
        }
    }
}

impl<'a, T: 'a> Iterator for CombIterator<'a, T> {
    type Item = Vec<&'a T>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut key_list = Vec::new();
        for keys in &self.keys_list {
            if let Some(key) = keys.last() {
                key_list.push(*key);
            } else {
                return None;
            }

        }

        // keys_listを元に値のリストを作成
        let mut value_list = Vec::new();
        for key in key_list {
            value_list.push(&self.comb.data_list[key]);
        }

        self.change_keys_list();

        Some(value_list)
    }

}

#[test]
fn test_comb_iterator() {
    let data_list = vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
        11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        21, 22, 23, 24, 25, 26, 27, 28, 29, 30
    ];
    let comb = Combination::new(data_list, 3);
    //let mut iter = comb.iter();

    for c in comb.iter() {
    }

    // 0.6s
    //println!("{:?}", iter.next());
    //println!("{:?}", iter.next());
    //println!("{:?}", iter.next());
    //println!("{:?}", iter.next());
    //println!("{:?}", iter.next());
    //println!("{:?}", iter.next());
    //println!("{:?}", iter.next());
    //println!("{:?}", iter.next());
    //println!("{:?}", iter.next());
    //println!("{:?}", iter.next());
    //println!("{:?}", iter.next());

}
