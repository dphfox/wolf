import { spawn, SpawnOptions } from 'child_process';
import { ConfigState } from './config';

async function spawnWithStdin(
    command: string, 
    args: string[], 
    stdinData: string, 
    options: SpawnOptions,
	safetyTimeout: number
): Promise<string> {
    return new Promise<string>((resolve, reject) => {
        const child = spawn(command, args, options);

		let timeout: NodeJS.Timeout | null = setTimeout(() => {
			if (!child.killed) child.kill();
			reject(new Error("The Wolf executable took too long - safety timout reached."));
		}, safetyTimeout);

        let stdout = '';
        let stderr = '';
        child.stdout?.on('data', (data) => {
            stdout += data.toString();
        });
        child.stderr?.on('data', (data) => {
            stderr += data.toString();
        });
        child.on('error', (error) => {
			if (timeout != null) {
				clearTimeout(timeout);
				timeout = null;
			}
            reject(error);
        });
        child.on('close', (code) => {
			if (timeout != null) {
				clearTimeout(timeout);
				timeout = null;
			}
            if (code !== 0) {
                reject(new Error(`Process exited with code ${code}: ${stderr}`));
            } else {
                resolve(stdout);
            }
        });

        child.stdin?.write(stdinData);
        child.stdin?.end();
    });
}

export async function callExe(
	config: ConfigState,
	args: string[],
    stdinData: string
): Promise<string> {
	return await spawnWithStdin(await config.getExePath(), args, stdinData, { "windowsHide": true }, config.getSafetyTimeout());
}