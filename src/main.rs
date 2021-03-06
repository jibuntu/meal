use std::str::FromStr;
use std::fs::File;
use std::io::BufReader;

use clap::{Arg, App, SubCommand, ArgMatches, AppSettings};

mod food;
mod food_table;
mod kijun;
mod parse_json;
mod automatic_selection;
mod combination;
use food_table::FoodTable;
use food::food_data::FoodData;
use parse_json::parse_json;
use crate::kijun::Kijun;

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

    let mut name_list = vec!["食品番号".to_string(), "食品名".to_string()];
    if let Some(values) = matches.values_of("column") {
        values.into_iter().for_each(|v| name_list.push(v.to_string()));
    }

    let text_list: Vec<_> = matches.values_of("text").unwrap().collect();
    match matches.value_of("column-type") {
        None => (),
        Some(t) => match t {
            "0" => [
                "廃棄率", "エネルギー", "エネルギー（kJ)", "水分", "たんぱく質",
                "アミノ酸組成によるたんぱく質", "脂質",
                "トリアシルグリセロール当量", "飽和脂肪酸", "一価不飽和脂肪酸",
                "多価不飽和脂肪酸", "コレステロール", "炭水化物",
                "利用可能炭水化物（単糖当量）", "水溶性食物繊維", "不溶性食物繊維",
                "食物繊維総量", "灰   分", "ナトリウム", "カリウム", "カルシウム",
                "マグネシウム", "リン", "鉄", "亜鉛", "銅", "マンガン", "ヨウ素",
                "セレン", "クロム", "モリブデン", "レチノール", "α-カロテン",
                "β-カロテン", "β-クリプトキサンチン", "β-カロテン当量",
                "レチノール活性当量", "ビタミンD", "α-トコフェロール",
                "β-トコフェロール", "γ-トコフェロール", "δ-トコフェロール",
                "ビタミンK", "ビタミンB1", "ビタミンB2", "ナイアシン",
                "ビタミンB6", "ビタミンB12", "葉酸", "パントテン酸", "ビオチン",
                "ビタミンC", "食塩相当量", "アルコール", "硝酸イオン",
                "テオブロミン", "カフェイン", "タンニン", "ポリフェノール", "酢酸",
                "調理油", "有機酸", "重量変化率"
            ].iter().for_each(|v| name_list.push(v.to_string())),
            "1" => [
                "重量", "エネルギー", "たんぱく質", "脂質", "炭水化物",
                "食物繊維総量", "ナトリウム", "カルシウム", "鉄",
                "レチノール活性当量", "ビタミンB1", "ビタミンB2", "ビタミンC"
            ].iter().for_each(|v| name_list.push(v.to_string())),
            "2" => [
                "重量", "エネルギー", "たんぱく質", "脂質", "多価不飽和脂肪酸",
                "炭水化物", "食物繊維総量", "レチノール活性当量", "ビタミンD",
                "α-トコフェロール", "ビタミンK", "ビタミンB1", "ビタミンB2",
                "ナイアシン", "ビタミンB6", "ビタミンB12", "葉酸", "パントテン酸",
                "ビオチン", "ビタミンC", "ナトリウム", "カリウム", "カルシウム",
                "マグネシウム", "リン", "鉄", "亜鉛", "銅", "マンガン", "ヨウ素",
                "セレン", "クロム", "モリブデン"
            ].iter().for_each(|v| name_list.push(v.to_string())),
            _ => ["食品番号", "食品名"].iter().for_each(|v| name_list.push(v.to_string())),
        }
    };

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

    if let Some(name) = matches.value_of("ascending-order") {
        foods.sort_ascending_order(name);
    }

    if let Some(name) = matches.value_of("descending-order") {
        foods.sort_descending_order(name);
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

fn print_table(path: &str, foods: &FoodTable) -> Result<(), String> {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => return Err(e.to_string())
    };

    let parsed_data = match parse_json(BufReader::new(file)) {
        Ok(parsed_data) => parsed_data,
        Err(e) => return Err(e)
    };

    let mut udf_table = FoodTable::new();
    if let Some(udfs) = parsed_data.user_definition_foods {
        for udf in udfs {
            let mut food = food::Food::new();
            // 食品番号の先頭にuをつける
            // すでにある食品番号と被らないように
            food.set("食品番号", FoodData::String("u".to_string() + &udf.number));
            food.set("重量", FoodData::Number(udf.weight));
            for (name, food_data) in udf.data {
                food.set(&name, FoodData::from_str(&food_data))
            }
            udf_table.add(food)
        }
    }

    let mut food_table = FoodTable::new();
    for parsed_food in parsed_data.foods {
        let mut food = match foods.get(&parsed_food.number) {
            Some(food) => food.change_weight(parsed_food.weight.unwrap_or(100.0)).unwrap(),
            None => match udf_table.get(&parsed_food.number) {
                Some(food) => food.change_weight(parsed_food.weight.unwrap_or(100.0)).unwrap(),
                None => return Err(format!("{}番の食材はありません。JSONの値が間違っています", &parsed_food.number))
            }
        };

        if let Some(price) = parsed_food.price {
            food.set("価格", FoodData::Number(price));
        }

        if let Some(class) = parsed_food.class {
            food.set("クラス", FoodData::String(class));
        }

        if parsed_food.include_refuse == true {
            food = match food.include_refuse() {
                Some(food) => food,
                None => return Err(format!("{}番の食材は廃棄部分を含めることはできません", &parsed_food.number))
            };
        }

        food_table.add(food);
    }

    let list: Vec<_> = parsed_data.name_list.iter().map(|name| name.as_str()).collect();

    let kijun = Kijun::new(parsed_data.body.age,
                           parsed_data.body.weight,
                           parsed_data.body.height,
                           parsed_data.body.gender,
                           parsed_data.body.pal,
                           parsed_data.body.days.unwrap_or(1));
    food_table.print_with_sum_and_kijun(&list, &kijun);

//    if parsed_data.options.show_status {
//        println!("{}", food_table.get_status());
//    }
    
    println!();

    Ok(())
}

fn calc(matches: &ArgMatches) -> Result<(), String>{
    let path = "/home/jibuntu/programming_language/rust/project/meal/data/foods.json";
    let foods = match FoodTable::from_json(path) {
        Ok(foods) => foods,
        Err(e) => return Err(e.to_string())
    };

    for file_name in matches.values_of("file").unwrap() {
        if let Err(e) = print_table(file_name, &foods) {
            return Err(e);
        }
    }

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
            .arg(Arg::with_name("ascending-order")
                .long("12")
                .takes_value(true)
                .max_values(1)
                .help("結果を昇順ソートします。値は列名です。"))
            .arg(Arg::with_name("descending-order")
                .long("21")
                .takes_value(true)
                .max_values(1)
                .help("結果を降順ソートします。値は列名です。"))
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
                .min_values(1)
                .required(true)
                .help("ファイルを指定します")))
        .subcommand(SubCommand::with_name("automatic-selection")
            .visible_alias("a")
            .setting(AppSettings::DeriveDisplayOrder)
            .about("自動的に食材を選択します")
            .arg(Arg::with_name("file")
                .takes_value(true)
                .required(true)
                .help("ファイルを指定します")))
        .get_matches();

    let result = if let Some(matches) = matches.subcommand_matches("search") {
        search(matches)
    } else if let Some(matches) = matches.subcommand_matches("calc") {
        calc(matches)
    } else if let Some(matches) = matches.subcommand_matches("automatic-selection") {
        automatic_selection::automatic_selection(matches)
    } else {
        Err("サブコマンドが間違っています".to_string())
    };

    if let Err(e) = result {
        println!("error: {}", e);
    }


}
