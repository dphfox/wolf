import * as vscode from 'vscode';

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

export function tokeniseDocument(document: vscode.TextDocument): Token[] {
	return [];
}

export function highlightTokenStream(document: vscode.TextDocument, tokens: Token[]): vscode.DecorationOptions[] {
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
				currentByte += 4;  // Surrogate pair = 4 UTF-8 bytes
				currentUtf16 += 2;
			} else if (code < 0x80) {
				currentByte += 1;  // ASCII
				currentUtf16 += 1;
			} else if (code < 0x800) {
				currentByte += 2;  // 2-byte UTF-8
				currentUtf16 += 1;
			} else {
				currentByte += 3;  // 3-byte UTF-8
				currentUtf16 += 1;
			}
		}
		const endPos = document.positionAt(currentUtf16);
		const decoration = { range: new vscode.Range(startPos, endPos), hoverMessage: token.ty }
		highlights.push(decoration);
		startPos = endPos;
	}
	return highlights;
}