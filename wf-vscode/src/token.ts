import * as vscode from 'vscode';
import * as child_process from 'child_process';
import { promisify } from 'util';
import { ConfigState } from './config';
import { output } from './main';
import { callExe } from './cli';

const execFile = promisify(child_process.execFile);

export type TokenType = 
	| "unexpected" 

	| "whitespace" 
	| "comment" 
	| "name" 
	| "string" 
	
	| "loop" 
	| "and" 
	| "let" 
	| "or" 
	| "fn"

	| "ellipsis"
	| "double_slash"
	| "bang_equal"
	| "less_equal"
	| "more_equal"
	| "thin_arrow"
	| "fat_arrow"
	| "open_bracket"
	| "close_bracket"
	| "open_paren"
	| "close_paren"
	| "comma"
	| "dot"
	| "colon"
	| "plus"
	| "minus"
	| "asterisk"
	| "slash"
	| "caret"
	| "equal"
	| "bang"
	| "less"
	| "more"
	| "end_line"

export type Token = {
	ty: TokenType,
	span: {
		// These are ASCII byte offsets.
		index: number,
		length: number
	}
}

export async function tokeniseDocument(document: vscode.TextDocument, config: ConfigState): Promise<Token[]> {
	output.appendLine("Calling for tokens...");
	const stdout = await callExe(config, ["tokenise"], document.getText());
	let tokens = [];
	try {
		let startIndex = 0;
		let currentIndex = 0;
		let parts = [];
		while (currentIndex < stdout.length) {
			const char = stdout.charAt(currentIndex);
			if (char == ",") {
				parts.push(stdout.substring(startIndex, currentIndex));
				startIndex = currentIndex + 1;
			} else if (char == ";") {
				parts.push(stdout.substring(startIndex, currentIndex));
				startIndex = currentIndex + 1;
				if (parts.length != 3) throw new Error("Unexpected number of token parts");
				let index = parseInt(parts[0]);
				let length = parseInt(parts[1]);
				if (isNaN(index) || isNaN(length)) throw new Error("Unexpected token start/length (not integer)");
				let ty = parts[2] as TokenType;
				tokens.push({ty: ty, span: {index: index, length: length}});
				parts = [];
			}
			currentIndex += 1;
		}
	} catch (e) {
		throw new Error("Failed to parse stdout: " + e);
	}
	output.appendLine("Found " + tokens.length + " tokens.");
	return tokens;
}

export async function highlightTokenStream(document: vscode.TextDocument, config: ConfigState): Promise<Record<string, vscode.DecorationOptions[]>> {
	const text = document.getText();
	const highlights: Record<string, vscode.DecorationOptions[]> = {};
	let startUtf16 = 0;
	let currentUtf16 = 0;
	let currentByte = 0;
	for (let token of await tokeniseDocument(document, config)) {
		const endByte = token.span.index + token.span.length;
		output.appendLine("token is  " + token.ty + " from " + token.span.index + " for " + token.span.length );
		while (currentByte < endByte && currentUtf16 < text.length) {
			currentByte += 1;
			currentUtf16 += 1;
		}
		const startPos = document.positionAt(startUtf16);
		const endPos = document.positionAt(currentUtf16);
		const decoration: vscode.DecorationOptions = { range: new vscode.Range(startPos, endPos), hoverMessage: token.ty };
		output.appendLine("Highlighting from " + startUtf16 + " to " + currentUtf16);
		if (!highlights[token.ty]) {
			highlights[token.ty] = [];
		}
		highlights[token.ty].push(decoration);
		startUtf16 = currentUtf16;
	}
	return highlights;
}