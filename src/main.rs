use std::str::FromStr;
use std::fs::File;
use std::io::{BufReader, Read};

use clap::{Arg, App, SubCommand, ArgMatches, AppSettings};
use serde_json::Value;

mod food;
mod food_table;
use food_table::FoodTable;
use crate::food::FoodData;


enum SearchType {
    Or,
    And,
    Number
}

fn search(matches: &ArgMatches) -> Result<(), String> {
    let search_type = if matches.is_present("search-or") {
        SearchType::Or
    } else if matches.is_present("search-and") {
        SearchType::And
    } else if matches.is_present("search-number") {
        SearchType::Number
    } else {
        SearchType::And
    };

    let text_list: Vec<_> = matches.values_of("text").unwrap().collect();
    let mut name_list = match matches.value_of("column-type") {
        None => vec!["食品番号".to_string(), "食品名".to_string()],
        Some(t) => match t {
            "0" => ["食品番号", "食品名", "重量", "エネルギー", "たんぱく質",
                    "脂質", "炭水化物", "食物繊維総量", "ナトリウム", "カルシウム",
                    "鉄", "レチノール活性当量", "ビタミンB1", "ビタミンB2",
                    "ビタミンC"].iter().map(|n| n.to_string()).collect(),
            "1" => ["食品番号", "食品名", "重量", "エネルギー", "たんぱく質",
                    "脂質", "炭水化物", "食物繊維総量", "レチノール活性当量",
                    "ビタミンD", "α-トコフェロール", "ビタミンK", "ビタミンB1",
                    "ビタミンB2", "ナイアシン", "ビタミンB6", "ビタミンB12",
                    "葉酸", "パントテン酸", "ビオチン", "ビタミンC", "ナトリウム",
                    "カリウム", "カルシウム", "マグネシウム", "リン", "鉄",
                    "亜鉛", "銅", "マンガン", "ヨウ素", "セレン", "クロム",
                    "モリブデン"].iter().map(|n| n.to_string()).collect(),

            _ => vec!["食品番号".to_string(), "食品名".to_string()],
        }
    };

    if let Some(values) = matches.values_of("column") {
        let mut ns: Vec<_> = values.into_iter().map(|value| value.to_string()).collect();
        name_list.append(&mut ns);
    }

    let path = "/home/jibuntu/programming_language/rust/project/meal/data/foods.json";
    let mut foods = match FoodTable::from_json(path) {
        Ok(foods) => foods,
        Err(e) => return Err(e.to_string())
    };

    if let Some(weight) = matches.value_of("weight") {
        match f32::from_str(weight) {
            Ok(num) => foods.set_weight(num),
            _ => ()
        }
    }

    let result = match search_type {
        SearchType::Or => foods.search_or(&text_list),
        SearchType::And => foods.search_and(&text_list),
        SearchType::Number => foods.get_list(&text_list)
    };

    let list: Vec<_> = name_list.iter().map(|name| name.as_str()).collect();
    result.print(&list);

    Ok(())
}

struct ParsedData {
    foods: Vec<ParsedFood>,
    name_list: Vec<String>
}

struct ParsedFood {
    number: String,
    weight: Option<f32>
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

fn parse_json<T: std::io::Read>(reader: BufReader<T>) -> Result<ParsedData, String> {
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


    Ok(ParsedData {
        foods,
        name_list
    })
}

fn calc(matches: &ArgMatches) -> Result<(), String>{
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
    food_table.print(&list);

    Ok(())
}

fn main() {
    let matches = App::new("meal")
        .version("0.0")
        .about("栄養の計算、食品の検索")
        .author("Jibuntu")
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(SubCommand::with_name("search")
            .visible_alias("s")
            .setting(AppSettings::DeriveDisplayOrder)
            .about("食品を検索します")
            .arg(Arg::with_name("text")
                .min_values(1)
                .required(true))
            .arg(Arg::with_name("search-or")
                .long("or")
                .short("o")
                .help("OR検索をします"))
            .arg(Arg::with_name("search-and")
                .long("and")
                .short("a")
                .help("AND検索をします"))
            .arg(Arg::with_name("search-number")
                .long("number")
                .short("n")
                .help("食品番号がtextのものを取得します"))
            .arg(Arg::with_name("weight")
                .long("weight")
                .short("w")
                .takes_value(true)
                .help("重量を指定します"))
            .arg(Arg::with_name("column")
                .long("column")
                .short("c")
                .takes_value(true)
                .min_values(1)
                .help(r#"表示する列を指定します。食品番号と食品名はデフォルトです。"#))
            .arg(Arg::with_name("column-type")
                .long("type")
                .short("t")
                .min_values(1)
                .help("columnのデフォルトの種類を設定します\n\
                      0: 「調理・献立作成の基礎」と同じ設定にします\n\
                      1: 摂取基準と同じ設定にします")))
        .subcommand(SubCommand::with_name("calc")
            .visible_alias("c")
            .setting(AppSettings::DeriveDisplayOrder)
            .about("JSONから読み出して計算します")
            .arg(Arg::with_name("file")
                .takes_value(true)
                .required(true)
                .help("ファイルを指定します")))
        .get_matches();

    let result = if let Some(matches) = matches.subcommand_matches("search") {
        search(matches)
    } else if let Some(matches) = matches.subcommand_matches("calc") {
        calc(matches)
    } else {
        Err("サブコマンドが間違っています".to_string())
    };

    if let Err(e) = result {
        println!("error: {}", e);
    }


}
