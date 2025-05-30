import * as vscode from 'vscode';
import * as fs from 'fs';
import * as path from 'path';
import { VQLDecorationProvider } from './decorationProvider';

export class VQLStorageWatcher implements vscode.Disposable {
    private watcher: vscode.FileSystemWatcher | null = null;
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
            // Use VS Code's built-in file watcher
            const pattern = new vscode.RelativePattern(workspaceRoot, path.relative(workspaceRoot, pathToWatch));
            this.watcher = vscode.workspace.createFileSystemWatcher(pattern);

            this.watcher.onDidChange(() => {
                console.log('VQL storage changed, refreshing decorations');
                // Add a small delay to ensure file write is complete
                setTimeout(() => {
                    this.decorationProvider.refresh();
                    
                    // Notify all listeners
                    this.changeListeners.forEach(listener => listener());
                }, 100);
            });

            // Also watch for creation in case file is deleted and recreated
            this.watcher.onDidCreate(() => {
                console.log('VQL storage created, refreshing decorations');
                // Add a small delay to ensure file write is complete
                setTimeout(() => {
                    this.decorationProvider.refresh();
                    
                    // Notify all listeners
                    this.changeListeners.forEach(listener => listener());
                }, 100);
            });

            console.log(`Watching VQL storage at: ${pathToWatch}`);
        }
    }

    dispose(): void {
        if (this.watcher) {
            this.watcher.dispose();
            this.watcher = null;
        }
    }
}