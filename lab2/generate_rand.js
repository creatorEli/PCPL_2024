let strToFile = "";

for (let i = 0; i < 100000; i++) { // массив на 100_000 элементов. Могут повторяться
    strToFile += Math.round(Math.random()*200_000).toString();
    if(i!=100000-1){strToFile += "\n";}
}

const fs = require("fs");

fs.writeFile("rand.txt", strToFile, function(error){
    if(error) {return console.log(error)};
});

console.log("file is written successfully!");