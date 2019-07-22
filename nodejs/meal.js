const fs = require("fs")
const spawn = require("child_process").spawnSync

function Command() {
  this.calc = (path) => {
    let options = ["c"]
    
    if(Array.isArray(path)){
      path.forEach(p => options.push(p))
    }else{
      options.push(path)
    }
    
    let result = spawn("meal", options)
    console.log(result.stdout.toString())
    return this
  }
}

function Json() {
  this.data = {}
  this.path = ""
  
  this.foods = (foods) => {
    this.data["foods"] = foods.data_list
    return this
  }

  this.body = (body) => {
    this.data["body"] = body.data
    return this
  }

  this.name_list = (type) => {
    if(type == "摂取基準") {
      this.data["name_list"] = [
        "食品番号", "食品名", "価格", "廃棄率", "重量", "エネルギー", "たんぱく質",
        "脂質", "飽和脂肪酸", "多価不飽和脂肪酸", "炭水化物", "食物繊維総量", 
        "レチノール活性当量", "ビタミンD", "α-トコフェロール", 
        "ビタミンK", "ビタミンB1", "ビタミンB2", "ナイアシン", 
        "ビタミンB6", "ビタミンB12", "葉酸", "パントテン酸", 
        "ビオチン", "ビタミンC", "ナトリウム", "カリウム", 
        "カルシウム", "マグネシウム", "リン", "鉄", "亜鉛", "銅", 
        "マンガン", "ヨウ素", "セレン", "クロム", "モリブデン"
      ]
    }
    return this
  }

  this.combination = (list) => {
    if(Array.isArray(list) === false) {
      this.data["combination"] = [list]
      return this
    }
    this.data["combination"] = list
    return this
  }

  this.comb = (list) => {
    return this.combination(list)
  }

  this.path = (path) => {
    this.path = path
    return this
  }

  this.write = () => {
    let json_data = JSON.stringify(this.data)
    let file = fs.openSync(this.path, "w")
    fs.writeSync(file, json_data, "uft8")
    return this
  }

  this.automaticSelection = () => {
    let result = spawn("meal", ["a", this.path])
    console.log(result.stdout.toString())
    return this
  }

  this.calc = () => {
    let result = spawn("meal", ["c", this.path])
    console.log(result.stdout.toString())
    return this
  }
}

function Body() {
  this.data = {}

  this.age = (num) => {
    this.data["age"] = num
    return this
  }

  this.weight = (num) => {
    this.data["weight"] = num
    return this
  }

  this.height = (num) => {
    this.data["height"] = num
    return this
  }

  this.gender = (gender) => {
    if(gender == "female") {
      this.data["gender"] = "female"
    } else if(gender == "male") {
      this.data["gender"] = "male"
    }
    return this
  }

  this.pal = (pal) => {
    if(pal === "low" || pal === 1) {
      this.data["pal"] = "low"
    } else if(pal === "moderate" || pal === 2) {
      this.data["pal"] = "moderate"
    } else if(pal === "high" || pal === 3) {
      this.data["pal"] = "high"
    }
    return this
  }

  this.days = (days) => {
    this.data["days"] = days
    return this
  }
}

function Foods() {
  this.data_list = []

  this.food = (food_number) => {
    this.data_list.push({
      "number": food_number
    })
    return this
  }

  this.f = (food_number) => {
    return this.food(food_number)
  }

  // data_listの末尾にある要素のweightを設定する
  // すでに要素にweightが設定されている場合は、新しい要素を引数のweightでもう一つ作成する
  this.weight = (weight) => {
    if(Array.isArray(weight)) {
      for(let w of weight) {
        this.weight(w)
      }
      return null
    }

    if(this.data_list[this.data_list.length - 1]["weight"] !== undefined) {
      let food_number = this.data_list[this.data_list.length - 1]["number"];
      this.data_list.push({
        "number": food_number,
        "weight": weight
      })
    } else {
      this.data_list[this.data_list.length - 1]["weight"] = weight
    }
    return this
  }

  this.w = (num) => {
    this.weight(num)
    return this
  }

  // data_listの末尾にある要素のpriceを設定する
  this.price = (price) => {
    this.data_list[this.data_list.length - 1]["price"] = price
    return this
  }

  this.p = (price) => {
    return this.price(price)
  }

  this.price_per_gram = (price_per_gram) => {
    let weight = this.data_list[this.data_list.length - 1]["weight"]
    if(weight === undefined){
      return this
    }
    
    this.data_list[this.data_list.length - 1]["price"] = price_per_gram * weight
    return this
  }

  this.ppg = (price_per_gram) => {
    return this.price_per_gram(price_per_gram)
  }

  this.changeWeight = (rate) => {
    let data_list = []
    for(let data of this.data_list){
      let new_data = {
        "number": data.number
      }
      
      if(data["weight"] != undefined){
        new_data["weight"] = data["weight"] * rate
      }
      if(data["price"] != undefined){
        new_data["price"] = data["price"] * rate
      }
      
      data_list.push(new_data)
    }

    let foods = new Foods()
    foods.data_list = data_list
    
    return foods
  }
}

module.exports = {
  "Command": Command,
  "Json": Json,
  "Body": Body,
  "Foods": Foods
}

