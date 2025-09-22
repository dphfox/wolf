import * as vscode from 'vscode';
import * as path from 'path';
import * as fs from 'fs/promises';
import * as ui from './ui';

type Configuration = {
	exec: {
		path: string,
		safetyTimeout: number
	}
}

type ExePath = { ok: true, path: string } | { ok: false, err: unknown };

export class ConfigState {
	private config: Configuration;
	private exePath: ExePath | null = null;

	constructor(editor: vscode.TextEditor) {
		const docConfig = vscode.workspace.getConfiguration('wolf', editor.document);
		this.config = {
			exec: {
				path: docConfig.get('exec.path', ''),
				safetyTimeout: docConfig.get('exec.safetyTimeout', 1000)
			}
		}
	}

	public async getExePath(): Promise<string> {
		let exePath = this.exePath ?? await this.initExePath();
		if (exePath.ok) {
			return exePath.path;
		} else {
			throw exePath.err;
		}
	}

	public getSafetyTimeout(): number {
		return this.config.exec.safetyTimeout;
	}

	private async initExePath(): Promise<ExePath> {
		let wolfExe = this.config.exec.path;
		if (wolfExe.trim().length === 0) {
			try {
				wolfExe = await findExecutable("wolf");
			} catch (e) {
				ui.errorWolfNotOnPath();
				return { ok: false, err: e }
			}
		}
		try {
			wolfExe = resolveWorkspacePath(wolfExe);
		} catch(e) {
			ui.errorManualWolfPathNotResolved(e);
			return { ok: false, err: e }
		}
		try {
			await checkFileExists(wolfExe);
		} catch (e) {
			ui.errorManualWolfPathNotExe();
			return { ok: false, err: e }
		}
		return { ok: true, path: wolfExe }
	}
}

async function findExecutable(exe: string): Promise<string> {
	const envPath = process.env.PATH || "";
	const envExt = process.env.PATHEXT || "";
	const pathDirs = envPath.replace(/["]+/g, "").split(path.delimiter).filter(Boolean);
	const extensions = envExt.split(";");
	const candidates = pathDirs.flatMap((d) => extensions.map((ext) => path.join(d, exe + ext)));
	try {
		return await Promise.any(candidates.map(checkFileExists));
	} catch (e) {
		throw new Error(`Could not find ${exe} executable on PATH`);
	}
}

async function checkFileExists(filePath: string): Promise<string> {
	if ((await fs.stat(filePath)).isFile()) {
		return filePath;
	}
	throw new Error("Not a file");
}

function resolveWorkspacePath(workspacePath: string): string {
    if (path.isAbsolute(workspacePath)) return workspacePath;
    const workspace_folder = vscode.workspace.workspaceFolders?.[0];
    if (!workspace_folder) throw new Error("Relative paths aren't supported outside of folders or workspaces.");
    return path.resolve(workspace_folder.uri.fsPath, workspacePath);
}
