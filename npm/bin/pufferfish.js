#!/usr/bin/env node
var shell = require("shelljs");
let arguments = ""
process.argv.forEach((val, index, array) => {
    console.log(index, val);
    if (index != 0 && index != 1) {
        arguments += val + " "
    }
});
shell.exec(`$(dirname "$1")/../gem/bin/puf ${arguments}`);
