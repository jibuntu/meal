use std::io::{BufReader, Read};

use serde_json::Value;

use crate::kijun::{Gender, PAL};

macro_rules! value_or_error {
    ($option:expr, $error:expr) => {
        match $option {
            Some(value) => value,
            None => return Err($error .to_string())
        }
    };
}

pub struct ParsedData {
    pub foods: Vec<ParsedFood>,
    pub name_list: Vec<String>,
    pub body: Body,
    pub comb: Option<Vec<usize>>
}

pub struct ParsedFood {
    pub number: String,
    pub weight: Option<f32>,
    pub price: Option<f32>,
    pub include_refuse: bool
}

pub struct Body {
    pub age: usize,
    pub weight: f32,
    pub height: f32,
    pub gender: Gender,
    pub pal: PAL,
    pub days: Option<usize>
}

pub fn parse_foods(data: &Value) -> Result<Vec<ParsedFood>, String> {
    let food_list = match data {
        Value::Array(food_list) => food_list,
        _ => return Err("foodsの値はオブジェクトの配列にしてください".to_string())
    };

    let mut parsed_foods = Vec::new();

    for food in food_list {
        let parsed_food = match food {
            Value::Object(obj) => {
                let number = match obj.get("number") {
                    Some(number) => {
                        match number {
                            Value::String(number) => number.clone(),
                            _ => return Err("numberの値は文字列にしてください".to_string())
                        }
                    },
                    None => return Err("foods配列のオブジェクトにnumber属性がありません".to_string())
                };

                let weight = match obj.get("weight") {
                    Some(weight) => {
                        match weight {
                            Value::Number(weight) => match weight.as_f64() {
                                Some(number) => Some(number as f32),
                                _ => return Err("weightの値をf64に変換できません".to_string())
                            },
                            _ => return Err("weightの値は数値にしてください".to_string())
                        }
                    },
                    None => None
                };

                let price = match obj.get("price") {
                    Some(price) => {
                        let price = value_or_error!(price.as_f64(), "priceの値をf64に変換できません");
                        Some(price as f32)
                    },
                    _ => None
                };

                let include_refuse = match obj.get("include_refuse") {
                    Some(include_refuse) => value_or_error!(include_refuse.as_bool(), "include_refuseの値はboolにしてください"),
                    _ => false
                };

                ParsedFood {
                    number,
                    weight,
                    price,
                    include_refuse
                }
            },
            _ => return Err("foodsの値はオブジェクトの配列にしてください".to_string())
        };

        parsed_foods.push(parsed_food)
    }

    Ok(parsed_foods)
}

pub fn parse_name_list(data: &Value) -> Result<Vec<String>, String> {
    let mut parsed_name_list = Vec::new();
    let name_list = match data {
        Value::Array(name_list) => name_list,
        _ => return Err("name_listの値は文字列の配列にしてください".to_string())
    };

    for name in name_list {
        let parsed_name = match name {
            Value::String(parsed_name) => parsed_name.clone(),
            _ => return Err("name_listの値は文字列の配列にしてください".to_string())
        };

        parsed_name_list.push(parsed_name)
    }

    Ok(parsed_name_list)
}

pub fn parse_body(data: &Value) -> Result<Body, String> {
    let obj = match data {
        Value::Object(obj) => obj,
        _ => return Err("bodyの値はオブジェクトにしてください".to_string())
    };

    let age = match obj.get("age") {
        Some(value) => match value {
            Value::Number(num) => match num.as_f64() {
                Some(num) => num as usize,
                _ => return Err("ageの値をf64に変換できません".to_string())
            },
            _ => return Err("ageの値は数値にしてください".to_string())
        },
        None => return Err("bodyにage属性がありません".to_string())
    };

    let weight = match obj.get("weight") {
        Some(value) => match value {
            Value::Number(num) => match num.as_f64() {
                Some(num) => num as f32,
                _ => return Err("weightの値をf64に変換できません".to_string())
            },
            _ => return Err("weightの値は数値にしてください".to_string())
        },
        None => return Err("bodyにweight属性がありません".to_string())
    };

    let height = match obj.get("height") {
        Some(value) => match value {
            Value::Number(num) => match num.as_f64() {
                Some(num) => num as f32,
                _ => return Err("heightの値をf64に変換できません".to_string())
            },
            _ => return Err("heightの値は数値にしてください".to_string())
        },
        None => return Err("bodyにheight属性がありません".to_string())
    };

    let gender = match obj.get("gender") {
        Some(value) => match value {
            Value::String(gender) => match gender.as_str() {
                "female" => Gender::Female,
                "male" => Gender::Male,
                _ => return Err("genderの値は文字列の \"female\" または \"male\"\
                             にしてください".to_string())
            },
            _ => return Err("genderの値は文字列の \"female\" または \"male\"\
                             にしてください".to_string())
        },
        None => return Err("bodyにgender属性がありません".to_string())
    };

    let pal = match obj.get("pal") {
        Some(value) => match value {
            Value::String(pal) => match pal.as_str() {
                "low" => PAL::Low,
                "moderate" => PAL::Moderate,
                "high" => PAL::High,
                _ => return Err("palの値は \"low\", \"moderate\", \"high\" \
                             のいずれかの文字列にしてください".to_string())
            },
            _ => return Err("palの値は \"low\", \"moderate\", \"high\" \
                             のいずれかの文字列にしてください".to_string())
        },
        None => return Err("bodyにpal属性がありません".to_string())
    };

    let days = match obj.get("days") {
        Some(value) => Some(value_or_error!(value.as_u64(), "daysの値をu64に変換できません") as usize),
        None => None
    };


    Ok(Body {
        age,
        weight,
        height,
        gender,
        pal,
        days
    })
}

pub fn parse_combination(data: &Value) -> Result<Vec<usize>, String> {
    let mut values = Vec::new();
    let arr = value_or_error!(data.as_array(), "combinationの値は配列にしてください");

    for value in arr {
        let num = value_or_error!(value.as_i64(), "combinationの配列の値は数値にしてください");
        values.push(num as usize);
    }

    Ok(values)
}

pub fn parse_json<T: std::io::Read>(reader: BufReader<T>) -> Result<ParsedData, String> {
    let data: Value = match serde_json::from_reader(reader) {
        Ok(data) => data,
        Err(e) => return Err(e.to_string())
    };

    let obj = match data {
        Value::Object(obj) => obj,
        _ => return Err("Cannot load json".to_string())
    };

    let foods = match obj.get("foods") {
        None => return Err("foods属性がありません".to_string()),
        Some(value) => match parse_foods(value) {
            Ok(foods) => foods,
            Err(e) => return Err(e)
        }
    };

    let name_list = match obj.get("name_list") {
        None => return Err("name_list属性がありません".to_string()),
        Some(value) => match parse_name_list(value) {
            Ok(name_list) => name_list,
            Err(e) => return Err(e)
        }
    };

    let body = match obj.get("body") {
        None => return Err("body属性がありません".to_string()),
        Some(value) => match parse_body(value) {
            Ok(body) => body,
            Err(e) => return Err(e)
        }
    };

    let comb = match obj.get("combination") {
        None => None,
        Some(value) => match parse_combination(value) {
            Ok(comb) => Some(comb),
            Err(e) => return Err(e)
        }
    };


    Ok(ParsedData {
        foods,
        name_list,
        body,
        comb
    })
}

#[test]
fn test_parse_foods() {
    let test_json = r#"{
    "foods": [
        {"number": "04047"},
        {"number": "01083", "weight": 112},
        {"number": "12004", "weight": 50, "include_refuse": true}
    ],
    "name_list": ["食品番号", "食品名", "重量", "廃棄率", "エネルギー", "多価不飽和脂肪酸", "ビタミンC", "脂質"],
    "body": {
        "age": 20,
        "weight": 50,
        "height": 160,
        "gender": "male",
        "pal": "low"
    }
}"#;
    let reader = BufReader::new(test_json.as_bytes());
    let parsed_data = parse_json(reader).unwrap();

    assert_eq!(parsed_data.foods[0].number, "04047".to_string());
    assert_eq!(parsed_data.foods[0].weight, None);
    assert_eq!(parsed_data.foods[0].include_refuse, false);
    
    assert_eq!(parsed_data.foods[1].number, "01083".to_string());
    assert_eq!(parsed_data.foods[1].weight, Some(112.0));
    assert_eq!(parsed_data.foods[1].include_refuse, false);
   
    assert_eq!(parsed_data.foods[2].number, "12004".to_string());
    assert_eq!(parsed_data.foods[2].weight, Some(50.0));
    assert_eq!(parsed_data.foods[2].include_refuse, true);
}