import __wbg_init, { fileLoaded, setPanicHook } from "./pkg/rjvm_web.js";

await __wbg_init();

setPanicHook();

self.appendText = function(text) {
    self.postMessage({
        "type": "textOutput",
        "data": text,
    });
}

let currentFileName = null;
let currentFileData = null;

self.addEventListener("message", function(e) {
    switch (e.data.type) {
        case "fileUpload":
            currentFileName = e.data.fileName;
            currentFileData = e.data.fileData;

            break;

        case "runFile":
            let args = e.data.args;
            if (currentFileName == null || currentFileData == null) {
                throw new Error("Called \"runFile\" with null name or data");
            }

            fileLoaded(currentFileName, currentFileData, args);
            break;
    }
});
