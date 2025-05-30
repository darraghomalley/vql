import * as vscode from 'vscode';
import * as path from 'path';
import * as child_process from 'child_process';
import { VQLStorageReader } from './storageReader';

export class VQLMetadataProvider {
    private panel: vscode.WebviewPanel | undefined;
    private storageReader: VQLStorageReader;

    constructor(private context: vscode.ExtensionContext) {
        this.storageReader = new VQLStorageReader();
    }

    public refresh(): void {
        if (this.panel) {
            // Reload storage and update content
            this.storageReader = new VQLStorageReader();
            this.updateContent();
        }
    }

    public show(): void {
        // Create or reveal the webview panel
        if (this.panel) {
            this.panel.reveal();
        } else {
            this.panel = vscode.window.createWebviewPanel(
                'vqlMetadata',
                'VQL Metadata',
                vscode.ViewColumn.One,
                {
                    enableScripts: true,
                    retainContextWhenHidden: true
                }
            );

            this.panel.webview.html = this.getWebviewContent();

            // Handle disposal
            this.panel.onDidDispose(() => {
                this.panel = undefined;
            }, null, this.context.subscriptions);

            // Handle messages from the webview
            this.panel.webview.onDidReceiveMessage(
                async message => {
                    switch (message.command) {
                        case 'saveEntity':
                            await this.saveEntity(message.short, message.long);
                            break;
                        case 'saveAssetType':
                            await this.saveAssetType(message.short, message.description);
                            break;
                        case 'saveAssetReference':
                            await this.saveAssetReference(message.short, message.entity, message.assetType, message.path, message.exemplar);
                            break;
                        case 'savePrinciple':
                            await this.savePrinciple(message.short, message.long, message.guidance);
                            break;
                        case 'refresh':
                            this.refresh();
                            break;
                    }
                },
                null,
                this.context.subscriptions
            );
        }

        this.updateContent();
    }

    private async runVQLCommand(args: string[]): Promise<{ success: boolean; message: string }> {
        return new Promise((resolve) => {
            const workspaceFolders = vscode.workspace.workspaceFolders;
            if (!workspaceFolders) {
                resolve({ success: false, message: 'No workspace folder found' });
                return;
            }

            const cwd = workspaceFolders[0].uri.fsPath;
            const vqlPath = vscode.workspace.getConfiguration('vql').get<string>('cliPath', 'vql');
            
            child_process.exec(`${vqlPath} ${args.join(' ')}`, { cwd }, (error, stdout, stderr) => {
                if (error) {
                    resolve({ success: false, message: stderr || error.message });
                } else {
                    resolve({ success: true, message: stdout });
                }
            });
        });
    }

    private async saveEntity(short: string, long: string): Promise<void> {
        const result = await this.runVQLCommand(['-er', '-add', short, long]);
        if (result.success) {
            vscode.window.showInformationMessage(`Entity ${short} saved successfully`);
            this.refresh();
        } else {
            vscode.window.showErrorMessage(`Failed to save entity: ${result.message}`);
        }
    }

    private async saveAssetType(short: string, description: string): Promise<void> {
        const result = await this.runVQLCommand(['-at', '-add', short, description]);
        if (result.success) {
            vscode.window.showInformationMessage(`Asset Type ${short} saved successfully`);
            this.refresh();
        } else {
            vscode.window.showErrorMessage(`Failed to save asset type: ${result.message}`);
        }
    }

    private async saveAssetReference(short: string, entity: string, assetType: string, path: string, exemplar: boolean): Promise<void> {
        // First add the asset reference
        const result = await this.runVQLCommand(['-ar', '-add', short, entity, assetType, path]);
        
        // If exemplar flag is set, run separate command to set exemplar
        if (result.success && exemplar) {
            const exemplarResult = await this.runVQLCommand(['-se', short, 't']);
            if (!exemplarResult.success) {
                vscode.window.showWarningMessage(`Asset added but failed to set as exemplar: ${exemplarResult.message}`);
            }
        }
        if (result.success) {
            vscode.window.showInformationMessage(`Asset Reference ${short} saved successfully`);
            this.refresh();
        } else {
            vscode.window.showErrorMessage(`Failed to save asset reference: ${result.message}`);
        }
    }

    private async savePrinciple(short: string, long: string, guidance: string): Promise<void> {
        const result = await this.runVQLCommand(['-pr', '-add', short, long, guidance]);
        if (result.success) {
            vscode.window.showInformationMessage(`Principle ${short} saved successfully`);
            this.refresh();
        } else {
            vscode.window.showErrorMessage(`Failed to save principle: ${result.message}`);
        }
    }

    public showAddAssetReference(filePath: string): void {
        // First show the panel
        this.show();

        // Get workspace folder
        const workspaceFolders = vscode.workspace.workspaceFolders;
        if (!workspaceFolders) {
            vscode.window.showErrorMessage('No workspace folder found');
            return;
        }

        const workspaceRoot = workspaceFolders[0].uri.fsPath;
        
        // Check if file is within workspace
        if (!filePath.startsWith(workspaceRoot)) {
            vscode.window.showErrorMessage('File must be within the workspace');
            return;
        }

        // Calculate relative path
        const relativePath = path.relative(workspaceRoot, filePath).replace(/\\/g, '/');

        // Check if asset already exists
        const storage = this.storageReader.getStorage();
        if (storage) {
            const existingAsset = Object.values(storage.asset_references).find(
                asset => asset.path === relativePath
            );

            if (existingAsset) {
                vscode.window.showInformationMessage(
                    `File is already tracked as asset '${existingAsset.short_name}' (${existingAsset.entity}${existingAsset.asset_type})`
                );
                return;
            }
        }

        // Wait a bit for panel to be ready, then send message
        setTimeout(() => {
            if (this.panel) {
                this.panel.webview.postMessage({
                    command: 'showAddAssetReferenceForm',
                    path: relativePath
                });
            }
        }, 100);
    }


    private updateContent(): void {
        if (!this.panel) return;

        const storage = this.storageReader.getStorage();
        if (!storage) return;

        this.panel.webview.postMessage({
            command: 'updateMetadata',
            entities: storage.entities,
            assetTypes: storage.asset_types,
            assetReferences: storage.asset_references,
            principles: storage.principles
        });
    }

    private getWebviewContent(): string {
        return `<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>VQL Metadata</title>
    <style>
        body {
            font-family: var(--vscode-font-family);
            font-size: var(--vscode-font-size);
            color: var(--vscode-foreground);
            background-color: var(--vscode-editor-background);
            padding: 0;
            margin: 0;
            height: 100vh;
            overflow: hidden;
        }

        .main-container {
            display: flex;
            height: 100vh;
            padding: 20px;
            box-sizing: border-box;
            gap: 10px;
        }

        .left-pane {
            flex: 0 0 50%;
            display: flex;
            flex-direction: column;
            gap: 10px;
        }

        .right-pane {
            flex: 0 0 50%;
            display: flex;
            flex-direction: column;
            gap: 10px;
        }

        .section {
            flex: 1;
            background-color: var(--vscode-editor-background);
            border: 1px solid var(--vscode-panel-border);
            padding: 20px;
            overflow-y: auto;
        }

        .section h3 {
            margin-top: 0;
            color: var(--vscode-foreground);
            border-bottom: 1px solid var(--vscode-panel-border);
            padding-bottom: 10px;
            display: flex;
            justify-content: space-between;
            align-items: center;
        }

        .resize-handle-vertical {
            width: 10px;
            cursor: col-resize;
            background-color: var(--vscode-widget-border);
            position: relative;
            user-select: none;
            flex-shrink: 0;
        }

        .resize-handle-horizontal {
            height: 10px;
            cursor: row-resize;
            background-color: var(--vscode-widget-border);
            position: relative;
            user-select: none;
            flex-shrink: 0;
        }

        .resize-handle-vertical:hover,
        .resize-handle-horizontal:hover {
            background-color: var(--vscode-focusBorder);
        }

        .resize-handle-vertical::after,
        .resize-handle-horizontal::after {
            content: '';
            position: absolute;
            background-color: var(--vscode-foreground);
            opacity: 0.2;
        }

        .resize-handle-vertical::after {
            width: 1px;
            height: 30px;
            left: 50%;
            top: 50%;
            transform: translate(-50%, -50%);
        }

        .resize-handle-horizontal::after {
            height: 1px;
            width: 30px;
            left: 50%;
            top: 50%;
            transform: translate(-50%, -50%);
        }

        .item-list {
            margin: 15px 0;
        }

        .item {
            padding: 10px;
            margin: 5px 0;
            background-color: var(--vscode-list-inactiveSelectionBackground);
            border-radius: 4px;
            cursor: pointer;
            display: flex;
            justify-content: space-between;
            align-items: center;
        }

        .item:hover {
            background-color: var(--vscode-list-hoverBackground);
        }

        .item-short {
            font-family: monospace;
            font-weight: bold;
            color: var(--vscode-textLink-foreground);
        }

        .item-description {
            color: var(--vscode-descriptionForeground);
        }

        .edit-form {
            margin: 15px 0;
            padding: 15px;
            background-color: var(--vscode-textBlockQuote-background);
            border-radius: 4px;
        }

        .form-field {
            margin: 10px 0;
        }

        .form-field label {
            display: block;
            margin-bottom: 5px;
            font-weight: 500;
        }

        .form-field input,
        .form-field textarea,
        .form-field select {
            width: 100%;
            padding: 6px;
            background-color: var(--vscode-input-background);
            color: var(--vscode-input-foreground);
            border: 1px solid var(--vscode-input-border);
            border-radius: 2px;
        }

        .form-field textarea {
            min-height: 100px;
            resize: vertical;
        }

        .form-actions {
            margin-top: 15px;
            display: flex;
            gap: 10px;
        }

        button {
            background-color: var(--vscode-button-background);
            color: var(--vscode-button-foreground);
            border: none;
            padding: 6px 14px;
            cursor: pointer;
            border-radius: 2px;
        }

        button:hover {
            background-color: var(--vscode-button-hoverBackground);
        }

        button.secondary {
            background-color: var(--vscode-button-secondaryBackground);
            color: var(--vscode-button-secondaryForeground);
        }

        button.secondary:hover {
            background-color: var(--vscode-button-secondaryHoverBackground);
        }

        .add-button {
            background-color: var(--vscode-button-background);
            color: var(--vscode-button-foreground);
            border: none;
            padding: 4px 10px;
            cursor: pointer;
            border-radius: 2px;
            font-size: 0.9em;
        }

        .add-button:hover {
            background-color: var(--vscode-button-hoverBackground);
        }

        .empty {
            color: var(--vscode-descriptionForeground);
            text-align: center;
            margin-top: 20px;
        }
    </style>
</head>
<body>
    <div class="main-container">
        <div class="left-pane" id="leftPane">
            <div class="section" id="assetTypesSection" style="flex: 0 0 30%;">
                <h3>
                    <span>Asset Types</span>
                    <button class="add-button" onclick="addAssetType()">+</button>
                </h3>
                <div id="assetTypesContent"></div>
            </div>
            <div class="resize-handle-horizontal" id="leftHorizontalResize"></div>
            <div class="section" id="assetReferencesSection" style="flex: 0 0 70%;">
                <h3>
                    <span>Asset References</span>
                    <button class="add-button" onclick="addAssetReference()">+</button>
                </h3>
                <div id="assetReferencesContent"></div>
            </div>
        </div>
        <div class="resize-handle-vertical" id="verticalResize"></div>
        <div class="right-pane" id="rightPane">
            <div class="section" id="entitiesSection" style="flex: 0 0 30%;">
                <h3>
                    <span>Entities</span>
                    <button class="add-button" onclick="addEntity()">+</button>
                </h3>
                <div id="entitiesContent"></div>
            </div>
            <div class="resize-handle-horizontal" id="rightHorizontalResize"></div>
            <div class="section" id="principlesSection" style="flex: 0 0 70%;">
                <h3>
                    <span>Principles</span>
                    <button class="add-button" onclick="addPrinciple()">+</button>
                </h3>
                <div id="principlesContent"></div>
            </div>
        </div>
    </div>

    <script>
        const vscode = acquireVsCodeApi();
        let metadata = null;
        let editingMode = {};

        // Handle messages from the extension
        window.addEventListener('message', event => {
            const message = event.data;
            if (message.command === 'updateMetadata') {
                metadata = message;
                renderAll();
            } else if (message.command === 'showAddAssetReferenceForm') {
                showAssetReferenceForm('', null, message.path);
            }
        });


        function renderAll() {
            if (!metadata) return;
            renderEntities();
            renderAssetTypes();
            renderAssetReferences();
            renderPrinciples();
        }

        function renderEntities() {
            const content = document.getElementById('entitiesContent');
            if (!metadata.entities || Object.keys(metadata.entities).length === 0) {
                content.innerHTML = '<div class="empty">No entities defined</div>';
                return;
            }

            const items = Object.entries(metadata.entities).map(([short, entity]) => \`
                <div class="item" onclick="editEntity('\${short}')">
                    <div>
                        <span class="item-short">\${short}</span>
                        <span class="item-description">\${entity.description || entity.long_name || ''}</span>
                    </div>
                </div>
            \`).join('');
            
            content.innerHTML = '<div class="item-list">' + items + '</div>';
        }

        function renderAssetTypes() {
            const content = document.getElementById('assetTypesContent');
            if (!metadata.assetTypes || Object.keys(metadata.assetTypes).length === 0) {
                content.innerHTML = '<div class="empty">No asset types defined</div>';
                return;
            }

            const items = Object.entries(metadata.assetTypes).map(([short, type]) => \`
                <div class="item" onclick="editAssetType('\${short}')">
                    <div>
                        <span class="item-short">\${short}</span>
                        <span class="item-description">\${type.description}</span>
                    </div>
                </div>
            \`).join('');
            
            content.innerHTML = '<div class="item-list">' + items + '</div>';
        }

        function renderAssetReferences() {
            const content = document.getElementById('assetReferencesContent');
            if (!metadata.assetReferences || Object.keys(metadata.assetReferences).length === 0) {
                content.innerHTML = '<div class="empty">No asset references defined</div>';
                return;
            }

            const items = Object.entries(metadata.assetReferences).map(([short, ref]) => \`
                <div class="item" onclick="editAssetReference('\${short}')">
                    <div>
                        <span class="item-short">\${short}</span>
                        <span class="item-description">\${ref.entity}\${ref.asset_type} - \${ref.path}</span>
                        \${ref.exemplar ? '<span>⭐</span>' : ''}
                    </div>
                </div>
            \`).join('');
            
            content.innerHTML = '<div class="item-list">' + items + '</div>';
        }

        function renderPrinciples() {
            const content = document.getElementById('principlesContent');
            if (!metadata.principles || Object.keys(metadata.principles).length === 0) {
                content.innerHTML = '<div class="empty">No principles defined</div>';
                return;
            }

            const items = Object.entries(metadata.principles).map(([short, principle]) => \`
                <div class="item" onclick="editPrinciple('\${short}')">
                    <div>
                        <span class="item-short">\${short}</span>
                        <span class="item-description">\${principle.long_name}</span>
                    </div>
                </div>
            \`).join('');
            
            content.innerHTML = '<div class="item-list">' + items + '</div>';
        }

        // Entity functions
        function addEntity() {
            showEntityForm();
        }

        function editEntity(short) {
            const entity = metadata.entities[short];
            showEntityForm(short, entity);
        }

        function showEntityForm(short = '', entity = null) {
            const content = document.getElementById('entitiesContent');
            content.innerHTML = \`
                <div class="edit-form">
                    <div class="form-field">
                        <label>Short Name</label>
                        <input type="text" id="entityShort" value="\${short}" \${short ? 'readonly' : ''}>
                    </div>
                    <div class="form-field">
                        <label>Long Name</label>
                        <input type="text" id="entityLong" value="\${entity?.description || entity?.long_name || ''}">
                    </div>
                    <div class="form-actions">
                        <button onclick="saveEntity('\${short}')">Save</button>
                        <button class="secondary" onclick="renderEntities()">Cancel</button>
                    </div>
                </div>
            \`;
        }

        function saveEntity(originalShort) {
            const short = document.getElementById('entityShort').value;
            const long = document.getElementById('entityLong').value;
            
            if (!short || !long) {
                alert('Both short and long names are required');
                return;
            }

            vscode.postMessage({
                command: 'saveEntity',
                short: short,
                long: long
            });
        }

        // Asset Type functions
        function addAssetType() {
            showAssetTypeForm();
        }

        function editAssetType(short) {
            const type = metadata.assetTypes[short];
            showAssetTypeForm(short, type);
        }

        function showAssetTypeForm(short = '', type = null) {
            const content = document.getElementById('assetTypesContent');
            content.innerHTML = \`
                <div class="edit-form">
                    <div class="form-field">
                        <label>Short Name</label>
                        <input type="text" id="assetTypeShort" value="\${short}" \${short ? 'readonly' : ''}>
                    </div>
                    <div class="form-field">
                        <label>Description</label>
                        <input type="text" id="assetTypeDesc" value="\${type?.description || ''}">
                    </div>
                    <div class="form-actions">
                        <button onclick="saveAssetType('\${short}')">Save</button>
                        <button class="secondary" onclick="renderAssetTypes()">Cancel</button>
                    </div>
                </div>
            \`;
        }

        function saveAssetType(originalShort) {
            const short = document.getElementById('assetTypeShort').value;
            const description = document.getElementById('assetTypeDesc').value;
            
            if (!short || !description) {
                alert('Both short name and description are required');
                return;
            }

            vscode.postMessage({
                command: 'saveAssetType',
                short: short,
                description: description
            });
        }

        // Asset Reference functions
        function addAssetReference() {
            showAssetReferenceForm();
        }

        function editAssetReference(short) {
            const ref = metadata.assetReferences[short];
            showAssetReferenceForm(short, ref);
        }

        function showAssetReferenceForm(short = '', ref = null, prePath = null) {
            const content = document.getElementById('assetReferencesContent');
            
            // Create entity options
            const entityOptions = Object.entries(metadata.entities).map(([s, e]) => 
                \`<option value="\${s}" \${ref?.entity === s ? 'selected' : ''}>\${s} - \${e.description || e.long_name || ''}</option>\`
            ).join('');
            
            // Create asset type options
            const typeOptions = Object.entries(metadata.assetTypes).map(([s, t]) => 
                \`<option value="\${s}" \${ref?.asset_type === s ? 'selected' : ''}>\${s} - \${t.description}</option>\`
            ).join('');
            
            content.innerHTML = \`
                <div class="edit-form">
                    <div class="form-field">
                        <label>Short Name</label>
                        <input type="text" id="assetRefShort" value="\${short}" \${short ? 'readonly' : ''}>
                    </div>
                    <div class="form-field">
                        <label>Entity</label>
                        <select id="assetRefEntity">\${entityOptions}</select>
                    </div>
                    <div class="form-field">
                        <label>Asset Type</label>
                        <select id="assetRefType">\${typeOptions}</select>
                    </div>
                    <div class="form-field">
                        <label>Path</label>
                        <input type="text" id="assetRefPath" value="\${prePath || ref?.path || ''}">
                    </div>
                    <div class="form-field">
                        <label>
                            <input type="checkbox" id="assetRefExemplar" \${ref?.exemplar ? 'checked' : ''}>
                            Exemplar
                        </label>
                    </div>
                    <div class="form-actions">
                        <button onclick="saveAssetReference('\${short}')">Save</button>
                        <button class="secondary" onclick="renderAssetReferences()">Cancel</button>
                    </div>
                </div>
            \`;
            
            // Focus on short name field if this was triggered by a drop
            if (prePath) {
                setTimeout(() => {
                    document.getElementById('assetRefShort').focus();
                }, 100);
            }
        }

        function saveAssetReference(originalShort) {
            const short = document.getElementById('assetRefShort').value;
            const entity = document.getElementById('assetRefEntity').value;
            const assetType = document.getElementById('assetRefType').value;
            const path = document.getElementById('assetRefPath').value;
            const exemplar = document.getElementById('assetRefExemplar').checked;
            
            if (!short || !entity || !assetType || !path) {
                alert('All fields are required');
                return;
            }

            vscode.postMessage({
                command: 'saveAssetReference',
                short: short,
                entity: entity,
                assetType: assetType,
                path: path,
                exemplar: exemplar
            });
        }

        // Principle functions
        function addPrinciple() {
            showPrincipleForm();
        }

        function editPrinciple(short) {
            const principle = metadata.principles[short];
            showPrincipleForm(short, principle);
        }

        function showPrincipleForm(short = '', principle = null) {
            const content = document.getElementById('principlesContent');
            content.innerHTML = \`
                <div class="edit-form">
                    <div class="form-field">
                        <label>Short Name</label>
                        <input type="text" id="principleShort" value="\${short}" \${short ? 'readonly' : ''}>
                    </div>
                    <div class="form-field">
                        <label>Long Name</label>
                        <input type="text" id="principleLong" value="\${principle?.long_name || ''}">
                    </div>
                    <div class="form-field">
                        <label>Guidance</label>
                        <textarea id="principleGuidance">\${principle?.guidance || ''}</textarea>
                    </div>
                    <div class="form-actions">
                        <button onclick="savePrinciple('\${short}')">Save</button>
                        <button class="secondary" onclick="renderPrinciples()">Cancel</button>
                    </div>
                </div>
            \`;
        }

        function savePrinciple(originalShort) {
            const short = document.getElementById('principleShort').value;
            const long = document.getElementById('principleLong').value;
            const guidance = document.getElementById('principleGuidance').value;
            
            if (!short || !long) {
                alert('Short and long names are required');
                return;
            }

            vscode.postMessage({
                command: 'savePrinciple',
                short: short,
                long: long,
                guidance: guidance
            });
        }

        // Resizable panes
        let isResizingVertical = false;
        let isResizingLeftHorizontal = false;
        let isResizingRightHorizontal = false;

        document.getElementById('verticalResize').addEventListener('mousedown', (e) => {
            isResizingVertical = true;
            document.body.style.cursor = 'col-resize';
        });

        document.getElementById('leftHorizontalResize').addEventListener('mousedown', (e) => {
            isResizingLeftHorizontal = true;
            document.body.style.cursor = 'row-resize';
        });

        document.getElementById('rightHorizontalResize').addEventListener('mousedown', (e) => {
            isResizingRightHorizontal = true;
            document.body.style.cursor = 'row-resize';
        });

        document.addEventListener('mousemove', (e) => {
            if (isResizingVertical) {
                const container = document.querySelector('.main-container');
                const leftPane = document.getElementById('leftPane');
                const percentage = (e.clientX - container.offsetLeft) / container.offsetWidth * 100;
                
                if (percentage > 10 && percentage < 90) {
                    leftPane.style.flex = \`0 0 \${percentage}%\`;
                    document.getElementById('rightPane').style.flex = \`0 0 \${100 - percentage - 1}%\`;
                }
            } else if (isResizingLeftHorizontal) {
                const leftPane = document.getElementById('leftPane');
                const assetTypesSection = document.getElementById('assetTypesSection');
                const percentage = (e.clientY - leftPane.offsetTop) / leftPane.offsetHeight * 100;
                
                if (percentage > 10 && percentage < 90) {
                    assetTypesSection.style.flex = \`0 0 \${percentage}%\`;
                    document.getElementById('assetReferencesSection').style.flex = \`0 0 \${100 - percentage - 1}%\`;
                }
            } else if (isResizingRightHorizontal) {
                const rightPane = document.getElementById('rightPane');
                const entitiesSection = document.getElementById('entitiesSection');
                const percentage = (e.clientY - rightPane.offsetTop) / rightPane.offsetHeight * 100;
                
                if (percentage > 10 && percentage < 90) {
                    entitiesSection.style.flex = \`0 0 \${percentage}%\`;
                    document.getElementById('principlesSection').style.flex = \`0 0 \${100 - percentage - 1}%\`;
                }
            }
        });

        document.addEventListener('mouseup', () => {
            isResizingVertical = false;
            isResizingLeftHorizontal = false;
            isResizingRightHorizontal = false;
            document.body.style.cursor = 'default';
        });
    </script>
</body>
</html>`;
    }
}