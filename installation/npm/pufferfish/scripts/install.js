#!/usr/bin/env node
const downloadReleases = require('download-github-release');
const os = require('os');
const path = require('path');
const fs = require('fs');
const yauzl = require('yauzl');
const tar = require('tar-stream');
const zlib = require('zlib');

console.error("Installing Pufferfish 🐡");

let os_info = {};

if (process.platform == "win32") {
    // throw new Error("Windows is not supported yet, please see the other instalation options.")
    os_info = {
        name: "windows",
        exec_path: "\\Windows\\System32\\puf"
    }
} else if (process.platform == "darwin") {
    os_info = {
        name: "darwin",
        exec_path: "/usr/local/bin/puf"
    }
} else if (process.platform == "linux") {
    os_info = {
        name: "linux",
        exec_path: "/usr/local/bin/puf"
    }
    // throw new Error("Linux is not supported yet, please see the other instalation options.")
} else {
    throw new Error(`Unkown operating system: ${process.platform}`)
}

const filter = (asset) => {
    return asset.name.indexOf(os_info.name) >= 0 && asset.name.indexOf("sha256sum") == -1;
};

let output_dir = path.join(os.tmpdir(), "be.jonaseveraert.pufferfish");
if (!fs.existsSync(output_dir)) {
    fs.mkdirSync(output_dir);
}


downloadReleases("jomy10", "pufferfish", output_dir, (rel) => {return true;}, filter)
    .then(() => {
        console.error("Downloaded pufferfish!");

        fs.readdirSync(output_dir).forEach((file) => {
            if (/.*\.tar\.gz/.test(file)) {
                let extract = tar.extract();
                let data = [];

                extract.on('entry', (header, stream, cb) => {
                    if (header.name.indexOf("puf") >= 0) {
                        stream.on('data', (chunk) => {
                            data.push(chunk);
                        });
                    }

                    stream.on("end", () => {
                        cb();
                    });

                    stream.on('error', () => {
                        throw new Error("Couldn't read executable file");
                    });
                    
                    stream.resume();
                });

                extract.on('finish', () => { 
                    if (data.length) {
                        let _data = Buffer.concat(data);
                        fs.writeFile(os_info.exec_path, _data, (err) => {
                            if (err) throw err;
                            fs.chmod(os_info.exec_path, 0o777, (err) => {
                                 if (err) throw err;
                                 console.error("Pufferfish installed!");
                                 console.error("Cleaning up...");
                                 fs.rmdir(output_dir, {recursive: true}, (err) => {
                                    if (err) throw err;
                                 });
                             });
                        });
                    } else {
                        console.error("Error: File was empty");
                    }
                });

                fs.createReadStream(path.join(output_dir, file))
                    .pipe(zlib.createGunzip())
                    .pipe(extract);
                // fs.readFile(path.join(output_dir, file), (err, data) => {
                //     if (err) throw err;
                //     ungzip(data).then((decompressed) => {
                //         fs.writeFile(os_info.exec_path, decompressed, "binary", (err) => {
                //             if (err) throw err;
                //             fs.chmod(os_info.exec_path, 0o777, (err) => {
                //                 if (err) throw err;
                //                 console.log("Pufferfish installed!");
                //             });
                //         })
                //     });
                // });
            } else if (/.*\.zip/.test(file)) {
                yauzl.open(path.join(output_dir, file), {}, (err, zipfile) => {
                    if (err) throw err;
                    zipfile.on("entry", (entry) => {
                        if (/puf.*/.test(entry.filename)) {
                            zipfile.openReadStream(entry, (err, stream) => {
                                console.log("Reading", stream);
                                if (err) throw err;
                                console.log("Writing executable");
                                stream.on("end", () => {
                                    zipfile.readEntry();
                                });
                                stream.pipe(os_info.exec_path);
    
                                console.error("Pufferfish installed.");
                            });
                        } else {
                            zipfile.readEntry();
                        }
                    });
                });
            } else {
                console.error("Unkown file extension for", file);
            }
        });
    })
    .catch((err) => {
        console.error(err.message);
    });