#!/usr/bin/env node
var shell = require("shelljs");
let arguments = ""
let scriptPath = ""
process.argv.forEach((val, index, array) => {
    // console.log(index, val);
    if (index != 0 && index != 1) {
        arguments += val + " "
    } else if (index == 1) {
        scriptPath = val;
    }
});
shell.exec(`echo $(dirname ${scriptPath})`)
shell.exec(`cd $(dirname ${scriptPath}) && cd ../gem/bin && ./puf ${arguments}`);
