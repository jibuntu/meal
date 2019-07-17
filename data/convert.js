// foods.jsonから不必要なデータを取り去って、価格用のprices.jsonを作成する

const fs = require("fs")
const log = console.log

const path = process.argv[2]
if(path == undefined){
  return log("error: オブションにファイル名を指定してください")
}

let foods = fs.readFileSync("foods.json", "utf8")

let prices = foods.split("\n")
  .filter(l => l != "")
  .map(l => l.split(","))
  .filter(cells => cells[3] != undefined)
  .map(cells => cells.map(cell => cell.replace(/"/g, "").trim()))
  .map(cells => [cells[1], cells[3]])

let jsonData = JSON.stringify({"foods": prices}, null, 4)
  .replace(/\[.*?\n.*?"/g, '["')
  .replace(/",.*?\n.*?"/g, '", "')
  .replace(/"\n.*?]/g, '"]')

fs.writeFileSync(path, jsonData)