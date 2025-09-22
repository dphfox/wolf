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
exports.tokeniseDocument = tokeniseDocument;
exports.highlightTokenStream = highlightTokenStream;
const vscode = __importStar(require("vscode"));
function tokeniseDocument(document) {
    return [];
}
function highlightTokenStream(document, tokens) {
    const text = document.getText();
    const highlights = [];
    let startPos = new vscode.Position(0, 0);
    let currentUtf16 = 0;
    let currentByte = 0;
    for (let token of tokens) {
        const endByte = token.span.index + token.span.length;
        while (currentByte < endByte && currentUtf16 < text.length) {
            const code = text.charCodeAt(currentUtf16);
            if (code >= 0xD800 && code <= 0xDBFF) {
                currentByte += 4; // Surrogate pair = 4 UTF-8 bytes
                currentUtf16 += 2;
            }
            else if (code < 0x80) {
                currentByte += 1; // ASCII
                currentUtf16 += 1;
            }
            else if (code < 0x800) {
                currentByte += 2; // 2-byte UTF-8
                currentUtf16 += 1;
            }
            else {
                currentByte += 3; // 3-byte UTF-8
                currentUtf16 += 1;
            }
        }
        const endPos = document.positionAt(currentUtf16);
        const decoration = { range: new vscode.Range(startPos, endPos), hoverMessage: token.ty };
        highlights.push(decoration);
        startPos = endPos;
    }
    return highlights;
}
//# sourceMappingURL=token.js.map