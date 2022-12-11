
// node tata.js

const fs = require('fs');

function parseInputs(file_path) {
    const data = fs.readFileSync(file_path, 'utf8');
    return data.split(/\r?\n/)
}

function getPriority(c) {
    if(c > 'a' && c <= 'z') {
        return 1+c.charCodeAt(0)-'a'.charCodeAt(0)
    } else if(c >= 'A' && c <= 'Z') {
        return 27+c.charCodeAt(0)-'A'.charCodeAt(0)
    } else {
        return 0 // Oups
    }
}

function exo1(inputs) {
    priority = 0
    inputs.forEach(line => {
        compartment_1 = line.substring(0, line.length/2)
        compartment_2 = line.substring(line.length/2, line.length)
        compartment_1 = [...compartment_1].reduce((acc, curr)=> {
            return acc.includes(curr) ? acc : acc + curr;
        }, "")
        compartment_1.split("").forEach(c => {
            if( compartment_2.includes(c) ){
                priority += getPriority(c)
            }
        })
    })
    return priority
}

function exo2(inputs) {
    priority = 0
    for (let i = 0; i < inputs.length/3; i++) { 
        blah = [...inputs[i*3]].reduce((acc, curr)=> {
            return acc.includes(curr) ? acc : acc + curr;
        }, "")
        blah.split("").forEach(c => {
            if( inputs[i*3+1].includes(c) && inputs[i*3+2].includes(c) ){
                priority += getPriority(c)
            }
        })
    }
    return priority
}


inputs = parseInputs("input.txt")

answ1 = exo1(inputs)
console.log("The first answer is "+answ1)
answ2 = exo2(inputs)
console.log("The second answer is "+answ2)
