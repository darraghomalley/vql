import * as vscode from 'vscode';
import * as fs from 'fs';
import * as path from 'path';
import * as chokidar from 'chokidar';
import { VQLDecorationProvider } from './decorationProvider';

export class VQLStorageWatcher implements vscode.Disposable {
    private watcher: chokidar.FSWatcher | null = null;
    private decorationProvider: VQLDecorationProvider;
    private changeListeners: (() => void)[] = [];

    constructor(decorationProvider: VQLDecorationProvider) {
        this.decorationProvider = decorationProvider;
        this.initializeWatcher();
    }

    public onStorageChanged(listener: () => void): void {
        this.changeListeners.push(listener);
    }

    private initializeWatcher(): void {
        const workspaceFolders = vscode.workspace.workspaceFolders;
        if (!workspaceFolders || workspaceFolders.length === 0) {
            return;
        }

        const workspaceRoot = workspaceFolders[0].uri.fsPath;
        
        // Watch for VQL storage file changes
        const patterns = [
            path.join(workspaceRoot, 'VQL', 'vql_storage.json'),
            path.join(workspaceRoot, 'vql', 'vql_storage.json')
        ];

        // Find which path exists
        let pathToWatch: string | null = null;
        for (const pattern of patterns) {
            if (fs.existsSync(pattern)) {
                pathToWatch = pattern;
                break;
            }
        }

        if (pathToWatch) {
            this.watcher = chokidar.watch(pathToWatch, {
                persistent: true,
                ignoreInitial: true
            });

            this.watcher.on('change', () => {
                console.log('VQL storage changed, refreshing decorations');
                this.decorationProvider.refresh();
                
                // Notify all listeners
                this.changeListeners.forEach(listener => listener());
            });

            this.watcher.on('error', (error) => {
                console.error('VQL storage watcher error:', error);
            });
        }
    }

    dispose(): void {
        if (this.watcher) {
            this.watcher.close();
            this.watcher = null;
        }
    }
}