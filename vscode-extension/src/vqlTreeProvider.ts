import * as vscode from 'vscode';
import * as fs from 'fs';
import * as path from 'path';

export class VQLTreeProvider implements vscode.TreeDataProvider<VQLTreeItem> {
    private _onDidChangeTreeData: vscode.EventEmitter<VQLTreeItem | undefined | null | void> = new vscode.EventEmitter<VQLTreeItem | undefined | null | void>();
    readonly onDidChangeTreeData: vscode.Event<VQLTreeItem | undefined | null | void> = this._onDidChangeTreeData.event;

    private decorationsEnabled: boolean;

    constructor(decorationsEnabled: boolean) {
        this.decorationsEnabled = decorationsEnabled;
    }

    refresh(decorationsEnabled?: boolean): void {
        if (decorationsEnabled !== undefined) {
            this.decorationsEnabled = decorationsEnabled;
        }
        this._onDidChangeTreeData.fire();
    }

    getTreeItem(element: VQLTreeItem): vscode.TreeItem {
        return element;
    }

    private isVQLInitialized(): boolean {
        const workspaceFolders = vscode.workspace.workspaceFolders;
        if (!workspaceFolders) {
            return false;
        }
        
        const workspaceRoot = workspaceFolders[0].uri.fsPath;
        const vqlPath = path.join(workspaceRoot, 'VQL', 'vql_storage.json');
        const vqlPathLower = path.join(workspaceRoot, 'vql', 'vql_storage.json');
        
        return fs.existsSync(vqlPath) || fs.existsSync(vqlPathLower);
    }

    getChildren(element?: VQLTreeItem): Thenable<VQLTreeItem[]> {
        if (!element) {
            const isInitialized = this.isVQLInitialized();
            
            if (!isInitialized) {
                // Show only setup when VQL is not initialized
                return Promise.resolve([
                    new VQLTreeItem(
                        '🚀 Setup VQL',
                        'Initialize VQL in this workspace',
                        'vql.setup',
                        vscode.TreeItemCollapsibleState.None
                    )
                ]);
            }
            
            // Show full menu when VQL is initialized
            return Promise.resolve([
                new VQLTreeItem(
                    `Icons: ${this.decorationsEnabled ? '🟩 On' : '🟥 Off'}`,
                    'Toggle file decorations',
                    'vql.toggleDecorations',
                    vscode.TreeItemCollapsibleState.None
                ),
                new VQLTreeItem(
                    '📊 Show Matrix',
                    'View compliance matrix',
                    'vql.showMatrix',
                    vscode.TreeItemCollapsibleState.None
                ),
                new VQLTreeItem(
                    '📝 Show Metadata',
                    'Edit entities, types, and principles',
                    'vql.showMetadata',
                    vscode.TreeItemCollapsibleState.None
                ),
                new VQLTreeItem(
                    '🔄 Refresh',
                    'Refresh VQL data',
                    'vql.refreshDecorations',
                    vscode.TreeItemCollapsibleState.None
                ),
                new VQLTreeItem(
                    '📖 Load Principles',
                    'Load principles from markdown',
                    'vql.loadPrinciples',
                    vscode.TreeItemCollapsibleState.None
                )
            ]);
        }
        return Promise.resolve([]);
    }
}

class VQLTreeItem extends vscode.TreeItem {
    constructor(
        public readonly label: string,
        public readonly description: string,
        public readonly commandId: string,
        public readonly collapsibleState: vscode.TreeItemCollapsibleState
    ) {
        super(label, collapsibleState);
        this.tooltip = this.description;
        this.command = {
            command: commandId,
            title: label,
            arguments: []
        };
    }
}