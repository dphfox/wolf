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
exports.ConfigState = void 0;
const vscode = __importStar(require("vscode"));
const path = __importStar(require("path"));
const fs = __importStar(require("fs/promises"));
const ui = __importStar(require("./ui"));
class ConfigState {
    config;
    exePath = null;
    constructor(editor) {
        const docConfig = vscode.workspace.getConfiguration('wolf', editor.document);
        this.config = {
            exec: {
                path: docConfig.get('exec.path', 'wolf') ?? ""
            }
        };
    }
    async getExePath() {
        let exePath = this.exePath ?? await this.initExePath();
        if (exePath.ok) {
            return exePath.path;
        }
        else {
            throw exePath.err;
        }
    }
    async initExePath() {
        let wolfExe = this.config.exec.path;
        if (wolfExe.trim().length === 0) {
            try {
                wolfExe = await findExecutable("wolf");
            }
            catch (e) {
                ui.errorWolfNotOnPath();
                return { ok: false, err: e };
            }
        }
        try {
            wolfExe = resolveWorkspacePath(wolfExe);
        }
        catch (e) {
            ui.errorManualWolfPathNotResolved(e);
            return { ok: false, err: e };
        }
        try {
            await checkFileExists(wolfExe);
        }
        catch (e) {
            ui.errorManualWolfPathNotExe();
            return { ok: false, err: e };
        }
        return { ok: true, path: wolfExe };
    }
}
exports.ConfigState = ConfigState;
async function findExecutable(exe) {
    const envPath = process.env.PATH || "";
    const envExt = process.env.PATHEXT || "";
    const pathDirs = envPath.replace(/["]+/g, "").split(path.delimiter).filter(Boolean);
    const extensions = envExt.split(";");
    const candidates = pathDirs.flatMap((d) => extensions.map((ext) => path.join(d, exe + ext)));
    try {
        return await Promise.any(candidates.map(checkFileExists));
    }
    catch (e) {
        throw new Error(`Could not find ${exe} executable on PATH`);
    }
}
async function checkFileExists(filePath) {
    if ((await fs.stat(filePath)).isFile()) {
        return filePath;
    }
    throw new Error("Not a file");
}
function resolveWorkspacePath(workspacePath) {
    if (path.isAbsolute(workspacePath))
        return workspacePath;
    const workspace_folder = vscode.workspace.workspaceFolders?.[0];
    if (!workspace_folder)
        throw new Error("Relative paths aren't supported outside of folders or workspaces.");
    return path.resolve(workspace_folder.uri.fsPath, workspacePath);
}
//# sourceMappingURL=config.js.map