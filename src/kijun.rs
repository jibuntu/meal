use std::collections::HashMap;

enum PAL {
    Low,
    Moderate,
    High
}

enum Gender {
    Female,
    Male
}

pub struct Kijun {
    age: usize,
    weight: f32,
    height: f32,
    gender: Gender,
    pal: PAL,
    data_list: HashMap<String, KijunValue>
}

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

enum KijunValue {
    Suisyo(f32), // 値になるべく近い方が望ましい
    Less(f32), // この値より小さい方が望ましい
    More(f32), // この値より大きい方が望ましい
    Range((f32, f32)), // (mix, max) この値の範囲ないが望ましい
    Measu(f32) // 値になるべく近い方が望ましいが、あくまで目安である
}

impl Kijun {
    // 身体活動レベル
    fn get_pal(age: usize, pal: PAL) -> Result<KijunValue, String> {
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

        return Ok(KijunValue::Suisyo(result))
    }

    // 基礎代謝量
    fn get_base_metabolism(weight: f32,
                           height: f32,
                           age: usize,
                           gender: Gender) -> KijunValue {
        KijunValue::Suisyo(gender_match!(gender,
            (0.0481 * weight + 0.0234 * height - 0.0138 * age as f32 - 0.4235)
                    * 1000.0 / 4.186,
            (0.0481 * weight + 0.0234 * height - 0.0138 * age as f32 - 0.9708)
                * 1000.0 / 4.186))
    }

    // エネルギー必要量（kcal）
    fn get_energy(bm: f32, pal: f32, age: usize) -> Result<KijunValue, String> {
        if age <= 17 {
            return Err("17歳以下はエネルギー必要量を求めることができません".to_string())
        }

        Ok(KijunValue::Suisyo(bm * pal))

    }

    // たんぱく質
    fn get_protein(age: usize, gender: Gender) -> Result<KijunValue, String>{
        let result = match age {
            0 => {
                return Err("0歳以下はたんぱく質の推奨量を求めることができません".to_string())
            },
            1 ... 2 => {   gender_match!(gender, 20.0, 20.0) },
            3 ... 5 => {   gender_match!(gender, 25.0, 25.0) },
            6 ... 7 => {   gender_match!(gender, 35.0, 30.0) },
            8 ... 9 => {   gender_match!(gender, 40.0, 40.0) },
            10 ... 11 => { gender_match!(gender, 50.0, 50.0) },
            12 ... 14 => { gender_match!(gender, 60.0, 55.0) },
            15 ... 17 => { gender_match!(gender, 65.0, 55.0) },
            18 ... 29 => { gender_match!(gender, 60.0, 50.0) },
            30 ... 49 => { gender_match!(gender, 60.0, 50.0) },
            50 ... 69 => { gender_match!(gender, 60.0, 50.0) },
            age if 70 <= age => { gender_match!(gender, 60.0, 50.0) },
            _ => {
                return Err("たんぱく質の推奨量を求めることができません".to_string())
            }
        };

        Ok(KijunValue::More(result))
    }

    // 脂質
    fn get_lipid(age: usize, energy: f32) -> Result<KijunValue, String> {
        if age <= 0 {
            return Err("0歳以下は脂質の目標量を求めることができません".to_string())
        }

        let result_min = (energy * 0.20 / 9.0);
        let result_max = (energy * 0.30 / 9.0);

        Ok(KijunValue::Range((result_min, result_min)))
    }

    // 飽和脂肪酸
    fn get_saturated_fatty_acid(age: usize, energy: f32) -> Result<KijunValue, String> {
        if age <= 17 {
            return Err("17歳以下は飽和脂肪酸の目標量を求めることができません".to_string())
        }

        Ok(KijunValue::Less((energy * 0.07 / 9.0)))
    }

    // n-6系脂肪酸
    fn get_n6_fatty_acid(age: usize, gender: Gender) -> Result<KijunValue, String> {
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
    fn get_n3_fatty_acid(age: usize, gender: Gender) -> Result<KijunValue, String> {
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

    // 炭水化物
    fn get_carbohydrate(age: usize, energy: f32) -> Result<KijunValue, String> {
        if age <= 0 {
            return Err("0歳以下は炭水化物の目標量を求めることができません".to_string())
        }
        let max = (energy * 0.50) / 4.0;
        let min = (energy * 0.65) / 4.0;

        Ok(KijunValue::Range((max, min)))
    }

    // 食物繊維
    fn get_fiber(age: usize, gender: Gender) -> Result<KijunValue, String> {
        let result = match age {
            0 ... 5 => {
                return Err("5歳以下は食物繊維の目標量を求めることができません".to_string())
            },
            6 ... 7 => {   gender_match!(gender, 11.0, 10.0) },
            8 ... 9 => {   gender_match!(gender, 12.0, 12.0) },
            10 ... 11 => { gender_match!(gender, 13.0, 13.0) },
            12 ... 14 => { gender_match!(gender, 17.0, 16.0) },
            15 ... 17 => { gender_match!(gender, 19.0, 17.0) },
            18 ... 29 => { gender_match!(gender, 20.0, 18.0) },
            30 ... 49 => { gender_match!(gender, 20.0, 18.0) },
            50 ... 69 => { gender_match!(gender, 20.0, 18.0) },
            age if 70 <= age => { gender_match!(gender, 19.0, 17.0) },
            _ => {
                return Err("食物繊維の目標量を求めることができません".to_string())
            }
        };

        Ok(KijunValue::More(result))
    }

    // ビタミンA
    fn get_vitamin_a(age: usize, gender: Gender) -> Result<KijunValue, String> {
        let result = match age {
            0 => {
                return Err("0歳以下はビタミンAの推奨量を求めることができません".to_string())
            },
            1 ... 2 => {   gender_match!(gender, 400.0, 350.0) },
            3 ... 5 => {   gender_match!(gender, 500.0, 400.0) },
            6 ... 7 => {   gender_match!(gender, 450.0, 400.0) },
            8 ... 9 => {   gender_match!(gender, 500.0, 500.0) },
            10 ... 11 => { gender_match!(gender, 600.0, 600.0) },
            12 ... 14 => { gender_match!(gender, 800.0, 700.0) },
            15 ... 17 => { gender_match!(gender, 900.0, 650.0) },
            18 ... 29 => { gender_match!(gender, 850.0, 650.0) },
            30 ... 49 => { gender_match!(gender, 900.0, 700.0) },
            50 ... 69 => { gender_match!(gender, 850.0, 700.0) },
            age if 70 <= age => { gender_match!(gender, 800.0, 650.0) },
            _ => {
                return Err("ビタミンAの推奨量を求めることができません".to_string())
            }
        };

        Ok(KijunValue::Suisyo(result))
    }

    // ビタミンD
    fn get_vitamin_d(age: usize, gender: Gender) -> Result<KijunValue, String> {
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
    fn get_vitamin_e(age: usize, gender: Gender) -> Result<KijunValue, String> {
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
    fn get_vitamin_k(age: usize, gender: Gender) -> Result<KijunValue, String> {
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
    fn get_vitamin_b1(age: usize, gender: Gender) -> Result<KijunValue, String> {
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
    fn get_vitamin_b2(age: usize, gender: Gender) -> Result<KijunValue, String> {
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
    fn get_niacin(age: usize, gender: Gender) -> Result<KijunValue, String> {
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
    fn get_vitamin_b6(age: usize, gender: Gender) -> Result<KijunValue, String> {
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
    fn get_vitamin_b12(age: usize, gender: Gender) -> Result<KijunValue, String> {
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
    fn get_folic_acid(age: usize, gender: Gender) -> Result<KijunValue, String> {
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
    fn get_pantothenic_acid(age: usize, gender: Gender) -> Result<KijunValue, String> {
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
    fn get_biotin(age: usize, gender: Gender) -> Result<KijunValue, String> {
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
    fn get_vitamin_c(age: usize, gender: Gender) -> Result<KijunValue, String> {
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
    fn get_sodium(age: usize, gender: Gender) -> Result<KijunValue, String> {
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
    fn get_potassium(age: usize, gender: Gender) -> Result<KijunValue, String> {
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
    fn get_calcium(age: usize, gender: Gender) -> Result<KijunValue, String> {
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
    fn get_magnesium(age: usize, gender: Gender) -> Result<KijunValue, String> {
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
    fn get_phosphorus(age: usize, gender: Gender) -> Result<KijunValue, String> {
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
    fn get_iron(age: usize, gender: Gender) -> Result<KijunValue, String> {
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
    fn get_zinc(age: usize, gender: Gender) -> Result<KijunValue, String> {
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
    fn get_copper(age: usize, gender: Gender) -> Result<KijunValue, String> {
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
    fn get_manganese(age: usize, gender: Gender) -> Result<KijunValue, String> {
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
    fn get_iodine(age: usize, gender: Gender) -> Result<KijunValue, String> {
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
    fn get_selenium(age: usize, gender: Gender) -> Result<KijunValue, String> {
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
    fn get_chromium(age: usize, gender: Gender) -> Result<KijunValue, String> {
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
    fn get_molybdenum(age: usize, gender: Gender) -> Result<KijunValue, String> {
        let result = match age {
            0 ... 17 => {
                return Err("0歳以下はモリブデンの推奨量を求めることができません".to_string())
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