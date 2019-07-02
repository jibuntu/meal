use std::str::FromStr;

use clap::{Arg, App, SubCommand, ArgMatches, AppSettings};

mod food;
mod food_table;
use food_table::FoodTable;


enum SearchType {
    Or,
    And,
    Number
}

fn search(matches: &ArgMatches) {
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
        Err(e) => return println!("error: {}", e)
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
            .about("JSONから読み出して計算します"))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("search") {
        search(matches)
    } else if let Some(_matches) = matches.subcommand_matches("calc") {
        return println!("jsonファイルから読み出して計算します。");
    }


}
