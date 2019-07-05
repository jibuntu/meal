from meal import Foods, Body, Json

body = Body()
body.age(19)
body.weight(53.0)
body.height(160.0)
body.gender("male")
body.pal(1)

foods = Foods()
foods.food("04047").w(40*4)
foods.food("01088").w(300)

json = Json()
json.name_list("摂取基準")
json.foods(foods)
json.body(body)

json.write("test.json")