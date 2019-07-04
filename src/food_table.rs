use std::fs::File;
use std::io::BufReader;

use serde_json::Value;
use prettytable::{Table, Row, Cell};

use crate::food::Food;
use crate::food::FoodData;
use crate::kijun::Kijun;
use std::collections::HashMap;


const KEY_LIST: [&str;68] = ["食品群", "食品番号", "索引番号", "食品名", "廃棄率", "エネルギー",
"エネルギー（kJ)", "水分", "たんぱく質", "アミノ酸組成によるたんぱく質", "脂質",
"トリアシルグリセロール当量", "飽和脂肪酸", "一価不飽和脂肪酸", "多価不飽和脂肪酸",
"コレステロール", "炭水化物", "利用可能炭水化物（単糖当量）", "水溶性食物繊維",
"不溶性食物繊維", "食物繊維総量", "灰   分", "ナトリウム", "カリウム", "カルシウム",
"マグネシウム", "リン", "鉄", "亜鉛", "銅", "マンガン", "ヨウ素", "セレン", "クロム",
"モリブデン", "レチノール", "α-カロテン", "β-カロテン", "β-クリプトキサンチン",
"β-カロテン当量", "レチノール活性当量", "ビタミンD", "α-トコフェロール",
"β-トコフェロール", "γ-トコフェロール", "δ-トコフェロール", "ビタミンK", "ビタミンB1",
"ビタミンB2", "ナイアシン", "ビタミンB6", "ビタミンB12", "葉酸", "パントテン酸", "ビオチン",
"ビタミンC", "食塩相当量", "アルコール", "硝酸イオン", "テオブロミン", "カフェイン",
"タンニン", "ポリフェノール", "酢酸", "調理油", "有機酸", "重量変化率", "備考"];

pub struct FoodTable {
    food_list: Vec<(String, Food)>,
}

impl FoodTable {
    pub fn new() -> FoodTable {
        FoodTable {
            food_list: Vec::new(),
        }
    }

    pub fn add(&mut self, food: Food) {
        let food_number = food.get("食品番号").unwrap().to_string();
        self.food_list.push((food_number, food));
    }

    pub fn from_json(path: &str) -> Result<FoodTable, String> {
        let mut food_table = FoodTable::new();
        let file = match File::open(path) {
            Ok(file) => file,
            Err(e) => return Err(e.to_string())
        };

        let reader = BufReader::new(file);
        let data: Value = match serde_json::from_reader(reader) {
            Ok(data) => data,
            Err(e) => return Err(e.to_string())
        };

        let obj = match data {
            Value::Object(obj) => obj,
            _ => return Err("Cannot load json".to_string())
        };

        for (_key, value) in obj {
            let arr = match value {
                Value::Array(arr) => arr,
                _ => return Err("Cannot load json".to_string())
            };

            if arr.len() != KEY_LIST.len() {
                return Err("Cannot load json".to_string())
            }

            let mut food = Food::new();

            // stringの部分をセット
            for i in 0..4 {
                let data = match &arr[i] {
                    Value::String(data) => data,
                    _ => return Err("Cannot load json".to_string())
                };
                food.set(KEY_LIST[i], FoodData::String(data.to_string()))
            }

            // それ以降をセット
            for i in 4..KEY_LIST.len() {
                let data = match &arr[i] {
                    Value::String(data) => data,
                    _ => return Err("Cannot load json".to_string())
                };
                food.set(KEY_LIST[i], FoodData::from_str(data))
            }


            food.set("重量", FoodData::String("100".to_string()));
            food_table.add(food)
        }

        Ok(food_table)
    }

    pub fn get(&self, search_key: &str) -> Option<&Food> {
        for (key, food) in &self.food_list {
            if key == search_key {
                return  Some(food);
            }
        }
        None
    }

    pub fn get_list(&self, keys: &[&str]) -> FoodTable {
        let mut food_table = FoodTable::new();

        for key in keys {
            if let Some(food) = self.get(key) {
                food_table.add(food.clone())
            }
        }

        food_table
    }

    pub fn get_sum(&self, keys: &[&str]) -> Vec<FoodData> {
        let mut sum = vec![0_f32; keys.len()];

        for (index, key) in keys.iter().enumerate() {
            for (_num, food) in &self.food_list {
                let food_data = match food.get(key) {
                    Some(food_data) => food_data,
                    None => continue
                };

                if let FoodData::Number(data) = food_data {
                    sum[index] += data;
                }
            }
        }

        sum.iter().map(|data| FoodData::Number(*data)).collect()
    }

    pub fn set_weight(&mut self, weight: f32) {
        for (key, food) in &mut self.food_list {
            food.set_weight(weight);
            food.set("重量", FoodData::String(weight.to_string()));
        }
    }

    pub fn search(&self, text: &str) -> FoodTable {
        let mut food_table = FoodTable::new();

        for (_key, food) in &self.food_list {
            match food.get("食品名") {
                Some(name) => {
                    if name.to_string().find(text).is_some() {
                        food_table.add(food.clone());
                    }
                }
                None => ()
            }
        }

        food_table
    }

    pub fn search_or(&self, text_list: &[&str]) -> FoodTable {
        let mut food_table = FoodTable::new();

        for (_key, food) in &self.food_list {
            match food.get("食品名") {
                Some(name) => {
                    for text in text_list {
                        if name.to_string().find(text).is_some() {
                            food_table.add(food.clone());
                            break
                        }
                    }
                }
                None => ()
            }
        }

        food_table
    }

    pub fn search_and(&self, text_list: &[&str]) -> FoodTable {
        let mut food_table = FoodTable::new();

        for (_key, food) in &self.food_list {
            match food.get("食品名") {
                Some(name) => {
                    let is_match = text_list.iter().all(|text| name.to_string().find(text).is_some());
                    if is_match {
                        food_table.add(food.clone());
                    }
                }
                None => ()
            }
        }

        food_table
    }

    pub fn get_table(&self, name_list: &[&str]) -> Table {
        let mut table = Table::new();
        table.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);


        let header: Vec<_> = name_list.iter().map(|n| {
            let name = n.chars()
                .map(|c| c.to_string() + "\n").collect::<String>();
            let mut cell = Cell::new(&name);
            cell.align(prettytable::format::Alignment::CENTER);
            cell

        }).collect();

        table.set_titles(Row::new(header));

        for (key, food) in &self.food_list {
            let mut row = Vec::new();
            for (food_data, name)
                in food.get_list(name_list).iter().zip(name_list.iter()) {
                let mut cell = match food_data {
                    Some(food_data) => Cell::new(&food_data.to_string()),
                    None => Cell::new("-")
                };
                if *name == "食品名" {
                    cell.align(prettytable::format::Alignment::LEFT);
                } else {
                    cell.align(prettytable::format::Alignment::RIGHT);
                }
                row.push(cell);
            }
            table.add_row(Row::new(row));
        }

        table
    }

    pub fn add_sum_to_table(&self, table: &mut Table, name_list: &[&str]) {
        // 合計を追加する
        let mut row = Vec::new();
        let sum = self.get_sum(name_list);
        for (name, food_data) in name_list.iter().zip(sum.iter()) {
            if *name == "食品名" {
                row.push(Cell::new("合計"));
            } else {
                let mut cell = Cell::new(&food_data.to_string());
                cell.align(prettytable::format::Alignment::RIGHT);
                row.push(cell);
            }
        }

        table.add_row(Row::new(row));
    }

    pub fn add_kijun_to_table(&self, table: &mut Table, name_list: &[&str], kijun: Kijun) {
        // 摂取基準を追加する
        let mut row = Vec::new();
        let kijun_values = kijun.get_list(name_list);
        for (name, value) in name_list.iter().zip(kijun_values.iter()) {
            if *name == "食品名" {
                row.push(Cell::new("摂取基準値"));
            } else {
                let data = match value {
                    Some(v) => v.to_string(),
                    None => "-".to_string()
                };
                let mut cell = Cell::new(&data);
                cell.align(prettytable::format::Alignment::RIGHT);
                row.push(cell);
            }
        }
    }

    pub fn print(&self, name_list: &[&str]) {
        let mut table = self.get_table(name_list);

        table.printstd();
    }

    pub fn print_with_sum(&self, name_list: &[&str]) {
        let mut table = self.get_table(name_list);
        self.add_sum_to_table(&mut table, name_list);

        table.printstd();
    }

    pub fn print_with_sum_and_kijun(&self, name_list: &[&str], kijun: Kijun) {
        let mut table = self.get_table(name_list);
        self.add_sum_to_table(&mut table, name_list);
        self.add_kijun_to_table(&mut table, name_list, kijun);

        table.printstd();
    }
}


#[test]
fn test_food_table_new() {
    let food_table = FoodTable::new();
}

#[test]
fn test_food_table_add() {
    let mut food_table = FoodTable::new();
    let mut food = Food::new();
    food.set("食品番号", FoodData::from_str("0"));
    food.set("食品名", FoodData::from_str("麦ごはん"));
    food_table.add(food);

    let food = &food_table.food_list[0].1;
    assert_eq!(food.get("食品名"), Some(FoodData::from_str("麦ごはん")));
    assert_eq!(food.get("食品群"), None);
}

#[test]
fn test_food_table_from_json() {
    let food_table = FoodTable::from_json("./test/test_foods.json").unwrap();
    for (key, _food) in &food_table.food_list {
        match key.as_str() {
            "01001" | "01002" | "01003" | "01004" | "01005" |
            "01006" | "18015" | "18016" | "18022" => (),
            _ => panic!(format!("key is {}", key))
        }
    }
    let food = food_table.get("01001").unwrap();
    assert_eq!(food.get("食品群").unwrap(), FoodData::String("01".to_string()));
}

#[test]
fn test_food_table_get() {
    let food_table = FoodTable::from_json("./test/test_foods.json").unwrap();
    let food = food_table.get("01001").unwrap();
    assert_eq!(food.get("食品名").unwrap(), FoodData::String("アマランサス　玄穀".to_string()))
}

#[test]
fn test_food_table_get_list() {
    let food_table = FoodTable::from_json("./test/test_foods.json").unwrap();
    let food_table = food_table.get_list(&["01001", "01002", "18022"]);
    let keys: Vec<_> =  food_table.food_list.iter().map(|(key, food)| key).collect();
    assert!(keys.contains(&&"01001".to_string()));
    assert!(keys.contains(&&"01002".to_string()));
    assert!(keys.contains(&&"18022".to_string()));
}

#[test]
fn test_food_table_get_sum() {
    let food_table = FoodTable::from_json("./test/test_foods.json").unwrap();
    let food_table = food_table.get_list(&["01001", "01002", "01003", "01004"]);
    let sum = food_table.get_sum(&["たんぱく質"]);
    assert_eq!(sum[0].to_string(), "42.7");
    let sum = food_table.get_sum(&["食品名"]);
    assert_eq!(sum[0].to_string(), "-");
}

#[test]
fn test_food_table_search() {
    let food_table = FoodTable::from_json("./test/test_foods.json").unwrap();
    let food_table = food_table.search("むぎ");
    for (key, _food) in &food_table.food_list {
        match key.as_str() {
            "01005" | "01006" => (),
            _ => panic!(format!("key is {}", key))
        }
    }
}

#[test]
fn test_food_table_search_or() {
    let food_table = FoodTable::from_json("./test/test_foods.json").unwrap();
    let food_table = food_table.search_or(&["あわ", "むぎ"]);
    let keys: Vec<_> =  food_table.food_list.iter().map(|(key, food)| key).collect();
    assert!(keys.contains(&&"01002".to_string()));
    assert!(keys.contains(&&"01003".to_string()));
    assert!(keys.contains(&&"01005".to_string()));
    assert!(keys.contains(&&"01006".to_string()));
}

#[test]
fn test_food_table_search_and() {
    let food_table = FoodTable::from_json("./test/test_foods.json").unwrap();
    let food_table = food_table.search_and(&["冷凍", "メンチ"]);
    let keys: Vec<_> =  food_table.food_list.iter().map(|(key, food)| key).collect();
    assert!(keys.contains(&&"18016".to_string()));
    assert!(keys.contains(&&"18022".to_string()));
}