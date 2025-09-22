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
exports.reload = reload;
exports.activate = activate;
const vscode = __importStar(require("vscode"));
const token_1 = require("./token");
const config_1 = require("./config");
const output = vscode.window.createOutputChannel('Wolf for VSCode');
const decorationType = vscode.window.createTextEditorDecorationType({
    borderRadius: '8px',
    cursor: 'crosshair',
    backgroundColor: { id: 'wolf.debug.1.20p' },
});
class EditorState {
    activeEditor;
    config;
    updateDecorationsTimeout = null;
    constructor(activeEditor) {
        this.activeEditor = activeEditor;
        this.config = new config_1.ConfigState(activeEditor);
        this.updateDecorations();
    }
    onChangeTextDocument(event) {
        if (event.document == this.activeEditor.document)
            this.updateDecorations();
    }
    shouldDecorate() {
        return this.activeEditor.document.languageId === "wolf";
    }
    updateDecorations() {
        if (!this.shouldDecorate()) {
            return;
        }
        if (this.updateDecorationsTimeout != null) {
            clearTimeout(this.updateDecorationsTimeout);
            this.updateDecorationsTimeout = null;
        }
        this.updateDecorationsTimeout = setTimeout(async () => {
            if (!this.shouldDecorate()) {
                return;
            }
            try {
                const decorations = (0, token_1.highlightTokenStream)(this.activeEditor.document, []);
                this.activeEditor.setDecorations(decorationType, decorations);
            }
            catch (e) {
                output.appendLine;
            }
        }, 200);
    }
}
let context = null;
let subscriptions = [];
function reload() {
    if (context == null)
        return;
    unload();
    let editorCache = vscode.window.activeTextEditor == null ? null : new EditorState(vscode.window.activeTextEditor);
    vscode.window.onDidChangeActiveTextEditor(editor => {
        editorCache = editor == null ? null : new EditorState(editor);
    }, null, subscriptions);
    vscode.workspace.onDidChangeTextDocument(event => {
        if (editorCache != null)
            editorCache.onChangeTextDocument(event);
    }, null, subscriptions);
}
function unload() {
    for (let disposable of subscriptions) {
        disposable.dispose();
    }
    subscriptions = [];
}
function activate(theContext) {
    context = theContext;
    context.subscriptions.push({ dispose: unload });
    reload();
}
//# sourceMappingURL=main.js.map