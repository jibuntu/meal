let meal = require("./meal")

let json = new meal.Json()
let body = new meal.Body()
let foods = new meal.Foods()

body
  .age(20)
  .weight(50)
  .height(160)
  .gender("male")
  .pal(1)

foods
  .food("01001").w(200)
  .food("01002").w([10, 20, 30])
  .food("01003").w(200).w(300)

json
  .body(body)
  .foods(foods)
  .name_list("摂取基準")
  .comb([3, 4, 5])
  .path("./test.json")
  .write()