let meal = require("./meal")

let command = new meal.Command()
let body = new meal.Body()

body
  .age(19)
  .weight(53)
  .height(160)
  .gender("male")
  .pal(1)
  .days(2)

let udf = new meal.UserDefinitionFoods()
    .food("1").w(100)
    .food("2").w(0).data({
      "食品名": "2番目"
    })
    .food("3").w(200).data({
      "食品名": "3番目",
      "エネルギー": "100"
    })

let foods = new meal.Foods()
  .food("01001").w(200)
  .food("01002").w([10, 20, 30])
  .food("01003").w(200).w(300)
  .food("01004").w(20).p(30)
  .food("01004").w(50).ppg(1)
  .food("01004").w(50).ppg(3)
  .food("02001").w(50).ir(true)
  .food("u1")
  .food("u2").c("テスト")
  .food("u3").c("テスト")

let json = new meal.Json()
  .body(body)
  .foods(foods)
  .userDefinitionFoods(udf)
  .name_list("摂取基準")
  .comb([3, 4, 5])
  .path("./test.json")
  .write()

foods = foods.changeWeight(0.5)

body = new meal.Body()
  .age(19)
  .weight(53)
  .height(160)
  .gender("male")
  .pal(1)
  .days(1)

json = new meal.Json()
  .body(body)
  .foods(foods)
  .userDefinitionFoods(udf)
  .name_list("摂取基準")
  .comb([3, 4, 5])
  .path("./test_2.json")
  .write()

command.calc(["./test.json", "./test_2.json"])
