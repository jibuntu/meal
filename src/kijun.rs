use std::collections::HashMap;


macro_rules! pal_match {
    ($pal:ident, $low:expr, $moderate:expr, $high:expr) => {
        match $pal {
            PAL::Low => $low,
            PAL::Moderate => $moderate,
            PAL::High => $high
        }
    };
}


macro_rules! gender_match {
    ($gender:ident, $male:expr, $female:expr) => {
        match $gender {
            Gender::Female => $female,
            Gender::Male => $male
        }
    };
}

#[derive(Copy, Clone, Debug)]
pub enum KijunValue {
    Suisyo(f32), // 値になるべく近い方が望ましい
    Measu(f32), // 値になるべく近い方が望ましいが、あくまで目安である
    Less(f32), // この値より小さい方が望ましい
    More(f32), // この値より大きい方が望ましい
    Range((f32, f32)), // (mix, max) この値の範囲ないが望ましい
}

impl KijunValue {
    pub fn to_string(&self) -> String {
        match self {
            KijunValue::Suisyo(kijun_value) => {
                if 30.0 < *kijun_value {
                    format!("= {}", (kijun_value.round() * 100.0).floor() / 100.0)
                } else {
                    format!("= {}", (*kijun_value * 100.0).floor() / 100.0)
                }
            },
            KijunValue::Measu(kijun_value) => {
                if 30.0 < *kijun_value {
                    format!("? {}", (kijun_value.round() * 100.0).floor() / 100.0)
                } else {
                    format!("? {}", (*kijun_value * 100.0).floor() / 100.0)
                }
            },
            KijunValue::Less(kijun_value) => {
                if 30.0 < *kijun_value {
                    format!("- {}", (kijun_value.round() * 100.0).floor() / 100.0)
                } else {
                    format!("- {}", (*kijun_value * 100.0).floor() / 100.0)
                }
            },
            KijunValue::More(kijun_value) => {
                if 30.0 < *kijun_value {
                    format!("+ {}", (kijun_value.round() * 100.0).floor() / 100.0)
                } else {
                    format!("+ {}", (*kijun_value * 100.0).floor() / 100.0)
                }
            },
            KijunValue::Range((min, max)) => {
                let min_value = if 30.0 < *min { min.round() } else { *min };
                let max_value = if 30.0 < *max { max.round() } else { *max };
                format!("{} ~ {}", (min_value * 100.0).floor() / 100.0,
                       (max_value * 100.0).floor() / 100.0)

            },
        }
    }
    
    pub fn get_percentage(&self, value: f32) -> f32 {
        match self {
            KijunValue::Suisyo(kijun_value) => {
                (value / *kijun_value) * 100.0
            },
            KijunValue::Measu(kijun_value) => {
                (value / *kijun_value) * 100.0
            },
            KijunValue::Less(kijun_value) => {
                if value <= *kijun_value {
                    100.0
                } else {
                    (value / *kijun_value) * 100.0
                }
            },
            KijunValue::More(kijun_value) => {
                if *kijun_value <= value {
                    100.0
                } else {
                    (value / *kijun_value) * 100.0
                }
            },
            KijunValue::Range((min, max)) => {
                if value < *min {
                    (value / *min) * 100.0
                } else if *max < value {
                    (value / *max) * 100.0
                } else {
                    100.0
                }
            },
        }
    }

    pub fn change_days(&mut self, days: usize) {
        match self {
            KijunValue::Suisyo(kijun_value) => *kijun_value = *kijun_value * days as f32,
            KijunValue::Measu(kijun_value) => *kijun_value = *kijun_value * days as f32,
            KijunValue::Less(kijun_value) => *kijun_value = *kijun_value * days as f32,
            KijunValue::More(kijun_value) => *kijun_value = *kijun_value * days as f32,
            KijunValue::Range(kijun_value) => {
                *kijun_value = (kijun_value.0 * days as f32, kijun_value.1 * days as f32)
            },
        }
    }
}

#[test]
fn test_kijun_value_to_string() {
    assert_eq!(&KijunValue::Suisyo(20.0).to_string(), "= 20");
    assert_eq!(&KijunValue::Suisyo(20.5).to_string(), "= 20.5");
    assert_eq!(&KijunValue::Suisyo(30.5).to_string(), "= 31");
    assert_eq!(&KijunValue::Range((20.5, 30.5)).to_string(), "20.5 ~ 31");
}

#[test]
fn test_kijun_value_get_percentage() {
    assert_eq!(KijunValue::Suisyo(20.0).get_percentage(20.0), 100.0);
    assert_eq!(KijunValue::Suisyo(20.0).get_percentage(40.0), 200.0);
    assert_eq!(KijunValue::Suisyo(20.0).get_percentage(10.0), 50.0);
    assert_eq!(KijunValue::Less(20.0).get_percentage(10.0), 100.0);
    assert_eq!(KijunValue::More(20.0).get_percentage(40.0), 100.0);
    assert_eq!(KijunValue::Range((20.0, 40.0)).get_percentage(30.0), 100.0);
}

#[test]
fn test_kijun_value_change_days() {
    let mut kijun_value = KijunValue::Suisyo(20.0);
    kijun_value.change_days(10);
    assert_eq!(&kijun_value.to_string(), "= 200");
    kijun_value.change_days(0);
    assert_eq!(&kijun_value.to_string(), "= 0");
}


#[derive(Copy, Clone)]
pub enum PAL {
    Low,
    Moderate,
    High
}

#[derive(Copy, Clone)]
pub enum Gender {
    Female,
    Male
}

pub struct Kijun {
    pub age: usize,
    pub weight: f32,
    pub height: f32,
    pub gender: Gender,
    pub pal: PAL,
    pub days: usize,
    data_list: HashMap<String, KijunValue>
}

impl Kijun {
    pub fn new(age: usize,
           weight: f32,
           height: f32,
           gender: Gender,
           pal: PAL,
           days: usize) -> Kijun {
        let mut data_list: HashMap<String, KijunValue> = HashMap::new();

        if let Ok(energy_val) = Kijun::get_energy(weight, height, age, gender, pal) {
            data_list.insert("エネルギー".to_string(), energy_val);

            let energy_val = match energy_val {
                KijunValue::Measu(energy_val) => energy_val,
                _ => panic!("内部的なエラーです。\
                                 energyのKijunValueの値が間違っています")
            };

            if let Ok(lipid) = Kijun::get_lipid(age, energy_val) {
                data_list.insert("脂質".to_string(), lipid);
            }

            if let Ok(sfa) = Kijun::get_saturated_fatty_acid(age, energy_val) {
                data_list.insert("飽和脂肪酸".to_string(), sfa);
            }

            if let Ok(ca) = Kijun::get_carbohydrate(age, energy_val) {
                data_list.insert("炭水化物".to_string(), ca);
            }
        }

        if let Ok(protein) = Kijun::get_protein(age, weight) {
            data_list.insert("たんぱく質".to_string(), protein);
        }
        
        if let Ok(n6_fatty_acid) = Kijun::get_n6_fatty_acid(age, gender) {
            data_list.insert("n-6系脂肪酸".to_string(), n6_fatty_acid);
        }
        
        if let Ok(n3_fatty_acid) = Kijun::get_n3_fatty_acid(age, gender) {
            data_list.insert("n-3系脂肪酸".to_string(), n3_fatty_acid);
        }

        if let Ok(pufa) = Kijun::get_pufa(age, gender) {
            data_list.insert("多価不飽和脂肪酸".to_string(), pufa);
        }
        
        if let Ok(fiber) = Kijun::get_fiber(age, weight) {
            data_list.insert("食物繊維".to_string(), fiber);
            data_list.insert("食物繊維総量".to_string(), fiber);
        }

        if let Ok(vitamin_a) = Kijun::get_vitamin_a(age, weight) {
            data_list.insert("ビタミンA".to_string(), vitamin_a);
            data_list.insert("レチノール活性当量".to_string(), vitamin_a);
        }

        if let Ok(vitamin_d) = Kijun::get_vitamin_d(age, gender) {
            data_list.insert("ビタミンD".to_string(), vitamin_d);
        }

        if let Ok(vitamin_e) = Kijun::get_vitamin_e(age, gender) {
            data_list.insert("ビタミンE".to_string(), vitamin_e);
            data_list.insert("α-トコフェロール".to_string(), vitamin_e);
        }

        if let Ok(vitamin_k) = Kijun::get_vitamin_k(age, gender) {
            data_list.insert("ビタミンK".to_string(), vitamin_k);
        }

        if let Ok(vitamin_b1) = Kijun::get_vitamin_b1(age, gender) {
            data_list.insert("ビタミンB1".to_string(), vitamin_b1);
        }

        if let Ok(vitamin_b2) = Kijun::get_vitamin_b2(age, gender) {
            data_list.insert("ビタミンB2".to_string(), vitamin_b2);
        }

        if let Ok(niacin) = Kijun::get_niacin(age, gender) {
            data_list.insert("ナイアシン".to_string(), niacin);
        }

        if let Ok(vitamin_b6) = Kijun::get_vitamin_b6(age, gender) {
            data_list.insert("ビタミンB6".to_string(), vitamin_b6);
        }

        if let Ok(vitamin_b12) = Kijun::get_vitamin_b12(age, gender) {
            data_list.insert("ビタミンB12".to_string(), vitamin_b12);
        }

        if let Ok(folic_acid) = Kijun::get_folic_acid(age, gender) {
            data_list.insert("葉酸".to_string(), folic_acid);
        }

        if let Ok(pantothenic_acid) = Kijun::get_pantothenic_acid(age, gender) {
            data_list.insert("パントテン酸".to_string(), pantothenic_acid);
        }

        if let Ok(biotin) = Kijun::get_biotin(age, gender) {
            data_list.insert("ビオチン".to_string(), biotin);
        }

        if let Ok(vitamin_c) = Kijun::get_vitamin_c(age, gender) {
            data_list.insert("ビタミンC".to_string(), vitamin_c);
        }

        if let Ok(sodium) = Kijun::get_sodium(age, gender) {
            data_list.insert("ナトリウム".to_string(), sodium);
        }

        if let Ok(potassium) = Kijun::get_potassium(age, gender) {
            data_list.insert("カリウム".to_string(), potassium);
        }

        if let Ok(calcium) = Kijun::get_calcium(age, gender) {
            data_list.insert("カルシウム".to_string(), calcium);
        }

        if let Ok(magnesium) = Kijun::get_magnesium(age, gender) {
            data_list.insert("マグネシウム".to_string(), magnesium);
        }

        if let Ok(phosphorus) = Kijun::get_phosphorus(age, gender) {
            data_list.insert("リン".to_string(), phosphorus);
        }

        if let Ok(iron) = Kijun::get_iron(age, gender) {
            data_list.insert("鉄".to_string(), iron);
        }

        if let Ok(zinc) = Kijun::get_zinc(age, gender) {
            data_list.insert("亜鉛".to_string(), zinc);
        }

        if let Ok(copper) = Kijun::get_copper(age, gender) {
            data_list.insert("銅".to_string(), copper);
        }

        if let Ok(manganese) = Kijun::get_manganese(age, gender) {
            data_list.insert("マンガン".to_string(), manganese);
        }

        if let Ok(iodine) = Kijun::get_iodine(age, gender) {
            data_list.insert("ヨウ素".to_string(), iodine);
        }

        if let Ok(selenium) = Kijun::get_selenium(age, gender) {
            data_list.insert("セレン".to_string(), selenium);
        }
        
        if let Ok(chromium) = Kijun::get_chromium(age, gender) {
            data_list.insert("クロム".to_string(), chromium);
        }

        if let Ok(molybdenum) = Kijun::get_molybdenum(age, gender) {
            data_list.insert("モリブデン".to_string(), molybdenum);
        }

        // 日数を反映
        for (_, kijun_value) in &mut data_list {
            kijun_value.change_days(days);
        }

        Kijun {
            age,
            weight,
            height,
            gender,
            pal,
            days,
            data_list
        }
    }

    pub fn get(&self, key: &str) -> Option<&KijunValue> {
        self.data_list.get(key)
    }

    pub fn get_list(&self, keys: &[&str]) -> Vec<Option<&KijunValue>> {
        let mut data_list = Vec::new();
        for key in keys {
            data_list.push(self.get(key))
        }

        data_list
    }

    // 身体活動レベル
    pub fn get_pal(age: usize, pal: PAL) -> Result<f32, String> {
        let result = if age < 1 {
            return Err("年齢が１より小さい場合はPALの値を求めることはできません".to_string())
        } else if 1 <= age && age <= 2 {
            match pal {
                PAL::Moderate => 1.35,
                _ => return Err("年齢が１〜５の場合はmoderate以外のPALの値を求めることはできません".to_string())
            }
        } else if 3 <= age && age <= 5 {
            match pal {
                PAL::Moderate => 1.45,
                _ => return Err("年齢が１〜５の場合はmoderate以外のPALの値を求めることはできません".to_string())
            }
        } else if 6 <= age && age <= 7 {
            pal_match!(pal, 1.35, 1.55, 1.75)
        } else if 8 <= age && age <= 9 {
            pal_match!(pal, 1.40, 1.60, 1.80)
        } else if 10 <= age && age <= 11 {
            pal_match!(pal, 1.45, 1.65, 1.85)
        } else if 12 <= age && age <= 14 {
            pal_match!(pal, 1.50, 1.70, 1.90)
        } else if 15 <= age && age <= 17 {
            pal_match!(pal, 1.55, 1.75, 1.95)
        } else if 18 <= age && age <= 29 {
            pal_match!(pal, 1.50, 1.75, 2.00)
        } else if 30 <= age && age <= 49 {
            pal_match!(pal, 1.50, 1.75, 2.00)
        } else if 50 <= age && age <= 69 {
            pal_match!(pal, 1.50, 1.75, 2.00)
        } else if 70 <= age {
            pal_match!(pal, 1.45, 1.70, 1.95)
        } else {
            return Err("PALを求めることができません".to_string())
        };

        return Ok(result)
    }

    // 基礎代謝量
    pub fn get_base_metabolism(weight: f32,
                           height: f32,
                           age: usize,
                           gender: Gender) -> f32 {
        gender_match!(gender,
            (0.0481 * weight + 0.0234 * height - 0.0138 * age as f32 - 0.4235)
                    * 1000.0 / 4.186,
            (0.0481 * weight + 0.0234 * height - 0.0138 * age as f32 - 0.9708)
                * 1000.0 / 4.186)
    }

    // エネルギー必要量（kcal）
    pub fn get_energy(weight: f32, 
                  height: f32, 
                  age: usize, 
                  gender: Gender, 
                  pal: PAL) -> Result<KijunValue, String> {
        let pal_value = match Kijun::get_pal(age, pal) {
            Ok(pal_value) => pal_value,
            Err(e) => return Err(e)
        };
        
        let bm = Kijun::get_base_metabolism(weight, height, age, gender);
        
        if age <= 17 {
            return Err("17歳以下はエネルギー必要量を求めることができません".to_string())
        }

        Ok(KijunValue::Measu(bm * pal_value))

    }

    // たんぱく質
    pub fn get_protein(age: usize, weight: f32) -> Result<KijunValue, String>{
        let result = match age {
            0 ... 17 => {
                return Err("17歳以下はたんぱく質の推奨量を求めることができません".to_string())
            },
            18 ... 69 => {
                (0.72 * weight) * 1.25
            },
            age if 70 <= age => {
                return Err("70歳以上はたんぱく質の推奨量を求めることができません".to_string())
            },
            _ => {
                return Err("たんぱく質の推奨量を求めることができません".to_string())
            }
        };

        Ok(KijunValue::More(result))
    }

    // 脂質
    pub fn get_lipid(age: usize, energy: f32) -> Result<KijunValue, String> {
        if age <= 0 {
            return Err("0歳以下は脂質の目標量を求めることができません".to_string())
        }

        // エネルギーの割合を計算して9で割ってグラムに変換する
        let result_min = (energy * 0.20) / 9.0;
        let result_max = (energy * 0.30) / 9.0;

        Ok(KijunValue::Range((result_min, result_max)))
    }

    // 飽和脂肪酸
    pub fn get_saturated_fatty_acid(age: usize, energy: f32) -> Result<KijunValue, String> {
        if age <= 17 {
            return Err("17歳以下は飽和脂肪酸の目標量を求めることができません".to_string())
        }

        // エネルギーの割合を計算して9で割ってグラムに変換する
        Ok(KijunValue::Less((energy * 0.07) / 9.0))
    }

    // n-6系脂肪酸
    pub fn get_n6_fatty_acid(age: usize, gender: Gender) -> Result<KijunValue, String> {
        let result = match age {
            0 => {
                return Err("0歳以下はn-6系脂肪酸の目安量を求めることができません".to_string())
            },
            1 ... 2 => {   gender_match!(gender, 5.0,  5.0) },
            3 ... 5 => {   gender_match!(gender, 7.0,  6.0) },
            6 ... 7 => {   gender_match!(gender, 7.0,  7.0) },
            8 ... 9 => {   gender_match!(gender, 9.0,  7.0) },
            10 ... 11 => { gender_match!(gender, 9.0,  8.0) },
            12 ... 14 => { gender_match!(gender, 12.0, 10.0) },
            15 ... 17 => { gender_match!(gender, 13.0, 10.0) },
            18 ... 29 => { gender_match!(gender, 11.0, 8.0) },
            30 ... 49 => { gender_match!(gender, 10.0, 8.0) },
            50 ... 69 => { gender_match!(gender, 10.0, 8.0) },
            age if 70 <= age => { gender_match!(gender, 8.0, 7.0) },
            _ => {
                return Err("n-6系脂肪酸の目安量を求めることができません".to_string())
            }
        };

        Ok(KijunValue::Measu(result))
    }

    // n-3系脂肪酸
    pub fn get_n3_fatty_acid(age: usize, gender: Gender) -> Result<KijunValue, String> {
        let result = match age {
            0 => {
                return Err("0歳以下はn-3系脂肪酸の目安量を求めることができません".to_string())
            },
            1 ... 2 => {   gender_match!(gender, 0.7, 0.8) },
            3 ... 5 => {   gender_match!(gender, 1.3, 1.1) },
            6 ... 7 => {   gender_match!(gender, 1.4, 1.3) },
            8 ... 9 => {   gender_match!(gender, 1.7, 1.4) },
            10 ... 11 => { gender_match!(gender, 1.7, 1.5) },
            12 ... 14 => { gender_match!(gender, 2.1, 1.8) },
            15 ... 17 => { gender_match!(gender, 2.3, 1.7) },
            18 ... 29 => { gender_match!(gender, 2.0, 1.6) },
            30 ... 49 => { gender_match!(gender, 2.1, 1.6) },
            50 ... 69 => { gender_match!(gender, 2.4, 2.0) },
            age if 70 <= age => { gender_match!(gender, 2.2, 1.9) },
            _ => {
                return Err("n-3系脂肪酸の目安量を求めることができません".to_string())
            }
        };

        Ok(KijunValue::Measu(result))
    }

    // 多価不飽和脂肪酸
    pub fn get_pufa(age: usize, gender: Gender) -> Result<KijunValue, String> { 
        let n6_fatty_acid = match Kijun::get_n6_fatty_acid(age, gender) {
            Ok(kijun_value) => match kijun_value {
                KijunValue::Measu(n6_fatty_acid) => n6_fatty_acid,
                _ => panic!("内部的なエラー。\
                             kijun_valueがMeasuではありません")
            } 
            Err(e) => return Err(e)
        };

        let n3_fatty_acid = match Kijun::get_n3_fatty_acid(age, gender) {
            Ok(kijun_value) => match kijun_value {
                KijunValue::Measu(n3_fatty_acid) => n3_fatty_acid,
                _ => panic!("内部的なエラー。\
                             kijun_valueがMeasuではありません")
            }
            Err(e) => return Err(e)
        };

        Ok(KijunValue::Measu(n6_fatty_acid + n3_fatty_acid))
    }

    // 炭水化物
    pub fn get_carbohydrate(age: usize, energy: f32) -> Result<KijunValue, String> {
        if age <= 0 {
            return Err("0歳以下は炭水化物の目標量を求めることができません".to_string())
        }
        let max = (energy * 0.50) / 4.0;
        let min = (energy * 0.65) / 4.0;

        Ok(KijunValue::Range((max, min)))
    }

    // 食物繊維
    pub fn get_fiber(age: usize, weight: f32) -> Result<KijunValue, String> {
        let result = match age {
            0 ... 17 => {
                return Err("17歳以下は食物繊維の目標量を求めることができません".to_string())
            },
            18 ... 69 => {
                18.9 * (weight / 57.8).powf(0.75)
            },
            age if 70 <= age => {
                return Err("70歳以上は食物繊維の目標量を求めることができません".to_string())
            },
            _ => {
                return Err("食物繊維の目標量を求めることができません".to_string())
            }
        };

        Ok(KijunValue::More(result))
    }

    // ビタミンA
    pub fn get_vitamin_a(age: usize, weight: f32) -> Result<KijunValue, String> {
        let result = match age {
            0 ... 17 => {
                return Err("17歳以下はビタミンAの推奨量を求めることができません".to_string())
            },
            18 ... 69 => {
                // https://www.mhlw.go.jp/file/05-Shingikai-10901000-Kenkoukyoku-Soumuka/0000114399.pdf - 166 page
                (9.3 * weight) * 1.4
            },
            age if 70 <= age => {
                return Err("70歳以上はビタミンAの推奨量を求めることができません".to_string())
            },
            _ => {
                return Err("ビタミンAの推奨量を求めることができません".to_string())
            }
        };

        Ok(KijunValue::Suisyo(result))
    }

    // ビタミンD
    pub fn get_vitamin_d(age: usize, gender: Gender) -> Result<KijunValue, String> {
        let result = match age {
            0 => {
                return Err("0歳以下はビタミンDの推奨量を求めることができません".to_string())
            },
            1 ... 2 => gender_match!(gender, 2.0, 2.0),
            3 ... 5 => gender_match!(gender, 2.5, 2.5),
            6 ... 7 => gender_match!(gender, 3.0, 3.0),
            8 ... 9 => gender_match!(gender, 3.5, 3.5),
            10 ... 11 => gender_match!(gender, 4.5, 4.5),
            12 ... 14 => gender_match!(gender, 5.5, 5.5),
            15 ... 17 => gender_match!(gender, 6.0, 6.0),
            18 ... 29 => gender_match!(gender, 5.5, 5.5),
            30 ... 49 => gender_match!(gender, 5.5, 5.5),
            50 ... 69 => gender_match!(gender, 5.5, 5.5),
            age if 70 <= age => gender_match!(gender, 5.5, 5.5),
            _ => {
               return Err("ビタミンDの推奨量を求めることができません".to_string())
            }
        };

        Ok(KijunValue::Measu(result))
    }

    // ビタミンE
    pub fn get_vitamin_e(age: usize, gender: Gender) -> Result<KijunValue, String> {
        let result = match age {
            0 => {
                return Err("0歳以下はビタミンEの目安量を求めることができません".to_string())
            },
            1 ... 2 => {   gender_match!(gender, 3.5, 3.5) },
            3 ... 5 => {   gender_match!(gender, 4.5, 4.5) },
            6 ... 7 => {   gender_match!(gender, 5.0, 5.0) },
            8 ... 9 => {   gender_match!(gender, 5.5, 5.5) },
            10 ... 11 => { gender_match!(gender, 5.5, 5.5) },
            12 ... 14 => { gender_match!(gender, 7.5, 6.0) },
            15 ... 17 => { gender_match!(gender, 7.5, 6.0) },
            18 ... 29 => { gender_match!(gender, 6.5, 6.0) },
            30 ... 49 => { gender_match!(gender, 6.5, 6.0) },
            50 ... 69 => { gender_match!(gender, 6.5, 6.0) },
            age if 70 <= age => { gender_match!(gender, 6.5, 6.0) },
            _ => {
                return Err("ビタミンEの目安量を求めることができません".to_string())
            }
        };

        Ok(KijunValue::Measu(result))
    }

    // ビタミンK
    pub fn get_vitamin_k(age: usize, gender: Gender) -> Result<KijunValue, String> {
        let result = match age {
            0 => {
                return Err("0歳以下はビタミンKの目安量を求めることができません".to_string())
            },
            1 ... 2 => gender_match!(gender, 60.0, 60.0),
            3 ... 5 => gender_match!(gender, 70.0, 70.0),
            6 ... 7 => gender_match!(gender, 85.0, 85.0),
            8 ... 9 => gender_match!(gender, 100.0, 100.0),
            10 ... 11 => gender_match!(gender, 120.0, 120.0),
            12 ... 14 => gender_match!(gender, 150.0, 150.0),
            15 ... 17 => gender_match!(gender, 160.0, 160.0),
            18 ... 29 => gender_match!(gender, 150.0, 150.0),
            30 ... 49 => gender_match!(gender, 150.0, 150.0),
            50 ... 69 => gender_match!(gender, 150.0, 150.0),
            age if 70 <= age => gender_match!(gender, 150.0, 150.0),
            _ => {
                return Err("ビタミンKの目安量を求めることができません".to_string())
            }
        };

        Ok(KijunValue::Measu(result))
    }

    // ビタミンB1
    pub fn get_vitamin_b1(age: usize, gender: Gender) -> Result<KijunValue, String> {
        let result = match age {
            0 => {
                return Err("0歳以下はビタミンB1の推奨量を求めることができません".to_string())
            },
            1 ... 2 => {   gender_match!(gender, 0.5, 0.5) },
            3 ... 5 => {   gender_match!(gender, 0.7, 0.7) },
            6 ... 7 => {   gender_match!(gender, 0.8, 0.8) },
            8 ... 9 => {   gender_match!(gender, 1.0, 0.9) },
            10 ... 11 => { gender_match!(gender, 1.2, 1.1) },
            12 ... 14 => { gender_match!(gender, 1.4, 1.3) },
            15 ... 17 => { gender_match!(gender, 1.5, 1.2) },
            18 ... 29 => { gender_match!(gender, 1.4, 1.1) },
            30 ... 49 => { gender_match!(gender, 1.4, 1.1) },
            50 ... 69 => { gender_match!(gender, 1.3, 1.0) },
            age if 70 <= age => { gender_match!(gender, 1.2, 0.9) },
            _ => {
                return Err("ビタミンB1の推奨量を求めることができません".to_string())
            }
        };

        Ok(KijunValue::Suisyo(result))
    }

    // ビタミンB2
    pub fn get_vitamin_b2(age: usize, gender: Gender) -> Result<KijunValue, String> {
        let result = match age {
            0 => {
                return Err("0歳以下はビタミンB2の推奨量を求めることができません".to_string())
            },
            1 ... 2 => {   gender_match!(gender, 0.6, 0.5) },
            3 ... 5 => {   gender_match!(gender, 0.8, 0.8) },
            6 ... 7 => {   gender_match!(gender, 0.9, 0.9) },
            8 ... 9 => {   gender_match!(gender, 1.1, 0.0) },
            10 ... 11 => { gender_match!(gender, 1.4, 1.3) },
            12 ... 14 => { gender_match!(gender, 1.6, 1.4) },
            15 ... 17 => { gender_match!(gender, 1.7, 1.4) },
            18 ... 29 => { gender_match!(gender, 1.6, 1.2) },
            30 ... 49 => { gender_match!(gender, 1.6, 1.2) },
            50 ... 69 => { gender_match!(gender, 1.5, 1.1) },
            age if 70 <= age => { gender_match!(gender, 1.3, 1.1) },
            _ => {
                return Err("ビタミンB2の推奨量を求めることができません".to_string())
            }
        };

        Ok(KijunValue::Suisyo(result))
    }

    // ナイアシン
    pub fn get_niacin(age: usize, gender: Gender) -> Result<KijunValue, String> {
        let result = match age {
            0 => {
                return Err("0歳以下はナイアシンの推奨量を求めることができません".to_string())
            },
            1 ... 2 => {   gender_match!(gender,  5.0,  5.0) },
            3 ... 5 => {   gender_match!(gender,  7.0,  7.0) },
            6 ... 7 => {   gender_match!(gender,  9.0,  8.0) },
            8 ... 9 => {   gender_match!(gender, 11.0, 10.0) },
            10 ... 11 => { gender_match!(gender, 13.0, 12.0) },
            12 ... 14 => { gender_match!(gender, 15.0, 14.0) },
            15 ... 17 => { gender_match!(gender, 16.0, 13.0) },
            18 ... 29 => { gender_match!(gender, 15.0, 11.0) },
            30 ... 49 => { gender_match!(gender, 15.0, 12.0) },
            50 ... 69 => { gender_match!(gender, 14.0, 11.0) },
            age if 70 <= age => { gender_match!(gender, 13.0, 10.0) },
            _ => {
                return Err("ナイアシンの推奨量を求めることができません".to_string())
            }
        };

        Ok(KijunValue::Suisyo(result))
    }

    // ビタミンB6
    pub fn get_vitamin_b6(age: usize, gender: Gender) -> Result<KijunValue, String> {
        let result = match age {
            0 => {
                return Err("0歳以下はビタミンB6の推奨量を求めることができません".to_string())
            },
            1 ... 2 => {   gender_match!(gender, 0.5, 0.5) },
            3 ... 5 => {   gender_match!(gender, 0.6, 0.6) },
            6 ... 7 => {   gender_match!(gender, 0.8, 0.7) },
            8 ... 9 => {   gender_match!(gender, 0.9, 0.9) },
            10 ... 11 => { gender_match!(gender, 1.2, 1.2) },
            12 ... 14 => { gender_match!(gender, 1.4, 1.3) },
            15 ... 17 => { gender_match!(gender, 1.5, 1.3) },
            18 ... 29 => { gender_match!(gender, 1.4, 1.2) },
            30 ... 49 => { gender_match!(gender, 1.4, 1.2) },
            50 ... 69 => { gender_match!(gender, 1.4, 1.2) },
            age if 70 <= age => { gender_match!(gender, 1.4, 1.2) },
            _ => {
                return Err("ビタミンB6の推奨量を求めることができません".to_string())
            }
        };

        Ok(KijunValue::Suisyo(result))
    }

    // ビタミンB12
    pub fn get_vitamin_b12(age: usize, gender: Gender) -> Result<KijunValue, String> {
        let result = match age {
            0 => {
                return Err("0歳以下はビタミンB12の推奨量を求めることができません".to_string())
            },
            1 ... 2 => {   gender_match!(gender, 0.9, 0.9) },
            3 ... 5 => {   gender_match!(gender, 1.0, 1.0) },
            6 ... 7 => {   gender_match!(gender, 1.3, 1.3) },
            8 ... 9 => {   gender_match!(gender, 1.5, 1.5) },
            10 ... 11 => { gender_match!(gender, 1.8, 1.8) },
            12 ... 14 => { gender_match!(gender, 2.3, 2.3) },
            15 ... 17 => { gender_match!(gender, 2.5, 2.5) },
            18 ... 29 => { gender_match!(gender, 2.4, 2.4) },
            30 ... 49 => { gender_match!(gender, 2.4, 2.4) },
            50 ... 69 => { gender_match!(gender, 2.4, 2.4) },
            age if 70 <= age => { gender_match!(gender, 2.4, 2.4) },
            _ => {
                return Err("ビタミンB12の推奨量を求めることができません".to_string())
            }
        };

        Ok(KijunValue::Suisyo(result))
    }
    
    // 葉酸
    pub fn get_folic_acid(age: usize, gender: Gender) -> Result<KijunValue, String> {
        let result = match age {
            0 => {
                return Err("0歳以下は葉酸の推奨量を求めることができません".to_string())
            },
            1 ... 2 => {   gender_match!(gender,  90.0,  90.0) },
            3 ... 5 => {   gender_match!(gender, 100.0, 100.0) },
            6 ... 7 => {   gender_match!(gender, 130.0, 130.0) },
            8 ... 9 => {   gender_match!(gender, 150.0, 150.0) },
            10 ... 11 => { gender_match!(gender, 180.0, 180.0) },
            12 ... 14 => { gender_match!(gender, 230.0, 230.0) },
            15 ... 17 => { gender_match!(gender, 250.0, 250.0) },
            18 ... 29 => { gender_match!(gender, 240.0, 240.0) },
            30 ... 49 => { gender_match!(gender, 240.0, 240.0) },
            50 ... 69 => { gender_match!(gender, 240.0, 240.0) },
            age if 70 <= age => { gender_match!(gender, 240.0, 240.0) },
            _ => {
                return Err("葉酸の推奨量を求めることができません".to_string())
            }
        };

        Ok(KijunValue::Suisyo(result))
    }

    // パントテン酸
    pub fn get_pantothenic_acid(age: usize, gender: Gender) -> Result<KijunValue, String> {
        let result = match age {
            0 => {
                return Err("0歳以下はパントテン酸の目安量を求めることができません".to_string())
            },
            1 ... 2 => {   gender_match!(gender, 3.0, 3.0) },
            3 ... 5 => {   gender_match!(gender, 4.0, 4.0) },
            6 ... 7 => {   gender_match!(gender, 5.0, 5.0) },
            8 ... 9 => {   gender_match!(gender, 5.0, 5.0) },
            10 ... 11 => { gender_match!(gender, 6.0, 6.0) },
            12 ... 14 => { gender_match!(gender, 7.0, 6.0) },
            15 ... 17 => { gender_match!(gender, 7.0, 5.0) },
            18 ... 29 => { gender_match!(gender, 5.0, 4.0) },
            30 ... 49 => { gender_match!(gender, 5.0, 4.0) },
            50 ... 69 => { gender_match!(gender, 5.0, 5.0) },
            age if 70 <= age => { gender_match!(gender, 5.0, 5.0) },
            _ => {
                return Err("パントテン酸の目安量を求めることができません".to_string())
            }
        };

        Ok(KijunValue::Suisyo(result))
    }

    // ビオチン
    pub fn get_biotin(age: usize, gender: Gender) -> Result<KijunValue, String> {
        let result = match age {
            0 => {
                return Err("0歳以下はビオチンの目安量を求めることができません".to_string())
            },
            1 ... 2 => {   gender_match!(gender, 20.0, 20.0) },
            3 ... 5 => {   gender_match!(gender, 20.0, 20.0) },
            6 ... 7 => {   gender_match!(gender, 25.0, 25.0) },
            8 ... 9 => {   gender_match!(gender, 30.0, 30.0) },
            10 ... 11 => { gender_match!(gender, 35.0, 35.0) },
            12 ... 14 => { gender_match!(gender, 50.0, 50.0) },
            15 ... 17 => { gender_match!(gender, 50.0, 50.0) },
            18 ... 29 => { gender_match!(gender, 50.0, 50.0) },
            30 ... 49 => { gender_match!(gender, 50.0, 50.0) },
            50 ... 69 => { gender_match!(gender, 50.0, 50.0) },
            age if 70 <= age => { gender_match!(gender, 50.0, 50.0) },
            _ => {
                return Err("ビオチンの目安量を求めることができません".to_string())
            }
        };

        Ok(KijunValue::Measu(result))
    }

    // ビタミンC
    pub fn get_vitamin_c(age: usize, gender: Gender) -> Result<KijunValue, String> {
        let result = match age {
            0 => {
                return Err("0歳以下はビタミンCの推奨量を求めることができません".to_string())
            },
            1 ... 2 => {   gender_match!(gender, 35.0, 35.0) },
            3 ... 5 => {   gender_match!(gender, 40.0, 40.0) },
            6 ... 7 => {   gender_match!(gender, 55.0, 55.0) },
            8 ... 9 => {   gender_match!(gender, 60.0, 60.0) },
            10 ... 11 => { gender_match!(gender, 75.0, 75.0) },
            12 ... 14 => { gender_match!(gender, 95.0, 95.0) },
            15 ... 17 => { gender_match!(gender, 100.0, 100.0) },
            18 ... 29 => { gender_match!(gender, 100.0, 100.0) },
            30 ... 49 => { gender_match!(gender, 100.0, 100.0) },
            50 ... 69 => { gender_match!(gender, 100.0, 100.0) },
            age if 70 <= age => { gender_match!(gender, 100.0, 100.0) },
            _ => {
                return Err("ビタミンCの推奨量を求めることができません".to_string())
            }
        };

        Ok(KijunValue::Suisyo(result))
    }

    // ナトリウム
    pub fn get_sodium(age: usize, gender: Gender) -> Result<KijunValue, String> {
        let result = match age {
            0 => {
                return Err("0歳以下はナトリウムの目標量を求めることができません".to_string())
            },
            1 ... 2 => {   gender_match!(gender, 3.0, 3.5) },
            3 ... 5 => {   gender_match!(gender, 4.0, 4.5) },
            6 ... 7 => {   gender_match!(gender, 5.0, 5.5) },
            8 ... 9 => {   gender_match!(gender, 5.5, 6.0) },
            10 ... 11 => { gender_match!(gender, 6.5, 7.0) },
            12 ... 14 => { gender_match!(gender, 8.0, 7.0) },
            15 ... 17 => { gender_match!(gender, 8.0, 7.0) },
            18 ... 29 => { gender_match!(gender, 8.0, 7.0) },
            30 ... 49 => { gender_match!(gender, 8.0, 7.0) },
            50 ... 69 => { gender_match!(gender, 8.0, 7.0) },
            age if 70 <= age => { gender_match!(gender, 8.0, 7.0) },
            _ => {
                return Err("ナトリウムの目標量を求めることができません".to_string())
            }
        };

        Ok(KijunValue::Less((result / 2.54) * 1000.0))
    }

    // カリウム
    pub fn get_potassium(age: usize, gender: Gender) -> Result<KijunValue, String> {
        let result = match age {
            0 => {
                return Err("0歳以下はカリウムの目安量を求めることができません".to_string())
            },
            1 ... 2 => {   gender_match!(gender,  900.0,  800.0) },
            3 ... 5 => {   gender_match!(gender, 1100.0, 1000.0) },
            6 ... 7 => {   gender_match!(gender, 1300.0, 1200.0) },
            8 ... 9 => {   gender_match!(gender, 1600.0, 1500.0) },
            10 ... 11 => { gender_match!(gender, 1900.0, 1800.0) },
            12 ... 14 => { gender_match!(gender, 2400.0, 2200.0) },
            15 ... 17 => { gender_match!(gender, 2800.0, 2100.0) },
            18 ... 29 => { gender_match!(gender, 2500.0, 2000.0) },
            30 ... 49 => { gender_match!(gender, 2500.0, 2000.0) },
            50 ... 69 => { gender_match!(gender, 2500.0, 2000.0) },
            age if 70 <= age => { gender_match!(gender, 2500.0, 2000.0) },
            _ => {
                return Err("カリウムの目安量を求めることができません".to_string())
            }
        };

        Ok(KijunValue::More(result))
    }

    // カルシウム
    pub fn get_calcium(age: usize, gender: Gender) -> Result<KijunValue, String> {
        let result = match age {
            0 => {
                return Err("0歳以下はカルシウムの推奨量を求めることができません".to_string())
            },
            1 ... 2 => {   gender_match!(gender,  450.0, 400.0) },
            3 ... 5 => {   gender_match!(gender,  600.0, 550.0) },
            6 ... 7 => {   gender_match!(gender,  600.0, 550.0) },
            8 ... 9 => {   gender_match!(gender,  650.0, 750.0) },
            10 ... 11 => { gender_match!(gender,  700.0, 750.0) },
            12 ... 14 => { gender_match!(gender, 1000.0, 800.0) },
            15 ... 17 => { gender_match!(gender,  800.0, 650.0) },
            18 ... 29 => { gender_match!(gender,  800.0, 650.0) },
            30 ... 49 => { gender_match!(gender,  650.0, 650.0) },
            50 ... 69 => { gender_match!(gender,  700.0, 650.0) },
            age if 70 <= age => { gender_match!(gender, 700.0, 650.0) },
            _ => {
                return Err("カルシウムの推奨量を求めることができません".to_string())
            }
        };

        Ok(KijunValue::More(result))
    }

    // マグネシウム
    pub fn get_magnesium(age: usize, gender: Gender) -> Result<KijunValue, String> {
        let result = match age {
            0 => {
                return Err("0歳以下はマグネシウムの推奨量を求めることができません".to_string())
            },
            1 ... 2 => {   gender_match!(gender,  70.0,  70.0) },
            3 ... 5 => {   gender_match!(gender, 100.0, 100.0) },
            6 ... 7 => {   gender_match!(gender, 130.0, 130.0) },
            8 ... 9 => {   gender_match!(gender, 170.0, 160.0) },
            10 ... 11 => { gender_match!(gender, 210.0, 220.0) },
            12 ... 14 => { gender_match!(gender, 290.0, 290.0) },
            15 ... 17 => { gender_match!(gender, 360.0, 310.0) },
            18 ... 29 => { gender_match!(gender, 340.0, 270.0) },
            30 ... 49 => { gender_match!(gender, 370.0, 290.0) },
            50 ... 69 => { gender_match!(gender, 350.0, 290.0) },
            age if 70 <= age => { gender_match!(gender, 320.0, 270.0) },
            _ => {
                return Err("マグネシウムの推奨量を求めることができません".to_string())
            }
        };

        Ok(KijunValue::Suisyo(result))
    }

    // リン
    pub fn get_phosphorus(age: usize, gender: Gender) -> Result<KijunValue, String> {
        let result = match age {
            0 => {
                return Err("0歳以下はリンの目安量を求めることができません".to_string())
            },
            1 ... 2 => {   gender_match!(gender,  500.0,  500.0) },
            3 ... 5 => {   gender_match!(gender,  800.0,  600.0) },
            6 ... 7 => {   gender_match!(gender,  900.0,  900.0) },
            8 ... 9 => {   gender_match!(gender, 1000.0,  900.0) },
            10 ... 11 => { gender_match!(gender, 1100.0, 1000.0) },
            12 ... 14 => { gender_match!(gender, 1200.0, 1100.0) },
            15 ... 17 => { gender_match!(gender, 1200.0,  900.0) },
            18 ... 29 => { gender_match!(gender, 1000.0,  800.0) },
            30 ... 49 => { gender_match!(gender, 1000.0,  800.0) },
            50 ... 69 => { gender_match!(gender, 1000.0,  800.0) },
            age if 70 <= age => { gender_match!(gender, 1000.0, 800.0) },
            _ => {
                return Err("リンの目安量を求めることができません".to_string())
            }
        };

        Ok(KijunValue::Measu(result))
    }

    // 鉄
    pub fn get_iron(age: usize, gender: Gender) -> Result<KijunValue, String> {
        let result = match age {
            0 => {
                return Err("0歳以下は鉄の推奨量を求めることができません".to_string())
            },
            1 ... 2 => {   gender_match!(gender,  4.5,  4.5) },
            3 ... 5 => {   gender_match!(gender,  5.5,  5.0) },
            6 ... 7 => {   gender_match!(gender,  6.5,  6.5) },
            8 ... 9 => {   gender_match!(gender,  8.0,  8.5) },
            10 ... 11 => { gender_match!(gender, 10.0, 10.0) },
            12 ... 14 => { gender_match!(gender, 11.5, 10.0) },
            15 ... 17 => { gender_match!(gender,  9.5,  7.0) },
            18 ... 29 => { gender_match!(gender,  7.0,  6.0) },
            30 ... 49 => { gender_match!(gender,  7.5,  6.5) },
            50 ... 69 => { gender_match!(gender,  7.5,  6.5) },
            age if 70 <= age => { gender_match!(gender, 7.0, 6.0) },
            _ => {
                return Err("鉄の推奨量を求めることができません".to_string())
            }
        };

        Ok(KijunValue::Suisyo(result))
    }

    // 亜鉛
    pub fn get_zinc(age: usize, gender: Gender) -> Result<KijunValue, String> {
        let result = match age {
            0 => {
                return Err("0歳以下は亜鉛の推奨量を求めることができません".to_string())
            },
            1 ... 2 => {   gender_match!(gender,  3.0, 3.0) },
            3 ... 5 => {   gender_match!(gender,  4.0, 4.0) },
            6 ... 7 => {   gender_match!(gender,  5.0, 5.0) },
            8 ... 9 => {   gender_match!(gender,  6.0, 5.0) },
            10 ... 11 => { gender_match!(gender,  7.0, 7.0) },
            12 ... 14 => { gender_match!(gender,  9.0, 8.0) },
            15 ... 17 => { gender_match!(gender, 10.0, 8.0) },
            18 ... 29 => { gender_match!(gender, 10.0, 8.0) },
            30 ... 49 => { gender_match!(gender, 10.0, 8.0) },
            50 ... 69 => { gender_match!(gender, 10.0, 8.0) },
            age if 70 <= age => { gender_match!(gender, 9.0, 7.0) },
            _ => {
                return Err("亜鉛の推奨量を求めることができません".to_string())
            }
        };

        Ok(KijunValue::Suisyo(result))
    }

    // 銅
    pub fn get_copper(age: usize, gender: Gender) -> Result<KijunValue, String> {
        let result = match age {
            0 => {
                return Err("0歳以下は銅の推奨量を求めることができません".to_string())
            },
            1 ... 2 => {   gender_match!(gender, 0.3, 0.3) },
            3 ... 5 => {   gender_match!(gender, 0.4, 0.4) },
            6 ... 7 => {   gender_match!(gender, 0.5, 0.5) },
            8 ... 9 => {   gender_match!(gender, 0.6, 0.5) },
            10 ... 11 => { gender_match!(gender, 0.7, 0.7) },
            12 ... 14 => { gender_match!(gender, 0.8, 0.8) },
            15 ... 17 => { gender_match!(gender, 1.0, 0.8) },
            18 ... 29 => { gender_match!(gender, 0.9, 0.8) },
            30 ... 49 => { gender_match!(gender, 1.0, 0.8) },
            50 ... 69 => { gender_match!(gender, 0.9, 0.8) },
            age if 70 <= age => { gender_match!(gender, 0.9, 0.7) },
            _ => {
                return Err("銅の推奨量を求めることができません".to_string())
            }
        };

        Ok(KijunValue::Suisyo(result))
    }

    // マンガン
    pub fn get_manganese(age: usize, gender: Gender) -> Result<KijunValue, String> {
        let result = match age {
            0 => {
                return Err("0歳以下はマンガンの目安量を求めることができません".to_string())
            },
            1 ... 2 => {   gender_match!(gender, 1.5, 1.5) },
            3 ... 5 => {   gender_match!(gender, 1.5, 1.5) },
            6 ... 7 => {   gender_match!(gender, 2.0, 2.0) },
            8 ... 9 => {   gender_match!(gender, 2.5, 2.5) },
            10 ... 11 => { gender_match!(gender, 3.0, 3.0) },
            12 ... 14 => { gender_match!(gender, 4.0, 4.0) },
            15 ... 17 => { gender_match!(gender, 4.5, 3.5) },
            18 ... 29 => { gender_match!(gender, 4.0, 3.5) },
            30 ... 49 => { gender_match!(gender, 4.0, 3.5) },
            50 ... 69 => { gender_match!(gender, 4.0, 3.5) },
            age if 70 <= age => { gender_match!(gender, 4.0, 3.5) },
            _ => {
                return Err("マンガンの目安量を求めることができません".to_string())
            }
        };

        Ok(KijunValue::Measu(result))
    }

    // ヨウ素
    pub fn get_iodine(age: usize, gender: Gender) -> Result<KijunValue, String> {
        let result = match age {
            0 => {
                return Err("0歳以下はヨウ素の推奨量を求めることができません".to_string())
            },
            1 ... 2 => {   gender_match!(gender, 50.0, 50.0) },
            3 ... 5 => {   gender_match!(gender, 60.0, 60.0) },
            6 ... 7 => {   gender_match!(gender, 75.0, 75.0) },
            8 ... 9 => {   gender_match!(gender, 90.0, 90.0) },
            10 ... 11 => { gender_match!(gender, 110.0, 110.0) },
            12 ... 14 => { gender_match!(gender, 140.0, 140.0) },
            15 ... 17 => { gender_match!(gender, 140.0, 140.0) },
            18 ... 29 => { gender_match!(gender, 130.0, 130.0) },
            30 ... 49 => { gender_match!(gender, 130.0, 130.0) },
            50 ... 69 => { gender_match!(gender, 130.0, 130.0) },
            age if 70 <= age => { gender_match!(gender, 130.0, 130.0) },
            _ => {
                return Err("ヨウ素の推奨量を求めることができません".to_string())
            }
        };

        Ok(KijunValue::Suisyo(result))
    }

    // セレン
    pub fn get_selenium(age: usize, gender: Gender) -> Result<KijunValue, String> {
        let result = match age {
            0 => {
                return Err("0歳以下はセレンの推奨量を求めることができません".to_string())
            },
            1 ... 2 => {   gender_match!(gender, 10.0, 10.0) },
            3 ... 5 => {   gender_match!(gender, 15.0, 10.0) },
            6 ... 7 => {   gender_match!(gender, 15.0, 15.0) },
            8 ... 9 => {   gender_match!(gender, 20.0, 20.0) },
            10 ... 11 => { gender_match!(gender, 25.0, 25.0) },
            12 ... 14 => { gender_match!(gender, 30.0, 30.0) },
            15 ... 17 => { gender_match!(gender, 35.0, 25.0) },
            18 ... 29 => { gender_match!(gender, 30.0, 25.0) },
            30 ... 49 => { gender_match!(gender, 30.0, 25.0) },
            50 ... 69 => { gender_match!(gender, 30.0, 25.0) },
            age if 70 <= age => { gender_match!(gender, 30.0, 25.0) },
            _ => {
                return Err("セレンの推奨量を求めることができません".to_string())
            }
        };

        Ok(KijunValue::Suisyo(result))
    }

    // クロム
    pub fn get_chromium(age: usize, gender: Gender) -> Result<KijunValue, String> {
        let result = match age {
            0 ... 17 => {
                return Err("17歳以下はクロムの目安量を求めることができません".to_string())
            },
            18 ... 29 => { gender_match!(gender, 10.0, 10.0) },
            30 ... 49 => { gender_match!(gender, 10.0, 10.0) },
            50 ... 69 => { gender_match!(gender, 10.0, 10.0) },
            age if 70 <= age => { gender_match!(gender, 10.0, 10.0) },
            _ => {
                return Err("クロムの目安量を求めることができません".to_string())
            }
        };

        Ok(KijunValue::Measu(result))
    }

    // モリブデン
    pub fn get_molybdenum(age: usize, gender: Gender) -> Result<KijunValue, String> {
        let result = match age {
            0 ... 17 => {
                return Err("17歳以下はモリブデンの推奨量を求めることができません".to_string())
            },
            18 ... 29 => { gender_match!(gender, 25.0, 20.0) },
            30 ... 49 => { gender_match!(gender, 30.0, 25.0) },
            50 ... 69 => { gender_match!(gender, 25.0, 25.0) },
            age if 70 <= age => { gender_match!(gender, 25.0, 20.0) },
            _ => {
                return Err("モリブデンの推奨量を求めることができません".to_string())
            }
        };

        Ok(KijunValue::Suisyo(result))
    }
}
