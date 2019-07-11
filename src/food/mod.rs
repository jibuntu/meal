use std::collections::HashMap;

pub mod food_data;
use food_data::FoodData;

#[derive(Clone, Debug, PartialEq)]
pub struct Food {
    pub group: FoodData,        // 食品群
    pub number: FoodData,       // 食品番号
    pub index: FoodData,        // 索引番号
    pub name: FoodData,         // 食品名
    pub weight: FoodData,       // 重量
    pub refuse: FoodData,       // 廃棄率
    pub enerc_kcal: FoodData,   // エネルギー（kcal）
    pub enerc: FoodData,        // エネルギー（kJ)
    pub water: FoodData,        // 水分
    pub protein: FoodData,      // たんぱく質
    pub protcaa: FoodData,      // アミノ酸組成によるたんぱく質
    pub lipid: FoodData,        // 脂質
    pub fatnlea: FoodData,      // トリアシルグリセロール当量
    pub fasat: FoodData,        // 飽和脂肪酸
    pub fams: FoodData,         // 一価不飽和脂肪酸
    pub fapu: FoodData,         // 多価不飽和脂肪酸
    pub chole: FoodData,        // コレステロール
    pub carbohydrate: FoodData, // 炭水化物
    pub choavlm: FoodData,      // 利用可能炭水化物（単糖当量）
    pub fibsol: FoodData,       // 水溶性食物繊維
    pub fibins: FoodData,       // 不溶性食物繊維
    pub fibtg: FoodData,        // 食物繊維総量
    pub ash: FoodData,          // 灰分
    pub na: FoodData,           // ナトリウム
    pub k: FoodData,            // カリウム
    pub ca: FoodData,           // カルシウム
    pub mg: FoodData,           // マグネシウム
    pub p: FoodData,            // リン
    pub fe: FoodData,           // 鉄
    pub zn: FoodData,           // 亜鉛
    pub cu: FoodData,           // 銅
    pub mn: FoodData,           // マンガン
    pub id: FoodData,           // ヨウ素
    pub se: FoodData,           // セレン
    pub cr: FoodData,           // クロム
    pub mo: FoodData,           // モリブデン
    pub retol: FoodData,        // レチノール
    pub carta: FoodData,        // α-カロテン
    pub cartb: FoodData,        // β-カロテン
    pub crypxb: FoodData,       // β-クリプトキサンチン
    pub cartbeq: FoodData,      // β-カロテン当量
    pub vita_rae: FoodData,     // レチノール活性当量
    pub vitd: FoodData,         // ビタミンD
    pub tocpha: FoodData,       // α-トコフェロール
    pub tocphb: FoodData,       // β-トコフェロール
    pub tocphg: FoodData,       // γ-トコフェロール
    pub tocphd: FoodData,       // δ-トコフェロール
    pub vitk: FoodData,         // ビタミンK
    pub thiahcl: FoodData,      // ビタミンB1
    pub ribf: FoodData,         // ビタミンB2
    pub nia: FoodData,          // ナイアシン
    pub vitb6a: FoodData,       // ビタミンB6
    pub vitb12: FoodData,       // ビタミンB12
    pub fol: FoodData,          // 葉酸
    pub pantac: FoodData,       // パントテン酸
    pub biot: FoodData,         // ビオチン
    pub vitc: FoodData,         // ビタミンC
    pub nacl_eq: FoodData,      // 食塩相当量
    pub alc: FoodData,          // アルコール
    pub nitra: FoodData,        // 硝酸イオン
    pub thebrn: FoodData,       // テオブロミン
    pub caffn: FoodData,        // カフェイン
    pub tan: FoodData,          // タンニン
    pub polyphent: FoodData,    // ポリフェノール
    pub aceac: FoodData,        // 酢酸
    pub cooking_oil: FoodData,  // 調理油
    pub oa: FoodData,           // 有機酸
    pub yield_per: FoodData,    // 重量変化率
    pub remark: FoodData        // 備考
}

impl Food {
    pub fn new() -> Food {
        Food {
            group: FoodData::None,        // 食品群
            number: FoodData::None,       // 食品番号
            index: FoodData::None,        // 索引番号
            name: FoodData::None,         // 食品名
            weight: FoodData::None,       // 重量
            refuse: FoodData::None,       // 廃棄率
            enerc_kcal: FoodData::None,   // エネルギー（kcal）
            enerc: FoodData::None,        // エネルギー（kJ)
            water: FoodData::None,        // 水分
            protein: FoodData::None,      // たんぱく質
            protcaa: FoodData::None,      // アミノ酸組成によるたんぱく質
            lipid: FoodData::None,        // 脂質
            fatnlea: FoodData::None,      // トリアシルグリセロール当量
            fasat: FoodData::None,        // 飽和脂肪酸
            fams: FoodData::None,         // 一価不飽和脂肪酸
            fapu: FoodData::None,         // 多価不飽和脂肪酸
            chole: FoodData::None,        // コレステロール
            carbohydrate: FoodData::None, // 炭水化物
            choavlm: FoodData::None,      // 利用可能炭水化物（単糖当量）
            fibsol: FoodData::None,       // 水溶性食物繊維
            fibins: FoodData::None,       // 不溶性食物繊維
            fibtg: FoodData::None,        // 食物繊維総量
            ash: FoodData::None,          // 灰分
            na: FoodData::None,           // ナトリウム
            k: FoodData::None,            // カリウム
            ca: FoodData::None,           // カルシウム
            mg: FoodData::None,           // マグネシウム
            p: FoodData::None,            // リン
            fe: FoodData::None,           // 鉄
            zn: FoodData::None,           // 亜鉛
            cu: FoodData::None,           // 銅
            mn: FoodData::None,           // マンガン
            id: FoodData::None,           // ヨウ素
            se: FoodData::None,           // セレン
            cr: FoodData::None,           // クロム
            mo: FoodData::None,           // モリブデン
            retol: FoodData::None,        // レチノール
            carta: FoodData::None,        // α-カロテン
            cartb: FoodData::None,        // β-カロテン
            crypxb: FoodData::None,       // β-クリプトキサンチン
            cartbeq: FoodData::None,      // β-カロテン当量
            vita_rae: FoodData::None,     // レチノール活性当量
            vitd: FoodData::None,         // ビタミンD
            tocpha: FoodData::None,       // α-トコフェロール
            tocphb: FoodData::None,       // β-トコフェロール
            tocphg: FoodData::None,       // γ-トコフェロール
            tocphd: FoodData::None,       // δ-トコフェロール
            vitk: FoodData::None,         // ビタミンK
            thiahcl: FoodData::None,      // ビタミンB1
            ribf: FoodData::None,         // ビタミンB2
            nia: FoodData::None,          // ナイアシン
            vitb6a: FoodData::None,       // ビタミンB6
            vitb12: FoodData::None,       // ビタミンB12
            fol: FoodData::None,          // 葉酸
            pantac: FoodData::None,       // パントテン酸
            biot: FoodData::None,         // ビオチン
            vitc: FoodData::None,         // ビタミンC
            nacl_eq: FoodData::None,      // 食塩相当量
            alc: FoodData::None,          // アルコール
            nitra: FoodData::None,        // 硝酸イオン
            thebrn: FoodData::None,       // テオブロミン
            caffn: FoodData::None,        // カフェイン
            tan: FoodData::None,          // タンニン
            polyphent: FoodData::None,    // ポリフェノール
            aceac: FoodData::None,        // 酢酸
            cooking_oil: FoodData::None,  // 調理油
            oa: FoodData::None,           // 有機酸
            yield_per: FoodData::None,    // 重量変化率
            remark: FoodData::None        // 備考
        }
    }

    pub fn set(&mut self, name: &str, value: FoodData) {
        let food_data = match name {
            "食品群" => &mut self.group,
            "食品番号" => &mut self.number,
            "索引番号" => &mut self.index,
            "食品名" => &mut self.name,
            "重量" => &mut self.weight,
            "廃棄率" => &mut self.refuse,
            "エネルギー（kcal）" | "エネルギー" => &mut self.enerc_kcal,
            "エネルギー（kJ)" => &mut self.enerc,
            "水分" => &mut self.water,
            "たんぱく質" => &mut self.protein,
            "アミノ酸組成によるたんぱく質" => &mut self.protcaa,
            "脂質" => &mut self.lipid,
            "トリアシルグリセロール当量" => &mut self.fatnlea,
            "飽和脂肪酸" => &mut self.fasat,
            "一価不飽和脂肪酸" => &mut self.fams,
            "多価不飽和脂肪酸" => &mut self.fapu,
            "コレステロール" => &mut self.chole,
            "炭水化物" => &mut self.carbohydrate,
            "利用可能炭水化物（単糖当量）" => &mut self.choavlm,
            "水溶性食物繊維" => &mut self.fibsol,
            "不溶性食物繊維" => &mut self.fibins,
            "食物繊維総量" => &mut self.fibtg,
            "灰分" => &mut self.ash,
            "ナトリウム" => &mut self.na,
            "カリウム" => &mut self.k,
            "カルシウム" => &mut self.ca,
            "マグネシウム" => &mut self.mg,
            "リン" => &mut self.p,
            "鉄" => &mut self.fe,
            "亜鉛" => &mut self.zn,
            "銅" => &mut self.cu,
            "マンガン" => &mut self.mn,
            "ヨウ素" => &mut self.id,
            "セレン" => &mut self.se,
            "クロム" => &mut self.cr,
            "モリブデン" => &mut self.mo,
            "レチノール" => &mut self.retol,
            "α-カロテン" => &mut self.carta,
            "β-カロテン" => &mut self.cartb,
            "β-クリプトキサンチン" => &mut self.crypxb,
            "β-カロテン当量" => &mut self.cartbeq,
            "レチノール活性当量" => &mut self.vita_rae,
            "ビタミンD" => &mut self.vitd,
            "α-トコフェロール" => &mut self.tocpha,
            "β-トコフェロール" => &mut self.tocphb,
            "γ-トコフェロール" => &mut self.tocphg,
            "δ-トコフェロール" => &mut self.tocphd,
            "ビタミンK" => &mut self.vitk,
            "ビタミンB1" => &mut self.thiahcl,
            "ビタミンB2" => &mut self.ribf,
            "ナイアシン" => &mut self.nia,
            "ビタミンB6" => &mut self.vitb6a,
            "ビタミンB12" => &mut self.vitb12,
            "葉酸" => &mut self.fol,
            "パントテン酸" => &mut self.pantac,
            "ビオチン" => &mut self.biot,
            "ビタミンC" => &mut self.vitc,
            "食塩相当量" => &mut self.nacl_eq,
            "アルコール" => &mut self.alc,
            "硝酸イオン" => &mut self.nitra,
            "テオブロミン" => &mut self.thebrn,
            "カフェイン" => &mut self.caffn,
            "タンニン" => &mut self.tan,
            "ポリフェノール" => &mut self.polyphent,
            "酢酸" => &mut self.aceac,
            "調理油" => &mut self.cooking_oil,
            "有機酸" => &mut self.oa,
            "重量変化率" => &mut self.yield_per,
            "備考" => &mut self.remark,
            _ => return ()
        };

        *food_data = value;
    }

    pub fn get(&self, name: &str) -> Option<&FoodData> {
        let food_data = match name {
            "食品群" => &self.group,
            "食品番号" => &self.number,
            "索引番号" => &self.index,
            "食品名" => &self.name,
            "重量" => &self.weight,
            "廃棄率" => &self.refuse,
            "エネルギー（kcal）" | "エネルギー" => &self.enerc_kcal,
            "エネルギー（kJ)" => &self.enerc,
            "水分" => &self.water,
            "たんぱく質" => &self.protein,
            "アミノ酸組成によるたんぱく質" => &self.protcaa,
            "脂質" => &self.lipid,
            "トリアシルグリセロール当量" => &self.fatnlea,
            "飽和脂肪酸" => &self.fasat,
            "一価不飽和脂肪酸" => &self.fams,
            "多価不飽和脂肪酸" => &self.fapu,
            "コレステロール" => &self.chole,
            "炭水化物" => &self.carbohydrate,
            "利用可能炭水化物（単糖当量）" => &self.choavlm,
            "水溶性食物繊維" => &self.fibsol,
            "不溶性食物繊維" => &self.fibins,
            "食物繊維総量" => &self.fibtg,
            "灰分" => &self.ash,
            "ナトリウム" => &self.na,
            "カリウム" => &self.k,
            "カルシウム" => &self.ca,
            "マグネシウム" => &self.mg,
            "リン" => &self.p,
            "鉄" => &self.fe,
            "亜鉛" => &self.zn,
            "銅" => &self.cu,
            "マンガン" => &self.mn,
            "ヨウ素" => &self.id,
            "セレン" => &self.se,
            "クロム" => &self.cr,
            "モリブデン" => &self.mo,
            "レチノール" => &self.retol,
            "α-カロテン" => &self.carta,
            "β-カロテン" => &self.cartb,
            "β-クリプトキサンチン" => &self.crypxb,
            "β-カロテン当量" => &self.cartbeq,
            "レチノール活性当量" => &self.vita_rae,
            "ビタミンD" => &self.vitd,
            "α-トコフェロール" => &self.tocpha,
            "β-トコフェロール" => &self.tocphb,
            "γ-トコフェロール" => &self.tocphg,
            "δ-トコフェロール" => &self.tocphd,
            "ビタミンK" => &self.vitk,
            "ビタミンB1" => &self.thiahcl,
            "ビタミンB2" => &self.ribf,
            "ナイアシン" => &self.nia,
            "ビタミンB6" => &self.vitb6a,
            "ビタミンB12" => &self.vitb12,
            "葉酸" => &self.fol,
            "パントテン酸" => &self.pantac,
            "ビオチン" => &self.biot,
            "ビタミンC" => &self.vitc,
            "食塩相当量" => &self.nacl_eq,
            "アルコール" => &self.alc,
            "硝酸イオン" => &self.nitra,
            "テオブロミン" => &self.thebrn,
            "カフェイン" => &self.caffn,
            "タンニン" => &self.tan,
            "ポリフェノール" => &self.polyphent,
            "酢酸" => &self.aceac,
            "調理油" => &self.cooking_oil,
            "有機酸" => &self.oa,
            "重量変化率" => &self.yield_per,
            "備考" => &self.remark,
            _ => return None
        };

        Some(food_data)
    }

    pub fn get_list(&self, keys: &[&str]) -> Vec<Option<&FoodData>> {
        let mut list = Vec::with_capacity(keys.len());
        for key in keys {
            list.push(self.get(key))
        }

        list
    }

    pub fn change_weight(&self, weight: f32) -> Option<Food> {
        let rate = match self.weight.get_number() {
            None => return None,
            Some(num) => weight / *num
        };

        let food = Food {
            group: self.group.clone(),                  // 食品群
            number: self.number.clone(),                // 食品番号
            index: self.index.clone(),                  // 索引番号
            name: self.name.clone(),                    // 食品名
            weight: self.weight.rate(rate),             // 重量
            refuse: self.refuse.rate(rate),             // 廃棄率
            enerc_kcal: self.enerc_kcal.rate(rate),     // エネルギー（kcal）
            enerc: self.enerc.rate(rate),               // エネルギー（kJ)
            water: self.water.rate(rate),               // 水分
            protein: self.protein.rate(rate),           // たんぱく質
            protcaa: self.protcaa.rate(rate),           // アミノ酸組成によるたんぱく質
            lipid: self.lipid.rate(rate),               // 脂質
            fatnlea: self.fatnlea.rate(rate),           // トリアシルグリセロール当量
            fasat: self.fasat.rate(rate),               // 飽和脂肪酸
            fams: self.fams.rate(rate),                 // 一価不飽和脂肪酸
            fapu: self.fapu.rate(rate),                 // 多価不飽和脂肪酸
            chole: self.chole.rate(rate),               // コレステロール
            carbohydrate: self.carbohydrate.rate(rate), // 炭水化物
            choavlm: self.choavlm.rate(rate),           // 利用可能炭水化物（単糖当量）
            fibsol: self.fibsol.rate(rate),             // 水溶性食物繊維
            fibins: self.fibins.rate(rate),             // 不溶性食物繊維
            fibtg: self.fibtg.rate(rate),               // 食物繊維総量
            ash: self.ash.rate(rate),                   // 灰分
            na: self.na.rate(rate),                     // ナトリウム
            k: self.k.rate(rate),                       // カリウム
            ca: self.ca.rate(rate),                     // カルシウム
            mg: self.mg.rate(rate),                     // マグネシウム
            p: self.p.rate(rate),                       // リン
            fe: self.fe.rate(rate),                     // 鉄
            zn: self.zn.rate(rate),                     // 亜鉛
            cu: self.cu.rate(rate),                     // 銅
            mn: self.mn.rate(rate),                     // マンガン
            id: self.id.rate(rate),                     // ヨウ素
            se: self.se.rate(rate),                     // セレン
            cr: self.cr.rate(rate),                     // クロム
            mo: self.mo.rate(rate),                     // モリブデン
            retol: self.retol.rate(rate),               // レチノール
            carta: self.carta.rate(rate),               // α-カロテン
            cartb: self.cartb.rate(rate),               // β-カロテン
            crypxb: self.crypxb.rate(rate),             // β-クリプトキサンチン
            cartbeq: self.cartbeq.rate(rate),           // β-カロテン当量
            vita_rae: self.vita_rae.rate(rate),         // レチノール活性当量
            vitd: self.vitd.rate(rate),                 // ビタミンD
            tocpha: self.tocpha.rate(rate),             // α-トコフェロール
            tocphb: self.tocphb.rate(rate),             // β-トコフェロール
            tocphg: self.tocphg.rate(rate),             // γ-トコフェロール
            tocphd: self.tocphd.rate(rate),             // δ-トコフェロール
            vitk: self.vitk.rate(rate),                 // ビタミンK
            thiahcl: self.thiahcl.rate(rate),           // ビタミンB1
            ribf: self.ribf.rate(rate),                 // ビタミンB2
            nia: self.nia.rate(rate),                   // ナイアシン
            vitb6a: self.vitb6a.rate(rate),             // ビタミンB6
            vitb12: self.vitb12.rate(rate),             // ビタミンB12
            fol: self.fol.rate(rate),                   // 葉酸
            pantac: self.pantac.rate(rate),             // パントテン酸
            biot: self.biot.rate(rate),                 // ビオチン
            vitc: self.vitc.rate(rate),                 // ビタミンC
            nacl_eq: self.nacl_eq.rate(rate),           // 食塩相当量
            alc: self.alc.rate(rate),                   // アルコール
            nitra: self.nitra.rate(rate),               // 硝酸イオン
            thebrn: self.thebrn.rate(rate),             // テオブロミン
            caffn: self.caffn.rate(rate),               // カフェイン
            tan: self.tan.rate(rate),                   // タンニン
            polyphent: self.polyphent.rate(rate),       // ポリフェノール
            aceac: self.aceac.rate(rate),               // 酢酸
            cooking_oil: self.cooking_oil.rate(rate),   // 調理油
            oa: self.oa.rate(rate),                     // 有機酸
            yield_per: self.yield_per.clone(),          // 重量変化率
            remark: self.remark.clone()                 // 備考
        };

        Some(food)
    }

    pub fn add(&self, food: &Food) -> Food {
        Food {
            group: FoodData::None,
            number: FoodData::None,
            index: FoodData::None,
            name: FoodData::None,
            weight: self.weight.add(&food.weight),
            refuse: self.refuse.add(&food.refuse),
            enerc_kcal: self.enerc_kcal.add(&food.enerc_kcal),
            enerc: self.enerc.add(&food.enerc),
            water: self.water.add(&food.water),
            protein: self.protein.add(&food.protein),
            protcaa: self.protcaa.add(&food.protcaa),
            lipid: self.lipid.add(&food.lipid),
            fatnlea: self.fatnlea.add(&food.fatnlea),
            fasat: self.fasat.add(&food.fasat),
            fams: self.fams.add(&food.fams),
            fapu: self.fapu.add(&food.fapu),
            chole: self.chole.add(&food.chole),
            carbohydrate: self.carbohydrate.add(&food.carbohydrate),
            choavlm: self.choavlm.add(&food.choavlm),
            fibsol: self.fibsol.add(&food.fibsol),
            fibins: self.fibins.add(&food.fibins),
            fibtg: self.fibtg.add(&food.fibtg),
            ash: self.ash.add(&food.ash),
            na: self.na.add(&food.na),
            k: self.k.add(&food.k),
            ca: self.ca.add(&food.ca),
            mg: self.mg.add(&food.mg),
            p: self.p.add(&food.p),
            fe: self.fe.add(&food.fe),
            zn: self.zn.add(&food.zn),
            cu: self.cu.add(&food.cu),
            mn: self.mn.add(&food.mn),
            id: self.id.add(&food.id),
            se: self.se.add(&food.se),
            cr: self.cr.add(&food.cr),
            mo: self.mo.add(&food.mo),
            retol: self.retol.add(&food.retol),
            carta: self.carta.add(&food.carta),
            cartb: self.cartb.add(&food.cartb),
            crypxb: self.crypxb.add(&food.crypxb),
            cartbeq: self.cartbeq.add(&food.cartbeq),
            vita_rae: self.vita_rae.add(&food.vita_rae),
            vitd: self.vitd.add(&food.vitd),
            tocpha: self.tocpha.add(&food.tocpha),
            tocphb: self.tocphb.add(&food.tocphb),
            tocphg: self.tocphg.add(&food.tocphg),
            tocphd: self.tocphd.add(&food.tocphd),
            vitk: self.vitk.add(&food.vitk),
            thiahcl: self.thiahcl.add(&food.thiahcl),
            ribf: self.ribf.add(&food.ribf),
            nia: self.nia.add(&food.nia),
            vitb6a: self.vitb6a.add(&food.vitb6a),
            vitb12: self.vitb12.add(&food.vitb12),
            fol: self.fol.add(&food.fol),
            pantac: self.pantac.add(&food.pantac),
            biot: self.biot.add(&food.biot),
            vitc: self.vitc.add(&food.vitc),
            nacl_eq: self.nacl_eq.add(&food.nacl_eq),
            alc: self.alc.add(&food.alc),
            nitra: self.nitra.add(&food.nitra),
            thebrn: self.thebrn.add(&food.thebrn),
            caffn: self.caffn.add(&food.caffn),
            tan: self.tan.add(&food.tan),
            polyphent: self.polyphent.add(&food.polyphent),
            aceac: self.aceac.add(&food.aceac),
            cooking_oil: self.cooking_oil.add(&food.cooking_oil),
            oa: self.oa.add(&food.oa),
            yield_per: self.yield_per.add(&food.yield_per),
            remark: FoodData::None
        }
    }
}


#[test]
fn test_food_new() {
    let food = Food::new();
}

#[test]
fn test_food_get() {
    let mut food = Food::new();
    assert_eq!(food.get("food"), None);

    assert_eq!(food.get("エネルギー"), Some(&FoodData::None));

    food.set("エネルギー", FoodData::Number(200.0));
    assert_eq!(food.get("エネルギー"), Some(&FoodData::Number(200.0)));

    food.set("たんぱく質", FoodData::EstimatedNumber(200.0));
    assert_eq!(food.get("たんぱく質"), Some(&FoodData::EstimatedNumber(200.0)));

    food.set("食品名", FoodData::String("ネギ".to_string()));
    assert_eq!(food.get("食品名"), Some(&FoodData::String("ネギ".to_string())));
}

#[test]
fn test_food_get_list() {
    let mut food = Food::new();

    food.set("食品名", FoodData::String("馬肉".to_string()));
    food.set("エネルギー", FoodData::Number(200.0));
    food.set("ナイアシン",  FoodData::Number(10.0));

    assert_eq!(food.get_list(&["食品名", "エネルギー", "ナイアシン"]),
               vec![Some(&FoodData::String("馬肉".to_string())),
                    Some(&FoodData::Number(200.0)),
                    Some(&FoodData::Number(10.0))]);
}

#[test]
fn test_food_change_weight() {
    let mut food = Food::new();
    assert_eq!(food.change_weight(10.0), None);

    let mut food = Food::new();
    food.set("食品名", FoodData::String("馬肉".to_string()));
    food.set("葉酸", FoodData::Number(200.0));
    food.set("ビオチン",  FoodData::Number(10.0));
    food.set("重量",  FoodData::Number(100.0));
    let new_food = food.change_weight(50.0).unwrap();
    assert_eq!(new_food.get("重量"), Some(&FoodData::Number(50.0)));
    food.set("重量",  FoodData::Number(50.0));
    let new_food = food.change_weight(27.0).unwrap();
    assert_eq!(new_food.get("重量").unwrap().to_string().as_str(), "27");
}

#[test]
fn test_food_add() {
    let mut left = Food::new();
    let mut right = Food::new();

    left.set("食品名", FoodData::String("食品".to_string()));
    left.set("クロム", FoodData::Number(10.0));
    right.set("食品名", FoodData::String("食品".to_string()));
    right.set("クロム", FoodData::Number(20.0));
    let result = left.add(&right);
    assert_eq!(result.get("食品名"), Some(&FoodData::None));
    assert_eq!(result.get("クロム"), Some(&FoodData::Number(30.0)));

    left.set("モリブデン", FoodData::Number(50.0));
    let result = left.add(&right);
    assert_eq!(result.get("モリブデン"), Some(&FoodData::Number(50.0)));
}