#!/usr/bin/env node

console.log("Installing Pufferfish 🐡");

if (process.platform == "win32") {
    
} else if (process.platform == "darwin") {

} else if (process.platform == "linux") {

} else {
    throw new Error(`Unkown operating system: ${process.platform}`)
}