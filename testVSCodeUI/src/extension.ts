import * as vscode from 'vscode';

// This is a dummy extension file for testing VQL decorations
export function activate(context: vscode.ExtensionContext) {
    console.log('Test extension is now active!');
    
    try {
        // Example of good error handling
        const disposable = vscode.commands.registerCommand('test.helloWorld', () => {
            vscode.window.showInformationMessage('Hello World from Test Extension!');
        });
        
        context.subscriptions.push(disposable);
    } catch (error) {
        console.error('Failed to register command:', error);
        vscode.window.showErrorMessage('Failed to activate extension');
    }
}

export function deactivate() {
    console.log('Test extension is now deactivated');
}