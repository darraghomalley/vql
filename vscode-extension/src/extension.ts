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

export async function activate(context: vscode.ExtensionContext) {
    console.log('VQL extension is now active');
    
    // Check VQL CLI version
    try {
        const { stdout } = await execAsync('vql --version');
        console.log('VQL CLI version:', stdout.trim());
    } catch (error) {
        console.error('Failed to check VQL version:', error);
        vscode.window.showWarningMessage('VQL CLI not found or version check failed');
    }

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
    const treeDataProvider = vscode.window.registerTreeDataProvider('vqlActions', treeProvider);
    context.subscriptions.push(treeDataProvider);

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

    // Register matrix refresh command
    const refreshMatrixCommand = vscode.commands.registerCommand('vql.refreshMatrix', () => {
        matrixViewProvider.refresh();
    });
    context.subscriptions.push(refreshMatrixCommand);

    // Register metadata refresh command
    const refreshMetadataCommand = vscode.commands.registerCommand('vql.refreshMetadata', () => {
        metadataProvider.refresh();
    });
    context.subscriptions.push(refreshMetadataCommand);

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
            
            // Refresh decorations and tree view
            decorationProvider.refresh();
            treeProvider.refresh(decorationsEnabled);
        } catch (error: any) {
            vscode.window.showErrorMessage(`Failed to initialize VQL: ${error.message}`);
        }
    });
    context.subscriptions.push(setupCommand);

    // Register add asset reference command for context menu
    const addAssetRefCommand = vscode.commands.registerCommand('vql.addAssetReference', async (uri: vscode.Uri) => {
        if (!uri || !uri.fsPath) {
            vscode.window.showErrorMessage('No file selected');
            return;
        }
        
        // Use the metadata provider to show the add asset form
        metadataProvider.showAddAssetReference(uri.fsPath);
    });
    context.subscriptions.push(addAssetRefCommand);

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
            console.log('VQL: Checking principles:', {
                principlesExists: !!storage.principles,
                principleCount: storage.principles ? Object.keys(storage.principles).length : 0,
                principleKeys: storage.principles ? Object.keys(storage.principles) : []
            });
            
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
                // Check if file exists
                const fullPath = path.isAbsolute(value) ? value : path.join(workspaceRoot, value);
                console.log('VQL: Validating path:', { input: value, fullPath, exists: fs.existsSync(fullPath) });
                if (!fs.existsSync(fullPath)) {
                    return `File not found: ${fullPath}`;
                }
                return null;
            }
        });

        if (!principlesPath) return;

        try {
            // Run vql principles loading command with proper argument handling
            // Use spawn for proper handling of paths with spaces
            const { spawn } = require('child_process');
            const result = await new Promise<{success: boolean, message: string}>((resolve) => {
                const child = spawn('vql', ['-pr', '-get', principlesPath], { cwd: workspaceRoot });
                let stdout = '';
                let stderr = '';
                
                child.stdout.on('data', (data: Buffer) => {
                    stdout += data.toString();
                });
                
                child.stderr.on('data', (data: Buffer) => {
                    stderr += data.toString();
                });
                
                child.on('close', (code: number) => {
                    if (code === 0) {
                        resolve({ success: true, message: stdout });
                    } else {
                        resolve({ success: false, message: stderr || `Command failed with code ${code}` });
                    }
                });
            });
            
            if (!result.success) {
                throw new Error(result.message);
            }
            
            // Log the actual output for debugging
            console.log('VQL: Load principles output:', result.message);
            
            vscode.window.showInformationMessage('Principles loaded successfully');
            
            // Wait a moment for file to be written completely
            await new Promise(resolve => setTimeout(resolve, 500));
            
            // Refresh decorations and tree view
            decorationProvider.refresh();
            treeProvider.refresh(decorationsEnabled);
            
            // Also refresh matrix and metadata if they're open
            matrixViewProvider.refresh();
            metadataProvider.refresh();
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