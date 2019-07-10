use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub enum FoodData {
    Number(f32),
    EstimatedNumber(f32),
    String(String),
    None,
}

impl FoodData {
    pub fn from_str(data: &str) -> FoodData {
        match f32::from_str(data) {
            Ok(num) => FoodData::Number(num),
            Err(_) => {
                match f32::from_str(data.trim_start_matches("(")
                    .trim_end_matches(")")) {
                    Ok(num) => FoodData::EstimatedNumber(num),
                    Err(_) => FoodData::String(data.to_string())
                }
            }
        }
    }

    pub fn get_number(&self) -> Option<&f32> {
        match self {
            FoodData::Number(num) => Some(num),
            FoodData::EstimatedNumber(num) => Some(num),
            _ => None
        }
    }

    pub fn rate(&self, rate: f32) -> FoodData {
        match self {
            FoodData::Number(num) => FoodData::Number(*num * rate),
            FoodData::EstimatedNumber(num) => {
                FoodData::EstimatedNumber(*num * rate)
            },
            _ => return self.clone(),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            FoodData::Number(number) => {
                if *number == 0.0 {
                    "0".to_string()
                } else {
                    let num = (*number * 100.0).round();
                    let num = num / 100.0;
                    num.to_string()
                }
            },
            FoodData::EstimatedNumber(number) => {
                if *number == 0.0 {
                    "(0)".to_string()
                } else {
                    let num = (*number * 100.0).round();
                    let num = num / 100.0;
                    "(".to_string() + &num.to_string() + ")"
                }
            }
            FoodData::String(string) => string.clone(),
            _ => "-".to_string()
        }
    }
}


#[test]
fn test_food_data_from_str() {
    let food_data = FoodData::from_str("100");
    assert_eq!(food_data, FoodData::Number(100.0));

    let food_data = FoodData::from_str("(100)");
    assert_eq!(food_data, FoodData::EstimatedNumber(100.0));

    let food_data = FoodData::from_str("(100.0)");
    assert_eq!(food_data, FoodData::EstimatedNumber(100.0));

    let food_data = FoodData::from_str("(0.1)");
    assert_eq!(food_data, FoodData::EstimatedNumber(0.1));

    let food_data = FoodData::from_str("(1.1)");
    assert_eq!(food_data, FoodData::EstimatedNumber(1.1));

    let food_data = FoodData::from_str("(0)");
    assert_eq!(food_data, FoodData::EstimatedNumber(0.0));

    let food_data = FoodData::from_str("(Tr)");
    assert_eq!(food_data, FoodData::String("(Tr)".to_string()));
}

#[test]
fn test_food_data_rate() {
    let food_data = FoodData::Number(100.0);
    assert_eq!(food_data.rate(0.5), FoodData::Number(50.0));

    let food_data = FoodData::Number(100.0);
    assert_eq!(food_data.rate(2.0), FoodData::Number(200.0));

    let food_data = FoodData::EstimatedNumber(100.0);
    assert_eq!(food_data.rate(2.0), FoodData::EstimatedNumber(200.0));
}

#[test]
fn test_food_data_to_string() {
    let food_data = FoodData::from_str("100");
    assert_eq!(&food_data.to_string(), "100");

    let food_data = FoodData::from_str("(100)");
    assert_eq!(&food_data.to_string(), "(100)");

    let food_data = FoodData::from_str("(100.0)");
    assert_eq!(&food_data.to_string(), "(100)");

    let food_data = FoodData::from_str("(0.1)");
    assert_eq!(&food_data.to_string(), "(0.1)");

    let food_data = FoodData::from_str("(1.1)");
    assert_eq!(&food_data.to_string(), "(1.1)");

    let food_data = FoodData::from_str("(0)");
    assert_eq!(&food_data.to_string(), "(0)");

    let food_data = FoodData::from_str("(Tr)");
    assert_eq!(&food_data.to_string(), "(Tr)");
}