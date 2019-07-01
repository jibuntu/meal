use std::collections::HashMap;
use std::str::FromStr;


#[derive(Debug, PartialEq, Clone)]
pub enum FoodData {
    Number(f32),
    String(String),
}

impl FoodData {
    pub fn from_str(data: &str) -> FoodData {
        match f32::from_str(data) {
            Ok(num) => FoodData::Number(num),
            Err(_) => FoodData::String(data.to_string())
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            FoodData::Number(number) => {
                if *number == 0.0 {
                    "-".to_string()
                } else {
                    number.to_string()

                }
            },
            FoodData::String(string) => string.clone(),
            _ => "-".to_string()
        }
    }
}


#[test]
fn test_food_data_from_str() {
    let food_data = FoodData::from_str("100");

    assert_eq!(food_data, FoodData::Number(100.0));
}


#[derive(Clone)]
pub struct Food {
    data_list: HashMap<String, FoodData>,
    weight: f32,
}

impl Food {
    const BASE_WEIGHT: f32 = 100.0;

    pub fn new() -> Food {
        Food {
            data_list: HashMap::new(),
            weight: 100.0,
        }
    }

    pub fn set_weight(&mut self, weight: f32) {
        self.weight = weight;
    }

    pub fn get(&self, key: &str) -> Option<FoodData> {
        let food_data = match self.data_list.get(key) {
            Some(food_data) => food_data,
            None => return None
        };

        match food_data {
            FoodData::String(_) => {
                return Some(food_data.clone())
            },
            FoodData::Number(num) => {
                let new_num = *num * (self.weight / Food::BASE_WEIGHT);
                return Some(FoodData::Number(new_num))
            }
        }

        None
    }

    pub fn get_list(&self, keys: &[&str]) -> Vec<Option<FoodData>> {
        let mut data_list = Vec::new();

        for key in keys {
            data_list.push(self.get(&key))
        }

        data_list
    }

    pub fn set(&mut self, key: &str, food_data: FoodData) {
        self.data_list.insert(key.to_string(), food_data);
    }
}


#[test]
fn test_food_new() {
    let food = Food::new();
}

#[test]
fn test_food_set_wight() {
    let mut food = Food::new();
    food.set_weight(50.0);
    assert_eq!(food.weight, 50.0);
    food.set_weight(0.0);
    assert_eq!(food.weight, 0.0);
}

#[test]
fn test_food__get() {
    let mut food = Food::new();
    food.set_weight(100.0);
    food.set("エネルギー", FoodData::Number(200.0));
    assert_eq!(food.get("エネルギー"), Some(FoodData::Number(200.0)));

    food.set_weight(50.0);
    assert_eq!(food.get("エネルギー"), Some(FoodData::Number(100.0)));

    food.set_weight(25.0);
    assert_eq!(food.get("エネルギー"), Some(FoodData::Number(50.0)));


    food.set("食品名", FoodData::String("ネギ".to_string()));
    assert_eq!(food.get("食品名"), Some(FoodData::String("ネギ".to_string())));
}