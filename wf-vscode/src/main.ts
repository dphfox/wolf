import * as vscode from 'vscode';
import { highlightTokenStream } from './token';
import { ConfigState } from './config';
import * as ui from './ui'

export const output = vscode.window.createOutputChannel('Wolf for VSCode');
const decorationTypes: Record<string, vscode.TextEditorDecorationType> = {
	"unexpected": vscode.window.createTextEditorDecorationType({
		before: {
			contentText: "<unexpected>",
			color: "red"
		},
		after: {
			contentText: "</unexpected>",
			color: "red"
		},
		cursor: "not-allowed",
		backgroundColor: "red"
	}), 
	"whitespace": vscode.window.createTextEditorDecorationType({
		cursor: "crosshair",
		backgroundColor: "#8881"
	}), 
	"comment": newDecorationType(4, false), 
	"name": newDecorationType(8, false), 
	"string": newDecorationType(12, false), 
	
	"throw": newDecorationType(1, true), 
	"catch": newDecorationType(1, true),
	"loop": newDecorationType(1, true), 
	"then": newDecorationType(1, true), 
	"else": newDecorationType(1, true), 
	"and": newDecorationType(1, true), 
	"let": newDecorationType(1, true), 
	"or": newDecorationType(1, true), 
	"fn": newDecorationType(1, true),
	"if": newDecorationType(1, true),
	"ellipsis": newDecorationType(4, true),
	"double_slash": newDecorationType(4, true),
	"slash_caret": newDecorationType(4, true),
	"bang_equal": newDecorationType(4, true),
	"less_equal": newDecorationType(4, true),
	"more_equal": newDecorationType(4, true),
	"thin_arrow": newDecorationType(7, true),
	"fat_arrow": newDecorationType(7, true),
	"open_bracket": newDecorationType(9, true),
	"close_bracket": newDecorationType(9, true),
	"open_paren": newDecorationType(10, true),
	"close_paren": newDecorationType(10, true),
	"comma": newDecorationType(4, true),
	"dot": newDecorationType(4, true),
	"colon": newDecorationType(4, true),
	"plus": newDecorationType(4, true),
	"minus": newDecorationType(4, true),
	"asterisk": newDecorationType(4, true),
	"slash": newDecorationType(4, true),
	"caret": newDecorationType(4, true),
	"equal": newDecorationType(4, true),
	"bang": newDecorationType(4, true),
	"less": newDecorationType(4, true),
	"more": newDecorationType(4, true),
	"end_line": vscode.window.createTextEditorDecorationType({
		before: {
			contentText: "↩",
			color: "#fff8"
		},
		cursor: "crosshair"
	})
}

function newDecorationType(index: number, filled: boolean): vscode.TextEditorDecorationType {
	if (filled) {
		return vscode.window.createTextEditorDecorationType({
			cursor: 'crosshair',
			borderRadius: '8px',
			backgroundColor: { id: `wolf.debug.${index}.20p` },
		});
	} else {
		return vscode.window.createTextEditorDecorationType({
			cursor: 'crosshair',
			borderRadius: '8px',
			borderWidth: '2px',
			borderStyle: 'solid',
			borderColor: { id: `wolf.debug.${index}.20p` },
		});
	}
}

class EditorState {
	private activeEditor: vscode.TextEditor;
	private config: ConfigState;
	private updateDecorationsTimeout: NodeJS.Timeout | null = null;

	constructor(activeEditor: vscode.TextEditor) {
		this.activeEditor = activeEditor;
		this.config = new ConfigState(activeEditor);
		this.updateDecorations();
	}

	public onChangeTextDocument(event: vscode.TextDocumentChangeEvent) {
		if (event.document == this.activeEditor.document) this.updateDecorations()
	}

	private shouldDecorate() {
		return this.activeEditor.document.languageId === "wolf";
	}

	private updateDecorations() {
		if (!this.shouldDecorate()) { return; }
		if (this.updateDecorationsTimeout != null) {
			clearTimeout(this.updateDecorationsTimeout);
			this.updateDecorationsTimeout = null;
		}
		this.updateDecorationsTimeout = setTimeout(async () => {
			if (!this.shouldDecorate()) { return; }
			try {
				const decorationsForTypes = await highlightTokenStream(this.activeEditor.document, this.config);
				for (let ty in decorationTypes) {
					let decorationType = decorationTypes[ty];
					let decorations = decorationsForTypes[ty];
					if (decorations == null) decorations = [];
					this.activeEditor.setDecorations(decorationType, decorations);
				}
			} catch (e) {
				ui.errorFailedToDecorate(e);
				output.appendLine("Failed to highlight tokens: " + e);
			}
		}, 200);
	}
}

let context: vscode.ExtensionContext | null = null;
let subscriptions: vscode.Disposable[] = [];
export function reload() {
	if (context == null) return;
	unload();

	let editorCache: EditorState | null = vscode.window.activeTextEditor == null ? null : new EditorState(vscode.window.activeTextEditor);
	vscode.window.onDidChangeActiveTextEditor(editor => {
		editorCache = editor == null ? null : new EditorState(editor);
	}, null, subscriptions);
	vscode.workspace.onDidChangeTextDocument(event => {
		if (editorCache != null) editorCache.onChangeTextDocument(event)
	}, null, subscriptions);
}

function unload() {
	for (let disposable of subscriptions) {
		disposable.dispose();
	}
	subscriptions = [];
}

export function activate(theContext: vscode.ExtensionContext) {
	context = theContext;
	context.subscriptions.push({ dispose: unload })
	reload()
}