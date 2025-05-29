import * as vscode from 'vscode';
import * as fs from 'fs';
import * as path from 'path';
import { exec } from 'child_process';
import { promisify } from 'util';
import { VQLDecorationProvider } from './decorationProvider';
import { VQLStorageWatcher } from './storageWatcher';
import { VQLMatrixViewProvider } from './matrixViewProvider';
import { VQLMetadataProvider } from './vqlMetadataProvider';
import { VQLTreeProvider } from './vqlTreeProvider';

const execAsync = promisify(exec);

let decorationProvider: VQLDecorationProvider;
let storageWatcher: VQLStorageWatcher;
let matrixViewProvider: VQLMatrixViewProvider;
let metadataProvider: VQLMetadataProvider;
let treeProvider: VQLTreeProvider;
let decorationsEnabled = true;
let decorationProviderDisposable: vscode.Disposable | undefined;

export function activate(context: vscode.ExtensionContext) {
    console.log('VQL extension is now active');

    // Create decoration provider
    decorationProvider = new VQLDecorationProvider();
    
    // Register the decoration provider
    decorationProviderDisposable = vscode.window.registerFileDecorationProvider(decorationProvider);
    context.subscriptions.push(decorationProviderDisposable);

    // Create storage watcher
    storageWatcher = new VQLStorageWatcher(decorationProvider);
    context.subscriptions.push(storageWatcher);

    // Create tree provider for activity bar
    treeProvider = new VQLTreeProvider(decorationsEnabled);
    vscode.window.registerTreeDataProvider('vqlActions', treeProvider);

    // Register toggle command
    const toggleCommand = vscode.commands.registerCommand('vql.toggleDecorations', () => {
        decorationsEnabled = !decorationsEnabled;
        
        if (decorationsEnabled) {
            // Re-enable decorations
            decorationProvider.setEnabled(true);
            decorationProvider.refresh();
            vscode.window.showInformationMessage('VQL Icons enabled');
        } else {
            // Disable decorations
            decorationProvider.setEnabled(false);
            decorationProvider.refresh();
            vscode.window.showInformationMessage('VQL Icons disabled');
        }
        
        // Update tree view
        treeProvider.refresh(decorationsEnabled);
    });
    context.subscriptions.push(toggleCommand);

    // Register refresh command
    const refreshCommand = vscode.commands.registerCommand('vql.refreshDecorations', () => {
        decorationProvider.refresh();
        vscode.window.showInformationMessage('VQL decorations refreshed');
    });
    context.subscriptions.push(refreshCommand);

    // Create matrix view provider
    matrixViewProvider = new VQLMatrixViewProvider(context);

    // Register matrix command
    const matrixCommand = vscode.commands.registerCommand('vql.showMatrix', () => {
        matrixViewProvider.show();
    });
    context.subscriptions.push(matrixCommand);

    // Create metadata view provider
    metadataProvider = new VQLMetadataProvider(context);

    // Register metadata command
    const metadataCommand = vscode.commands.registerCommand('vql.showMetadata', () => {
        metadataProvider.show();
    });
    context.subscriptions.push(metadataCommand);

    // Connect both views to storage watcher
    storageWatcher.onStorageChanged(() => {
        matrixViewProvider.refresh();
        metadataProvider.refresh();
    });

    // Register setup command
    const setupCommand = vscode.commands.registerCommand('vql.setup', async () => {
        const workspaceFolders = vscode.workspace.workspaceFolders;
        if (!workspaceFolders) {
            vscode.window.showErrorMessage('No workspace folder open');
            return;
        }

        const workspaceRoot = workspaceFolders[0].uri.fsPath;
        const vqlPath = path.join(workspaceRoot, 'VQL', 'vql_storage.json');

        // Check if already exists
        if (fs.existsSync(vqlPath)) {
            const response = await vscode.window.showWarningMessage(
                'VQL is already initialized in this workspace. Reinitialize?',
                'Yes', 'No'
            );
            if (response !== 'Yes') {
                return;
            }
        }

        try {
            // Run vql -su command
            await execAsync('vql -su', { cwd: workspaceRoot });
            vscode.window.showInformationMessage('VQL initialized successfully');
            
            // Refresh decorations
            decorationProvider.refresh();
        } catch (error: any) {
            vscode.window.showErrorMessage(`Failed to initialize VQL: ${error.message}`);
        }
    });
    context.subscriptions.push(setupCommand);

    // Register load principles command
    const loadPrinciplesCommand = vscode.commands.registerCommand('vql.loadPrinciples', async () => {
        const workspaceFolders = vscode.workspace.workspaceFolders;
        if (!workspaceFolders) {
            vscode.window.showErrorMessage('No workspace folder open');
            return;
        }

        const workspaceRoot = workspaceFolders[0].uri.fsPath;
        const vqlPath = path.join(workspaceRoot, 'VQL', 'vql_storage.json');

        // Check if VQL is initialized
        if (!fs.existsSync(vqlPath)) {
            vscode.window.showErrorMessage('VQL not initialized. Run Setup VQL first.');
            return;
        }

        // Check if principles already exist
        try {
            const storage = JSON.parse(fs.readFileSync(vqlPath, 'utf8'));
            if (storage.principles && Object.keys(storage.principles).length > 0) {
                const response = await vscode.window.showWarningMessage(
                    'Principles already loaded. Load new principles?',
                    'Yes', 'No'
                );
                if (response !== 'Yes') {
                    return;
                }
            }
        } catch (error) {
            // Continue if error reading
        }

        // Ask for principles file location
        const principlesPath = await vscode.window.showInputBox({
            prompt: 'Enter path to principles.md file',
            placeHolder: 'e.g., ./principles.md or /absolute/path/principles.md',
            validateInput: (value) => {
                if (!value) return 'Path is required';
                return null;
            }
        });

        if (!principlesPath) return;

        try {
            // Run vql -lp command
            await execAsync(`vql -lp "${principlesPath}"`, { cwd: workspaceRoot });
            vscode.window.showInformationMessage('Principles loaded successfully');
            
            // Refresh decorations
            decorationProvider.refresh();
        } catch (error: any) {
            vscode.window.showErrorMessage(`Failed to load principles: ${error.message}`);
        }
    });
    context.subscriptions.push(loadPrinciplesCommand);

    // Initial load
    decorationProvider.refresh();
}


export function deactivate() {
    if (storageWatcher) {
        storageWatcher.dispose();
    }
}