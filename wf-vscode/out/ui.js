"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || (function () {
    var ownKeys = function(o) {
        ownKeys = Object.getOwnPropertyNames || function (o) {
            var ar = [];
            for (var k in o) if (Object.prototype.hasOwnProperty.call(o, k)) ar[ar.length] = k;
            return ar;
        };
        return ownKeys(o);
    };
    return function (mod) {
        if (mod && mod.__esModule) return mod;
        var result = {};
        if (mod != null) for (var k = ownKeys(mod), i = 0; i < k.length; i++) if (k[i] !== "default") __createBinding(result, mod, k[i]);
        __setModuleDefault(result, mod);
        return result;
    };
})();
Object.defineProperty(exports, "__esModule", { value: true });
exports.errorWolfNotOnPath = errorWolfNotOnPath;
exports.errorManualWolfPathNotResolved = errorManualWolfPathNotResolved;
exports.errorManualWolfPathNotExe = errorManualWolfPathNotExe;
const vscode = __importStar(require("vscode"));
const main_1 = require("./main");
function errorWolfNotOnPath() {
    vscode.window.showErrorMessage("Failed to find the Wolf executable. \
		Ensure `wolf` is on your PATH or `wolf.exec.path` is set to a valid executable.", "Reload").then((action) => {
        if (action === "Reload")
            (0, main_1.reload)();
    });
}
function errorManualWolfPathNotResolved(e) {
    vscode.window.showErrorMessage("The Wolf executable from `wolf.exec.path` couldn't be resolved: " + e, "Reload").then((action) => {
        if (action === "Reload")
            (0, main_1.reload)();
    });
}
function errorManualWolfPathNotExe() {
    vscode.window.showErrorMessage("The Wolf executable from `wolf.exec.path` doesn't point at an executable. \
		Set it to a valid executable or leave it blank to infer from the PATH.", "Reload").then((action) => {
        if (action === "Reload")
            (0, main_1.reload)();
    });
}
//# sourceMappingURL=ui.js.map