import * as vscode from 'vscode';

export class VQLTreeProvider implements vscode.TreeDataProvider<VQLTreeItem> {
    private _onDidChangeTreeData: vscode.EventEmitter<VQLTreeItem | undefined | null | void> = new vscode.EventEmitter<VQLTreeItem | undefined | null | void>();
    readonly onDidChangeTreeData: vscode.Event<VQLTreeItem | undefined | null | void> = this._onDidChangeTreeData.event;

    private decorationsEnabled: boolean;

    constructor(decorationsEnabled: boolean) {
        this.decorationsEnabled = decorationsEnabled;
    }

    refresh(decorationsEnabled: boolean): void {
        this.decorationsEnabled = decorationsEnabled;
        this._onDidChangeTreeData.fire();
    }

    getTreeItem(element: VQLTreeItem): vscode.TreeItem {
        return element;
    }

    getChildren(element?: VQLTreeItem): Thenable<VQLTreeItem[]> {
        if (!element) {
            // Root level items
            return Promise.resolve([
                new VQLTreeItem(
                    `Icons: ${this.decorationsEnabled ? '🟩 On' : '🟥 Off'}`,
                    'Toggle file decorations',
                    'vql.toggleDecorations',
                    vscode.TreeItemCollapsibleState.None
                ),
                new VQLTreeItem(
                    'Show Matrix',
                    'View compliance matrix',
                    'vql.showMatrix',
                    vscode.TreeItemCollapsibleState.None
                ),
                new VQLTreeItem(
                    'Show Metadata',
                    'Edit entities, types, and principles',
                    'vql.showMetadata',
                    vscode.TreeItemCollapsibleState.None
                ),
                new VQLTreeItem(
                    'Setup VQL',
                    'Initialize VQL in workspace',
                    'vql.setup',
                    vscode.TreeItemCollapsibleState.None
                ),
                new VQLTreeItem(
                    'Load Principles',
                    'Load principles from markdown file',
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