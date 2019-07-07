use std::str::FromStr;
use std::fs::File;
use std::io::BufReader;

use clap::{Arg, App, SubCommand, ArgMatches, AppSettings};

mod food;
mod food_table;
mod kijun;
mod parse_json;
use food_table::FoodTable;
use crate::food::FoodData;
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

    let text_list: Vec<_> = matches.values_of("text").unwrap().collect();
    let mut name_list = match matches.value_of("column-type") {
        None => vec!["食品番号".to_string(), "食品名".to_string()],
        Some(t) => match t {
            "0" => [
                "食品群", "食品番号", "索引番号", "食品名", "廃棄率", "エネルギー",
                "エネルギー（kJ)", "水分", "たんぱく質",
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
            ].iter().map(|n| n.to_string()).collect(),
            "1" => [
                "食品番号", "食品名", "重量", "エネルギー", "たんぱく質", "脂質",
                "炭水化物", "食物繊維総量", "ナトリウム", "カルシウム", "鉄",
                "レチノール活性当量", "ビタミンB1", "ビタミンB2", "ビタミンC"
            ].iter().map(|n| n.to_string()).collect(),
            "2" => [
                "食品番号", "食品名", "重量", "エネルギー", "たんぱく質", "脂質",
                "多価不飽和脂肪酸", "炭水化物", "食物繊維総量",
                "レチノール活性当量", "ビタミンD", "α-トコフェロール", "ビタミンK",
                "ビタミンB1", "ビタミンB2", "ナイアシン", "ビタミンB6",
                "ビタミンB12", "葉酸", "パントテン酸", "ビオチン", "ビタミンC",
                "ナトリウム", "カリウム", "カルシウム", "マグネシウム", "リン",
                "鉄", "亜鉛", "銅", "マンガン", "ヨウ素", "セレン", "クロム",
                "モリブデン"
            ].iter().map(|n| n.to_string()).collect(),

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

    if let Some(body) = parsed_data.body {
        let kijun = Kijun::new(body.age,
                               body.weight,
                               body.height,
                               body.gender,
                               body.pal);
        food_table.print_with_sum_and_kijun(&list, &kijun);
    } else {
        food_table.print_with_sum(&list);
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
    } else {
        Err("サブコマンドが間違っています".to_string())
    };

    if let Err(e) = result {
        println!("error: {}", e);
    }


}
