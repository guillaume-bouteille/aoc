
// node tonton.js

const fs = require('fs');

function parseInputs(file_path) {
    const data = fs.readFileSync(file_path, 'utf8');
    return data.split(/\r?\n/)
}

function updatePwd(tree, cursor) {
    var pwd = tree;
    cursor.forEach(dir => {
        pwd = pwd[dir];
    })
    return pwd;
}

function populateTree(inputs) {
    var tree = {};
    var cursor = [];
    var pwd = tree;
    inputs.forEach(line => {
        res = line.match(/\$ cd (?<dir>.*)/)
        res_ls = line.match(/\$ ls/)
        if(res) {
            if(res.groups.dir == "/"){
                cursor = [];
            } else if (res.groups.dir == "..") {
                cursor = cursor.slice(0, -1);
            } else {
                cursor.push(res.groups.dir);
            }
            pwd = updatePwd(tree, cursor);
        } else if(res_ls) {
               // OSEF
        } else {
            res_dir = line.match(/dir (?<dir>.*)/)
            res_file = line.match(/(?<size>[0-9]+) (?<file>.*)/)
            if(res_dir) {
                pwd[res_dir.groups.dir] = {};
            } else if(res_file) {
                pwd[res_file.groups.file] = parseInt(res_file.groups.size, 10);
            } else {
                console.log("Error on line: "+line)
            }
        }
    })
    return tree
}

function getTotalSize(dir) {
    var sum = 0
    Object.keys(dir).forEach(k => {
        if(Number.isInteger(dir[k])) {
            sum += dir[k]
        } else {
            sum += getTotalSize(dir[k])
        }
    })
    return sum
}

function getDirsOfSizesUnder100000(dir) {
    var res = []
    Object.keys(dir).forEach(k => {
        if(Number.isInteger(dir[k])) {
            // Do nothing
        } else {
            if (getTotalSize(dir[k]) <= 100000) {
                res.push(dir[k])
            }
            res = res.concat(getDirsOfSizesUnder100000(dir[k]));
        }
    })
    return res
}


function getDirsOfSizesOver(dir, min_size) {
    var res = []
    Object.keys(dir).forEach(k => {
        if(Number.isInteger(dir[k])) {
            // Do nothing
        } else {
            if (getTotalSize(dir[k]) >= min_size) {
                res.push(dir[k])
            }
            res = res.concat(getDirsOfSizesOver(dir[k], min_size));
        }
    })
    return res
}


function exo1(inputs) {
    var tree = populateTree(inputs);
    var answ1 = 0
    var res = getDirsOfSizesUnder100000(tree)

    res.forEach(d => {
        answ1 += getTotalSize(d)
    })
    return answ1
}

function exo2(inputs) {
    var tree = populateTree(inputs);
    var additional_space_required = 30000000 - (70000000 - getTotalSize(tree))
    var res = getDirsOfSizesOver(tree, additional_space_required)
    // il y a surement un moyen plus elegant de faire un min, avec des lambdas, et tout, mais bon...
    var answ2 = 100000000000000
    res.forEach(d => {
        s = getTotalSize(d)
        if( s < answ2) {
            answ2 = s
        }
    })
    return answ2
}


inputs = parseInputs("input.txt")

answ1 = exo1(inputs)
console.log("The first answer is "+answ1)
answ2 = exo2(inputs)
console.log("The second answer is "+answ2)
