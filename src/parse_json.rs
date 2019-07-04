use std::io::{BufReader, Read};

use serde_json::Value;

use crate::kijun::{Gender, PAL};

pub struct ParsedData {
    pub foods: Vec<ParsedFood>,
    pub name_list: Vec<String>,
    pub body: Option<Body>
}

pub struct ParsedFood {
    pub number: String,
    pub weight: Option<f32>
}

pub struct Body {
    age: usize,
    weight: f32,
    height: f32,
    gender: Gender,
    pal: PAL
}

fn parse_foods(data: &Value) -> Result<Vec<ParsedFood>, String> {
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

                ParsedFood {
                    number,
                    weight
                }
            },
            _ => return Err("foodsの値はオブジェクトの配列にしてください".to_string())
        };

        parsed_foods.push(parsed_food)
    }

    Ok(parsed_foods)
}

fn parse_name_list(data: &Value) -> Result<Vec<String>, String> {
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

fn parse_body(data: &Value) -> Result<Body, String> {
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

    let gender = match obj.get("height") {
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


    Ok(Body {
        age,
        weight,
        height,
        gender,
        pal
    })
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
        None => None,
        Some(value) => match parse_body(value) {
            Ok(body) => Some(body),
            Err(e) => return Err(e)
        }
    };


    Ok(ParsedData {
        foods,
        name_list,
        body
    })
}