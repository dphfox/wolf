import * as vscode from 'vscode';
import { reload } from './main';

export function errorWolfNotOnPath() {
	vscode.window.showErrorMessage(
		"Failed to find the Wolf executable. \
		Ensure `wolf` is on your PATH or `wolf.exec.path` is set to a valid executable.",
		"Reload"
	).then((action: string | undefined) => {
		if (action === "Reload") reload();
	});
}

export function errorManualWolfPathNotResolved(e: unknown) {
	vscode.window.showErrorMessage(
		"The Wolf executable from `wolf.exec.path` couldn't be resolved: " + e,
		"Reload"
	).then((action: string | undefined) => {
		if (action === "Reload") reload();
	});
}

export function errorManualWolfPathNotExe() {
	vscode.window.showErrorMessage(
		"The Wolf executable from `wolf.exec.path` doesn't point at an executable. \
		Set it to a valid executable or leave it blank to infer from the PATH.",
		"Reload"
	).then((action: string | undefined) => {
		if (action === "Reload") reload();
	});
}