import * as vscode from 'vscode';
import { highlightTokenStream } from './token';
import { ConfigState } from './config';

const output = vscode.window.createOutputChannel('Wolf for VSCode');
const decorationType = vscode.window.createTextEditorDecorationType({
	borderRadius: '8px',
	cursor: 'crosshair',
	backgroundColor: { id: 'wolf.debug.1.20p' },
});

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
				
				const decorations = highlightTokenStream(this.activeEditor.document, []);
				this.activeEditor.setDecorations(decorationType, decorations);
			} catch (e) {
				output.appendLine
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