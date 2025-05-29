import * as vscode from 'vscode';
import { VQLStorageReader } from './storageReader';
import { ComplianceLevel, AssetReference } from './types';

export class VQLDecorationProvider implements vscode.FileDecorationProvider {
    private readonly _onDidChangeFileDecorations = new vscode.EventEmitter<vscode.Uri | vscode.Uri[]>();
    readonly onDidChangeFileDecorations = this._onDidChangeFileDecorations.event;
    
    private storageReader: VQLStorageReader;
    private decorations = new Map<string, vscode.FileDecoration>();
    private enabled = true;

    constructor() {
        this.storageReader = new VQLStorageReader();
    }

    setEnabled(enabled: boolean): void {
        this.enabled = enabled;
        if (!enabled) {
            // Clear all decorations when disabled
            this.decorations.clear();
        }
    }

    provideFileDecoration(uri: vscode.Uri): vscode.FileDecoration | undefined {
        return this.decorations.get(uri.toString());
    }

    refresh(): void {
        this.storageReader.reload();
        this.updateDecorations();
    }

    private updateDecorations(): void {
        this.decorations.clear();

        // Don't create decorations if disabled
        if (!this.enabled) {
            // Need to get all files and fire event to clear their decorations
            vscode.workspace.findFiles('**/*', '**/node_modules/**').then(files => {
                this._onDidChangeFileDecorations.fire(files);
            });
            return;
        }

        const workspaceFolders = vscode.workspace.workspaceFolders;
        if (!workspaceFolders || workspaceFolders.length === 0) {
            console.log('No workspace folders found');
            return;
        }

        // Get all files in workspace
        vscode.workspace.findFiles('**/*', '**/node_modules/**').then(files => {
            console.log(`Found ${files.length} files in workspace`);
            let decoratedCount = 0;
            
            for (const file of files) {
                const asset = this.storageReader.getAssetByPath(file.fsPath);
                if (asset) {
                    console.log(`Found asset for file: ${file.fsPath}`);
                    const decoration = this.createDecoration(asset);
                    if (decoration) {
                        this.decorations.set(file.toString(), decoration);
                        decoratedCount++;
                        console.log(`Created decoration for: ${file.toString()}, badge: ${decoration.badge}`);
                    }
                }
            }
            
            console.log(`Created ${decoratedCount} decorations`);
            
            // Notify VS Code that decorations have changed
            this._onDidChangeFileDecorations.fire(files);
        });
    }

    private createDecoration(asset: AssetReference): vscode.FileDecoration | undefined {
        const principles = this.storageReader.getAllPrinciples();
        if (principles.length === 0) {
            return undefined;
        }

        // Sort principles alphabetically for consistent display order
        const sortedPrinciples = [...principles].sort();

        // Build badge text and tooltip
        let badgeText = '';
        const tooltipParts: string[] = [];
        const complianceLevels: (ComplianceLevel | null)[] = [];
        
        for (const principle of sortedPrinciples) {
            const compliance = this.storageReader.getPrincipleCompliance(asset, principle);
            const letter = principle.toUpperCase();
            const principleName = this.storageReader.getPrincipleName(principle);
            
            // Show principle letters - colored for reviewed, grey for unreviewed
            // For now, use simple ASCII letters instead of Unicode monospace
            badgeText += letter;
            
            if (compliance) {
                complianceLevels.push(compliance);
            } else {
                // Use null to indicate grey/unreviewed
                complianceLevels.push(null);
            }
            
            // Add to tooltip with colored square symbols
            let complianceText = 'Not Reviewed';
            let emoji = '🟪'; // Purple square for not reviewed
            if (compliance === 'H') {
                complianceText = 'High';
                emoji = '🟩'; // Green square
            } else if (compliance === 'M') {
                complianceText = 'Medium';
                emoji = '🟨'; // Yellow/amber square
            } else if (compliance === 'L') {
                complianceText = 'Low';
                emoji = '🟥'; // Red square
            }
            
            // Simple format without letter prefix
            tooltipParts.push(`${emoji} ${principleName} (${complianceText})`);
        }

        // Get the lowest compliance level for overall color
        const lowestCompliance = this.getLowestCompliance(asset, principles);
        
        // Create decoration with badge
        console.log(`Creating decoration with badge: "${badgeText}" for asset: ${asset.short_name}`);
        
        // Use single colored square badge based on lowest compliance
        let simpleBadge = '';
        if (lowestCompliance === 'H') {
            simpleBadge = '🟩'; // Green square
        } else if (lowestCompliance === 'M') {
            simpleBadge = '🟨'; // Yellow/amber square
        } else if (lowestCompliance === 'L') {
            simpleBadge = '🟥'; // Red square
        } else {
            simpleBadge = '🟪'; // Purple square for nothing reviewed
        }
        
        const decoration = new vscode.FileDecoration(
            simpleBadge,
            '\n' + tooltipParts.join('\n'), // Add newline at start for alignment
            this.getComplianceColor(lowestCompliance)
        );
        
        // Don't propagate to parent folders
        decoration.propagate = false;
        
        console.log(`Decoration created - badge: ${decoration.badge}, color: ${decoration.color}`);

        return decoration;
    }

    private getLowestCompliance(asset: AssetReference, principles: string[]): ComplianceLevel {
        let hasLow = false;
        let hasMedium = false;
        let hasHigh = false;

        for (const principle of principles) {
            const compliance = this.storageReader.getPrincipleCompliance(asset, principle);
            if (compliance) {
                if (compliance === 'L') {
                    hasLow = true;
                } else if (compliance === 'M') {
                    hasMedium = true;
                } else if (compliance === 'H') {
                    hasHigh = true;
                }
            }
        }

        if (hasLow) return 'L';
        if (hasMedium) return 'M';
        if (hasHigh) return 'H';
        return null;
    }

    private getComplianceColor(compliance: ComplianceLevel): vscode.ThemeColor {
        switch (compliance) {
            case 'H':
                return new vscode.ThemeColor('vql.highCompliance');
            case 'M':
                return new vscode.ThemeColor('vql.mediumCompliance');
            case 'L':
                return new vscode.ThemeColor('vql.lowCompliance');
            default:
                return new vscode.ThemeColor('disabledForeground');
        }
    }

    private toMonospace(letter: string): string {
        // TEMPORARY: Use regular ASCII for debugging
        return letter;
        
        // Original Unicode implementation (commented out for testing):
        /*
        // Convert to Mathematical Monospace Unicode characters
        // These should render with consistent width
        const charCode = letter.charCodeAt(0);
        
        if (charCode >= 65 && charCode <= 90) { // A-Z
            // Mathematical Monospace Capital Letters start at U+1D670
            return String.fromCodePoint(0x1D670 + (charCode - 65));
        } else if (charCode >= 97 && charCode <= 122) { // a-z
            // Mathematical Monospace Small Letters start at U+1D68A
            return String.fromCodePoint(0x1D68A + (charCode - 97));
        }
        
        // Return original if not a letter
        return letter;
        */
    }
}