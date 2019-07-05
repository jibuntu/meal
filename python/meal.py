import json;

class Foods():
    def __init__(self):
        self.data_list = []

    def food(self, food_number):
        self.data_list.append({
            "number": food_number,
        })
        return self
    
    # data_listの一番後ろにある要素のweightを設定する
    def weight(self, num):
        self.data_list[-1]["weight"] = num
        return self

    def w(self, num):
        return self.weight(num)
    
    def egg(self, num):
        return self.weight(50 * num)

class Body():
    def __init__(self):
        self.data = {}
    
    def age(self, num):
        self.data["age"] = num
        return self
    
    def weight(self, num):
        self.data["weight"] = num
        return self

    def height(self, num):
        self.data["height"] = num
        return self

    def gender(self, gender):
        if gender == "female":
            self.data["gender"] = "female"
        elif gender == "male":
            self.data["gender"] = "male"
        return self

    def pal(self, pal):
        if pal in {"low", 1}:
            self.data["pal"] = "low"
        elif pal in {"moderate", 2}:
            self.data["pal"] = "moderate"
        elif pal in {"high", 3}:
            self.data["pal"] = "high"
        return self

class Json:
    def __init__(self):
        self.data = {}

    def foods(self, foods):
        self.data["foods"] = foods.data_list
        return self

    def body(self, body):
        self.data["body"] = body.data
        return self

    def name_list(self, type):
        if type == "摂取基準":
            self.data["name_list"] = [
                "食品番号", "食品名", "重量", "エネルギー", "たんぱく質",
                "脂質", "多価不飽和脂肪酸", "炭水化物", "食物繊維総量", 
                "レチノール活性当量", "ビタミンD", "α-トコフェロール", 
                "ビタミンK", "ビタミンB1", "ビタミンB2", "ナイアシン", 
                "ビタミンB6", "ビタミンB12", "葉酸", "パントテン酸", 
                "ビオチン", "ビタミンC", "ナトリウム", "カリウム", 
                "カルシウム", "マグネシウム", "リン", "鉄", "亜鉛", "銅", 
                "マンガン", "ヨウ素", "セレン", "クロム", "モリブデン"
            ]
        return self

    def write(self, path):
        json_data = json.dumps(self.data)
        file = open(path, "w")
        file.write(json_data)
        return self
