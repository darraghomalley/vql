import * as vscode from 'vscode';

interface DecorationConfig {
    color: string;
    badge: string;
}

// Medium error handling - basic but could be more comprehensive
export class DecorationProvider {
    private decorationTypes: Map<string, vscode.TextEditorDecorationType>;
    
    constructor() {
        this.decorationTypes = new Map();
        this.initializeDecorations();
    }
    
    private initializeDecorations(): void {
        try {
            // Strong type usage for VSCode API
            const configs: Record<string, DecorationConfig> = {
                high: { color: '#22c55e', badge: 'H' },
                medium: { color: '#f59e0b', badge: 'M' },
                low: { color: '#ef4444', badge: 'L' }
            };
            
            // Good separation but complex logic could be split
            for (const [key, config] of Object.entries(configs)) {
                this.decorationTypes.set(key, this.createDecorationType(config));
            }
        } catch (error) {
            console.error('Failed to initialize decorations');
        }
    }
    
    private createDecorationType(config: DecorationConfig): vscode.TextEditorDecorationType {
        return vscode.window.createTextEditorDecorationType({
            backgroundColor: config.color,
            after: {
                contentText: config.badge,
                color: 'white'
            }
        });
    }
}