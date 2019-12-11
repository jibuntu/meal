#![allow(dead_code)]

use std::fs::File;
use std::io::BufReader;
use std::slice::Iter;
use std::collections::HashMap;

use serde_json::Value;
use prettytable::{Table, Row, Cell};

use crate::food::KEY_LIST as FOOD_KEY_LIST;
use crate::food::Food;
use crate::food::food_data::FoodData;
use crate::kijun::KEY_LIST as KIJUN_KEY_LIST;
use crate::kijun::{Kijun, KijunValue};

macro_rules! value_or_error {
    ($option:expr, $error:expr) => {
        match $option {
            Some(value) => value,
            None => return Err($error .to_string())
        }
    };
}

fn color(text: &str, style: &str) -> String {
    let mut colored_text = String::new();
    let unset_style = "\x1b[0m";

    for c in style.chars() {
        let color_style = match c {
            'd' => "\x1b[30m",
            'r' => "\x1b[31m",
            'g' => "\x1b[32m",
            'y' => "\x1b[33m",
            'b' => "\x1b[34m",
            'm' => "\x1b[35m",
            'c' => "\x1b[36m",
            'w' => "\x1b[37m",
            'D' => "\x1b[90m",
            'R' => "\x1b[91m",
            'G' => "\x1b[92m",
            'Y' => "\x1b[93m",
            'B' => "\x1b[94m",
            'M' => "\x1b[95m",
            'C' => "\x1b[96m",
            'W' => "\x1b[97m",
            '+' => "\x1b[1m",
            _ => continue
        };

        colored_text += color_style;
    }

    colored_text + text + unset_style
}

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

        let obj = value_or_error!(data.as_object(), "jsonの値はオブジェクにしてください");
        let foods = value_or_error!(obj.get("foods"), "jsonのオブジェクトにfoods属性がありません");
        let food_list = value_or_error!(foods.as_array(), "foods属性の値は配列にしてください");


        for food_data in food_list {
            let value_list = value_or_error!(food_data.as_array(), "foods属性の配列の値は配列にしてください");
            let mut food = Food::new();

            // stringの部分をセット
            for (value, key) in value_list[0..4].iter().zip(FOOD_KEY_LIST[1..5].iter()) {
                let data = value_or_error!(value.as_str(), "foods属性の値が読み込めません");
                food.set(key, FoodData::String(data.to_string()));
            }

            // それ以降をセット
            let len = FOOD_KEY_LIST.len() - 1;
            for (value, key) in value_list[4..len].iter().zip(FOOD_KEY_LIST[5..len].iter()) {
                let data = value_or_error!(value.as_str(), "foods属性の値が読み込めません");
                food.set(key, FoodData::from_str(data));
            }

            food.set("価格", FoodData::String("-".to_string()));
            food.set("重量", FoodData::Number(100.0));
            if let Some(refuse) = food.refuse.get_number() {
                food.set("可食量", FoodData::Number(100.0 - *refuse));
            }            
            food_table.add(food);
        }

        Ok(food_table)
    }

    pub fn iter(&self) -> Iter<(String, Food)> {
        self.food_list.iter()
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

    pub fn get_sum(&self) -> Food {
        let mut sum = Food::new();

        for (_num, food) in &self.food_list {
            sum = sum.add(&food);
        }

        sum
    }

    pub fn set_weight(&mut self, weight: f32) {
        for (_, food) in &mut self.food_list {
            if let Some(new_food) = food.change_weight(weight) {
                *food = new_food;
            }
        }
    }

    pub fn split_by_class(&self) -> HashMap<String, FoodTable> {
        let mut class_list = HashMap::new();
        
        for (_, food) in &self.food_list {
            let class_name = if let FoodData::String(s) = food.get("クラス").unwrap() {
                s
            } else {
                continue
            };
            
            if !class_list.contains_key(class_name) {
                let mut food_table = FoodTable::new();
                food_table.add(food.clone());
                class_list.insert(class_name.to_string(), food_table);
            } else {
                class_list.get_mut(class_name).unwrap().add(food.clone())
            }
        }

        class_list
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

    pub fn sort_ascending_order(&mut self, name: &str) {
        self.food_list.sort_by(|(_, food_a), (_, food_b)| {
            let a = food_a.get(name).unwrap_or(&FoodData::None).get_number().unwrap_or(&0.0);
            let b = food_b.get(name).unwrap_or(&FoodData::None).get_number().unwrap_or(&0.0);

            a.partial_cmp(b).unwrap()
        })
    }

    pub fn sort_descending_order(&mut self, name: &str) {
        self.food_list.sort_by(|(_, food_a), (_, food_b)| {
            let a = food_a.get(name).unwrap_or(&FoodData::None).get_number().unwrap_or(&0.0);
            let b = food_b.get(name).unwrap_or(&FoodData::None).get_number().unwrap_or(&0.0);

            b.partial_cmp(a).unwrap()
        })
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

        for (_, food) in &self.food_list {
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
        let sum_food = self.get_sum();
        let sum = sum_food.get_list(name_list);
        for (name, food_data) in name_list.iter().zip(sum.iter()) {
            if *name == "食品名" {
                row.push(Cell::new(&color("合計", "y+")));
            } else {
                let food_data = food_data.unwrap_or(&FoodData::None);
                let mut cell = Cell::new(&color(&food_data.to_string(), "y+"));

                cell.align(prettytable::format::Alignment::RIGHT);
                row.push(cell);
            }
        }

        table.add_row(Row::new(row));
    }

    pub fn add_percentage_of_classes_to_table(&self, table: &mut Table, name_list: &[&str]) {
        // クラスごとの割合を追加する
        let sum_food = self.get_sum();
        let sum = sum_food.get_list(name_list);
        let class_list = self.split_by_class();

        for (class_name, class_food_table) in class_list {
            let sum_food = class_food_table.get_sum();
            let class_sum = sum_food.get_list(name_list);
            let mut row = Vec::new();

            for (name, (class_food_data, food_data)) in name_list.iter().zip(class_sum.iter().zip(sum.iter())) {
                if *name == "食品名" {
                    row.push(Cell::new(&color(&format!("{}の割合", class_name), "w+")));
                } else {
                    let data = match (class_food_data.unwrap().get_number(), food_data.unwrap().get_number()) {
                        (Some(class_num), Some(num)) => {
                            let per = (*class_num / *num) * 100.0;
                            format!("{:.0}%", per)
                        }
                        _ => "-".to_string()
                    };
                    let mut cell = Cell::new(&color(&data, "w+"));
                    //let mut cell = Cell::new(&data);
                    cell.align(prettytable::format::Alignment::RIGHT);
                    row.push(cell)
                }
            }
            table.add_row(Row::new(row));
        }
    }

    pub fn add_kijun_to_table(&self, table: &mut Table, name_list: &[&str], kijun: &Kijun) {
        // 摂取基準を追加する
        let mut row = Vec::new();
        let kijun_values = kijun.get_list(name_list);
        for (name, value) in name_list.iter().zip(kijun_values.iter()) {
            if *name == "食品名" {
                row.push(Cell::new(&color(&format!("摂取基準値（{}日分）", kijun.days), "c+")));
            } else {
                let data = match value {
                    Some(v) => v.to_string(),
                    None => "-".to_string()
                };
                let mut cell = Cell::new(&color(&data, "c+"));
                cell.align(prettytable::format::Alignment::RIGHT);
                row.push(cell);
            }
        }

        table.add_row(Row::new(row));
    }

    pub fn add_kijun_percentage_to_table(&self,
                                         table: &mut Table,
                                         name_list: &[&str],
                                         kijun: &Kijun) {
        // 摂取基準に対する割合を追加する
        let mut row = Vec::new();
        let sum_food = self.get_sum();
        let sum = sum_food.get_list(name_list);
        let kijun_values = kijun.get_list(name_list);
        let iter = name_list.iter().zip(kijun_values.iter()).zip(sum);
        for ((name, kijun_value), food_data) in iter {
            if *name == "食品名" {
                row.push(Cell::new(&color(&format!("摂取基準に対する割合（{}日分）", kijun.days), "g+")));
            } else {
                let data = match food_data.unwrap_or(&FoodData::None).get_number() {
                    Some(num) => match kijun_value {
                        Some(kijun_value) => {
                            let per = kijun_value.get_percentage(*num);
                            format!("{:.0}%", per)
                        },
                        None => "-".to_string()
                    },
                    _ => "-".to_string()
                };
                let mut cell = Cell::new(&color(&data, "g+"));
                cell.align(prettytable::format::Alignment::RIGHT);
                row.push(cell);
            }
        }

        table.add_row(Row::new(row));
    }

//    pub fn get_percentage_of_animal_protein(&self) -> usize {
//        let per = 0;
//        let mut food_table = FoodTable::new();
//        
//        for (_, food) in &self.food_list {
//            let group = match food.get("食品群").unwrap().get_number() {
//                Some(group) => group,
//                None => continue
//            };
//            
//            if 10.0 <= *group || *group <= 13.0 {
//                food_table.add(food.clone());
//            }
//        }
//
//        let sum = food_table.get_sum();
//        sum.get("たんぱく質").
//
//        2
//    }
//
//    pub fn get_status(&self) -> String {
//
//
//        "test get_status()".to_string()
//    }
    
    pub fn print(&self, name_list: &[&str]) {
        let table = self.get_table(name_list);

        table.printstd();
    }

    pub fn print_with_sum(&self, name_list: &[&str]) {
        let mut table = self.get_table(name_list);
        self.add_sum_to_table(&mut table, name_list);
        self.add_percentage_of_classes_to_table(&mut table, name_list);

        table.printstd();
    }

    pub fn print_with_sum_and_kijun(&self, name_list: &[&str], kijun: &Kijun) {
        let mut table = self.get_table(name_list);
        self.add_sum_to_table(&mut table, name_list);
        self.add_kijun_to_table(&mut table, name_list, &kijun);
        self.add_kijun_percentage_to_table(&mut table, name_list, &kijun);
        self.add_percentage_of_classes_to_table(&mut table, name_list);
        let percentage = self.percentage_of_kijun(&kijun).unwrap_or(-1.0);
        println!("{}", color(&format!("摂取基準の達成率（{}日分）: {:.2}%", kijun.days, percentage), "g+"));
        table.printstd();
    }

    pub fn percentage_of_kijun(&self, kijun: &Kijun) -> Option<f32> {
        let kijun_data_list = kijun.get_list(&KIJUN_KEY_LIST);
        let sum = self.get_sum();
        let sum_value_list = sum.get_list(&KIJUN_KEY_LIST);
        let mut sum_percentage = 0.0;

        for (kijun_data, sum_value) in kijun_data_list.iter().zip(sum_value_list.iter()) {
            if let &None = kijun_data { return None }
            let kijun_data = kijun_data.unwrap();

            if let &None = sum_value { return None }
            let sum_value = sum_value.unwrap();

            if let None = sum_value.get_number() { return None }
            let num = sum_value.get_number().unwrap();

            let mut percentage = kijun_data.get_percentage(*num);

            percentage = match kijun_data {
                // 範囲、以下が100%を超えた場合はマイナスにする
                KijunValue::Range(_) |
                KijunValue::Less(_) => {
                    if 100.0 < percentage {
                        100.0 - percentage
                    } else {
                        percentage
                    }
                },
                // 基準値の推奨値と以上と目安は100%を超えても無視する
                // そもそも基準値の以上は100%を超えることがない
                KijunValue::Suisyo(_) | KijunValue::Measu(_) |
                KijunValue::More(_) => {
                    if 100.0 < percentage {
                        100.0
                    } else {
                        percentage
                    }
                },
            };

            sum_percentage += percentage;
        }

        let percentage = sum_percentage / KIJUN_KEY_LIST.len() as f32;
        Some(percentage)
    }
}


#[cfg(test)]
mod test {
    use crate::food::Food;
    use crate::FoodTable;
    use crate::FoodData;
    use crate::kijun::{Kijun, Gender, PAL};


    #[test]
    fn test_food_table_new() {
        FoodTable::new();
    }

    #[test]
    fn test_food_table_add() {
        let mut food_table = FoodTable::new();
        let mut food = Food::new();
        food.set("食品番号", FoodData::from_str("0"));
        food.set("食品名", FoodData::from_str("麦ごはん"));
        food_table.add(food);

        let food = &food_table.food_list[0].1;
        assert_eq!(food.get("食品名"), Some(&FoodData::from_str("麦ごはん")));
        assert_eq!(food.get("食品群"), Some(&FoodData::None));
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
        assert_eq!(food.get("食品群").unwrap(), &FoodData::String("01".to_string()));
    }

    #[test]
    fn test_food_table_iter() {
        let food_table = FoodTable::from_json("./test/test_foods.json").unwrap();
        for _ in food_table.iter() {
            food_table.get_sum();
        }
    }

    #[test]
    fn test_food_table_get() {
        let food_table = FoodTable::from_json("./test/test_foods.json").unwrap();
        let food = food_table.get("01001").unwrap();
        assert_eq!(food.get("食品名").unwrap(), &FoodData::String("アマランサス　玄穀".to_string()))
    }

    #[test]
    fn test_food_table_get_list() {
        let food_table = FoodTable::from_json("./test/test_foods.json").unwrap();
        let food_table = food_table.get_list(&["01001", "01002", "18022"]);
        let keys: Vec<_> =  food_table.food_list.iter().map(|(key, _)| key).collect();
        assert!(keys.contains(&&"01001".to_string()));
        assert!(keys.contains(&&"01002".to_string()));
        assert!(keys.contains(&&"18022".to_string()));
    }

    #[test]
    fn test_food_table_get_sum() {
        let food_table = FoodTable::from_json("./test/test_foods.json").unwrap();
        let food_table = food_table.get_list(&["01001", "01002", "01003", "01004"]);
        let sum = food_table.get_sum();
        assert_eq!(sum.get("たんぱく質").unwrap().to_string(), "42.7");
        let sum = food_table.get_sum();
        assert_eq!(sum.get("食品名").unwrap().to_string(), "-");
    }

    #[test]
    fn test_food_table_set_weight() {
        let mut food_table = FoodTable::from_json("./test/test_foods.json").unwrap();
        assert_eq!(food_table.get("01001").unwrap().get("重量"), Some(&FoodData::Number(100.0)));

        food_table.set_weight(50.0);
        assert_eq!(food_table.get("01001").unwrap().get("重量"), Some(&FoodData::Number(50.0)))
    }

    #[test]
    fn test_food_table_split_by_class() {
        let mut food_table = FoodTable::new();

        let mut f1 = Food::new();
        f1.set("食品番号", FoodData::String("1".to_string()));
        food_table.add(f1.clone());
        let mut f2 = Food::new();
        f2.set("食品番号", FoodData::String("2".to_string()));
        f2.set("クラス", FoodData::String("test1".to_string()));
        food_table.add(f2.clone());
        let mut f3 = Food::new();
        f3.set("食品番号", FoodData::String("3".to_string()));
        f3.set("クラス", FoodData::String("test1".to_string()));
        food_table.add(f3.clone());
        let mut f4 = Food::new();
        f4.set("食品番号", FoodData::String("4".to_string()));
        f4.set("クラス", FoodData::String("test2".to_string()));
        food_table.add(f4.clone());
        let mut f5 = Food::new();
        f5.set("食品番号", FoodData::String("5".to_string()));
        f5.set("クラス", FoodData::String("test2".to_string()));
        food_table.add(f5.clone());
        
        let class_list = food_table.split_by_class();
        assert!(class_list.contains_key("test1"));
        assert!(class_list.contains_key("test2"));
        assert_eq!(class_list.get("test1").unwrap().food_list, vec![("2".to_string(), f2), ("3".to_string(), f3)]);
        assert_eq!(class_list.get("test2").unwrap().food_list, vec![("4".to_string(), f4), ("5".to_string(), f5)]);
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
        let keys: Vec<_> =  food_table.food_list.iter().map(|(key, _)| key).collect();
        assert!(keys.contains(&&"01002".to_string()));
        assert!(keys.contains(&&"01003".to_string()));
        assert!(keys.contains(&&"01005".to_string()));
        assert!(keys.contains(&&"01006".to_string()));
    }

    #[test]
    fn test_food_table_search_and() {
        let food_table = FoodTable::from_json("./test/test_foods.json").unwrap();
        let food_table = food_table.search_and(&["冷凍", "メンチ"]);
        let keys: Vec<_> =  food_table.food_list.iter().map(|(key, _)| key).collect();
        assert!(keys.contains(&&"18016".to_string()));
        assert!(keys.contains(&&"18022".to_string()));
    }

    #[test]
    fn test_food_table_ascending_order() {
        let mut food_table = FoodTable::from_json("./test/test_foods.json").unwrap();
        let mut iter = food_table.iter();
        assert_eq!(iter.next().unwrap().1.get("食品番号").unwrap(), &FoodData::String("01001".to_string()));

        food_table.sort_ascending_order("エネルギー");
        let mut iter = food_table.iter();
        assert_eq!(iter.next().unwrap().1.get("食品番号").unwrap(), &FoodData::String("18016".to_string()));
        assert_eq!(iter.next().unwrap().1.get("食品番号").unwrap(), &FoodData::String("01003".to_string()));
        assert_eq!(iter.next().unwrap().1.get("食品番号").unwrap(), &FoodData::String("18015".to_string()));

        food_table.sort_ascending_order("食品名");
        let mut iter = food_table.iter();
        assert_eq!(iter.next().unwrap().1.get("食品番号").unwrap(), &FoodData::String("18016".to_string()));
        assert_eq!(iter.next().unwrap().1.get("食品番号").unwrap(), &FoodData::String("01003".to_string()));
        assert_eq!(iter.next().unwrap().1.get("食品番号").unwrap(), &FoodData::String("18015".to_string()));
    }


    #[test]
    fn test_food_table_descending_order() {
        let mut food_table = FoodTable::from_json("./test/test_foods.json").unwrap();
        let mut iter = food_table.iter();
        assert_eq!(iter.next().unwrap().1.get("食品番号").unwrap(), &FoodData::String("01001".to_string()));
        
        food_table.sort_descending_order("エネルギー");
        let mut iter = food_table.iter();
        assert_eq!(iter.next().unwrap().1.get("食品番号").unwrap(), &FoodData::String("01004".to_string()));
        assert_eq!(iter.next().unwrap().1.get("食品番号").unwrap(), &FoodData::String("01002".to_string()));
        assert_eq!(iter.next().unwrap().1.get("食品番号").unwrap(), &FoodData::String("01001".to_string()));
        
        food_table.sort_ascending_order("食品名");
        let mut iter = food_table.iter();
        assert_eq!(iter.next().unwrap().1.get("食品番号").unwrap(), &FoodData::String("01004".to_string()));
        assert_eq!(iter.next().unwrap().1.get("食品番号").unwrap(), &FoodData::String("01002".to_string()));
        assert_eq!(iter.next().unwrap().1.get("食品番号").unwrap(), &FoodData::String("01001".to_string()));
    }
    
    #[test]
    fn test_food_table_percentage_of_kijun() {
        let food_table = FoodTable::new();
        let kijun = Kijun::new(20, 50.0, 160.0, Gender::Male, PAL::Low, 1);
        assert_eq!(food_table.percentage_of_kijun(&kijun), None);

        let food_table = FoodTable::from_json("./test/test_foods.json").unwrap();
        assert!(food_table.percentage_of_kijun(&kijun).is_some());
        
        //for key in KIJUN_KEY_LIST.iter() {
        //    println!("key {}, {:?}", key, kijun.get(key));
        //}
        
        // 摂取基準を完全に満たす食材を作成して、割合が100になればよい
        let mut food_table = FoodTable::new();
        let mut food = Food::new();
        food.set("エネルギー", FoodData::Number(1952.7593));
        food.set("たんぱく質", FoodData::Number(60.0));
        food.set("脂質", FoodData::Number(44.0));
        food.set("飽和脂肪酸", FoodData::Number(14.0));
        food.set("多価不飽和脂肪酸", FoodData::Number(13.0));
        food.set("炭水化物", FoodData::Number(245.0));
        food.set("食物繊維総量", FoodData::Number(20.0));
        food.set("レチノール活性当量", FoodData::Number(850.0));
        food.set("ビタミンD", FoodData::Number(5.5));
        food.set("α-トコフェロール", FoodData::Number(6.5));
        food.set("ビタミンK", FoodData::Number(150.0));
        food.set("ビタミンB1", FoodData::Number(1.4));
        food.set("ビタミンB2", FoodData::Number(1.6));
        food.set("ナイアシン", FoodData::Number(15.0));
        food.set("ビタミンB6", FoodData::Number(1.4));
        food.set("ビタミンB12", FoodData::Number(2.4));
        food.set("葉酸", FoodData::Number(240.0));
        food.set("パントテン酸", FoodData::Number(5.0));
        food.set("ビオチン", FoodData::Number(50.0));
        food.set("ビタミンC", FoodData::Number(100.0));
        food.set("ナトリウム", FoodData::Number(3148.0));
        food.set("カリウム", FoodData::Number(2500.0));
        food.set("カルシウム", FoodData::Number(800.0));
        food.set("マグネシウム", FoodData::Number(340.0));
        food.set("リン", FoodData::Number(1000.0));
        food.set("鉄", FoodData::Number(7.0));
        food.set("亜鉛", FoodData::Number(10.0));
        food.set("銅", FoodData::Number(0.9));
        food.set("マンガン", FoodData::Number(4.0));
        food.set("ヨウ素", FoodData::Number(130.0));
        food.set("セレン", FoodData::Number(30.0));
        food.set("クロム", FoodData::Number(10.0));
        food.set("モリブデン", FoodData::Number(25.0));
        food_table.add(food.clone());
        assert_eq!(food_table.percentage_of_kijun(&kijun), Some(100.0));
        
        let mut food_table = FoodTable::new();
        food.set("ナトリウム", FoodData::Number(4000.0));
        food_table.add(food.clone());
        // ナトリウムが過剰で他の栄養が完全な場合、割合が100以下になるはず
        assert!(food_table.percentage_of_kijun(&kijun).unwrap() < 100.0);
    }
}
