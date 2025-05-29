import * as vscode from 'vscode';
import * as fs from 'fs';
import * as path from 'path';
import { VQLStorage, AssetReference, ComplianceLevel } from './types';

export class VQLStorageReader {
    private storage: VQLStorage | null = null;
    private vqlPath: string | null = null;

    constructor() {
        this.findVQLStorage();
    }

    public getStorage(): VQLStorage | null {
        return this.storage;
    }

    private findVQLStorage(): void {
        const workspaceFolders = vscode.workspace.workspaceFolders;
        if (!workspaceFolders || workspaceFolders.length === 0) {
            return;
        }

        const workspaceRoot = workspaceFolders[0].uri.fsPath;
        
        // Look for VQL directory (case insensitive)
        const possiblePaths = [
            path.join(workspaceRoot, 'VQL', 'vql_storage.json'),
            path.join(workspaceRoot, 'vql', 'vql_storage.json')
        ];

        for (const possiblePath of possiblePaths) {
            if (fs.existsSync(possiblePath)) {
                this.vqlPath = possiblePath;
                this.loadStorage();
                break;
            }
        }
    }

    private loadStorage(): void {
        if (!this.vqlPath) {
            console.log('No VQL path set');
            return;
        }

        try {
            console.log(`Loading VQL storage from: ${this.vqlPath}`);
            const content = fs.readFileSync(this.vqlPath, 'utf8');
            this.storage = JSON.parse(content);
            if (this.storage && this.storage.asset_references) {
                console.log(`Loaded ${Object.keys(this.storage.asset_references).length} assets`);
            }
        } catch (error) {
            console.error('Failed to load VQL storage:', error);
            this.storage = null;
        }
    }

    public reload(): void {
        this.loadStorage();
    }

    public getAssetByPath(filePath: string): AssetReference | null {
        if (!this.storage) {
            console.log('No storage loaded');
            return null;
        }

        // Convert to project-relative path from workspace root
        const workspaceFolders = vscode.workspace.workspaceFolders;
        if (!workspaceFolders || workspaceFolders.length === 0) {
            return null;
        }

        const workspaceRoot = workspaceFolders[0].uri.fsPath;
        const projectRelativePath = path.relative(workspaceRoot, filePath).replace(/\\/g, '/');
        
        console.log(`Looking for asset with path: ${projectRelativePath} (from ${filePath})`);

        // Find asset with matching path
        for (const asset of Object.values(this.storage.asset_references)) {
            console.log(`  Checking against: ${asset.path}`);
            if (asset.path === projectRelativePath) {
                console.log(`  MATCH FOUND!`);
                return asset;
            }
        }

        return null;
    }

    public getPrincipleCompliance(asset: AssetReference, principleShortName: string): ComplianceLevel {
        // Check principle_reviews structure
        const review = asset.principle_reviews[principleShortName];
        if (review && review.rating) {
            return review.rating as ComplianceLevel;
        }
        return null;
    }

    public getAllPrinciples(): string[] {
        if (!this.storage) {
            return [];
        }
        return Object.keys(this.storage.principles).sort();
    }

    public getPrincipleName(shortName: string): string {
        if (!this.storage || !this.storage.principles[shortName]) {
            return shortName.toUpperCase();
        }
        return this.storage.principles[shortName].long_name;
    }

    public getStoragePath(): string | null {
        return this.vqlPath;
    }
}