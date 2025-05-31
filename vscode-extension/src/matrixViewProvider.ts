import * as vscode from 'vscode';
import * as fs from 'fs';
import * as path from 'path';
import { VQLStorageReader } from './storageReader';

export class VQLMatrixViewProvider {
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
                'vqlMatrix',
                'VQL Plane',
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
                        case 'openFile':
                            this.openFile(message.path);
                            break;
                        case 'refresh':
                            this.refresh();
                            break;
                        case 'showInfo':
                            vscode.window.showInformationMessage(message.message);
                            break;
                    }
                },
                null,
                this.context.subscriptions
            );
        }

        this.updateContent();
    }

    private openFile(filePath: string): void {
        const workspaceFolders = vscode.workspace.workspaceFolders;
        if (!workspaceFolders) return;

        const fullPath = vscode.Uri.file(`${workspaceFolders[0].uri.fsPath}/${filePath}`);
        vscode.window.showTextDocument(fullPath);
    }


    private updateContent(): void {
        if (!this.panel) return;

        const storage = this.storageReader.getStorage();
        if (!storage) return;

        // Build the matrix data
        const principles = Object.keys(storage.principles).sort();
        const assets = Object.values(storage.asset_references);
        
        // Debug logging
        console.log('VQL: Matrix data:', {
            principleCount: principles.length,
            assetCount: assets.length,
            assets: assets.map(a => ({
                name: a.short_name,
                path: a.path,
                hasReviews: a.principle_reviews && Object.keys(a.principle_reviews).length > 0
            }))
        });

        this.panel.webview.postMessage({
            command: 'updateMatrix',
            principles: principles,
            principleDetails: storage.principles,
            assets: assets,
            entities: storage.entities,
            assetTypes: storage.asset_types
        });
    }

    private getWebviewContent(): string {
        return `<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>VQL Compliance Matrix</title>
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
            padding-right: 40px; /* Extra right margin */
            box-sizing: border-box;
            gap: 10px;
        }

        .left-pane {
            flex: 0 0 66%;
            display: flex;
            flex-direction: column;
            gap: 10px;
            min-width: 400px;
        }

        .matrix-container {
            flex: 0 0 66%;
            display: flex;
            flex-direction: column;
            gap: 10px;
            overflow: hidden;
            border: 1px solid var(--vscode-panel-border);
            padding: 10px;
            min-height: 200px;
            position: relative;
        }

        .matrix-pane {
            flex: 1;
            overflow: auto;
            background-color: var(--vscode-editor-background);
            min-height: 0;
        }

        .vql-pane {
            flex: 0 0 120px;
            display: none;
            flex-direction: column;
            background-color: var(--vscode-editor-background);
            border: 1px solid var(--vscode-panel-border);
            padding: 10px;
            overflow: hidden;
        }

        .natural-pane {
            flex: 0 0 180px;
            display: none;
            flex-direction: column;
            background-color: var(--vscode-editor-background);
            border: 1px solid var(--vscode-panel-border);
            padding: 10px;
            overflow: hidden;
        }

        .matrix-resize-handle {
            height: 10px;
            cursor: row-resize;
            background-color: var(--vscode-widget-border);
            position: relative;
            user-select: none;
            flex-shrink: 0;
            display: none;
        }

        .matrix-resize-handle:hover {
            background-color: var(--vscode-focusBorder);
        }

        .matrix-resize-handle::after {
            content: '';
            position: absolute;
            background-color: var(--vscode-foreground);
            opacity: 0.2;
            height: 1px;
            width: 30px;
            left: 50%;
            top: 50%;
            transform: translate(-50%, -50%);
        }

        .matrix-pane.with-commands {
            flex: 1;
            min-height: 0;
        }

        .pane-header {
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: 10px;
            border-bottom: 1px solid var(--vscode-panel-border);
            padding-bottom: 5px;
        }

        .pane-header h4 {
            margin: 0;
            font-size: 14px;
            font-weight: bold;
        }

        .pane-textarea {
            flex: 1;
            width: 100%;
            background-color: var(--vscode-input-background);
            color: var(--vscode-input-foreground);
            border: 1px solid var(--vscode-input-border);
            padding: 8px;
            font-family: var(--vscode-font-family);
            resize: none;
            overflow-y: auto;
        }

        .vql-pane .pane-textarea {
            font-family: monospace;
            font-size: 12px;
        }

        .principle-panel {
            flex: 0 0 33%;
            background-color: var(--vscode-editor-background);
            border: 1px solid var(--vscode-panel-border);
            padding: 20px;
            overflow-y: auto;
            min-height: 150px;
            position: relative;
        }

        .details-panel {
            flex: 0 0 33%;
            background-color: var(--vscode-editor-background);
            border: 1px solid var(--vscode-panel-border);
            padding: 20px;
            overflow-y: auto;
            max-height: 100%;
            min-width: 300px;
            position: relative;
        }

        .toggle-button {
            position: absolute;
            z-index: 1000;
            background-color: var(--vscode-button-background);
            color: var(--vscode-button-foreground);
            border: none;
            padding: 6px 9px;
            cursor: pointer;
            border-radius: 4px;
            font-size: 12px;
            box-shadow: 0 2px 8px var(--vscode-widget-shadow);
            white-space: nowrap;
        }

        .toggle-button.in-matrix {
            transform: rotate(90deg);
            transform-origin: center;
        }

        .toggle-button:hover {
            background-color: var(--vscode-button-hoverBackground);
        }

        .asset-toggle.in-panel {
            top: 20px;
            right: 20px;
        }

        .asset-toggle.in-matrix {
            position: fixed;
            bottom: 1px;
            right: calc(1px + 12px + 15px);
            transform-origin: right bottom;
        }

        .principle-toggle.in-panel {
            top: 20px;
            right: 20px;
        }

        .principle-toggle.in-matrix {
            position: fixed;
            top: calc(1px + 80px + 30px);
            right: 1px;
            transform-origin: right top;
        }


        /* Resizable panes */
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

        .details-panel h3 {
            margin-top: 0;
            color: var(--vscode-foreground);
            border-bottom: 1px solid var(--vscode-panel-border);
            padding-bottom: 10px;
        }

        .details-panel p {
            margin: 10px 0;
            line-height: 1.5;
        }

        .details-panel .rating {
            font-size: 1.2em;
            margin: 15px 0;
        }

        .details-panel .analysis {
            background-color: var(--vscode-textBlockQuote-background);
            padding: 10px;
            border-radius: 4px;
            margin: 15px 0;
        }

        .details-panel .empty,
        .principle-panel .empty {
            color: var(--vscode-descriptionForeground);
            text-align: center;
            margin-top: 50px;
        }

        .principle-panel h3 {
            margin-top: 0;
            color: var(--vscode-foreground);
            border-bottom: 1px solid var(--vscode-panel-border);
            padding-bottom: 10px;
        }

        .principle-panel .actions {
            margin-top: 20px;
        }

        .principle-panel button {
            background-color: var(--vscode-button-background);
            color: var(--vscode-button-foreground);
            border: none;
            padding: 6px 14px;
            cursor: pointer;
            margin-right: 10px;
        }

        .principle-panel button:hover {
            background-color: var(--vscode-button-hoverBackground);
        }

        .principle-panel textarea {
            width: 100%;
            min-height: 150px;
            background-color: var(--vscode-input-background);
            color: var(--vscode-input-foreground);
            border: 1px solid var(--vscode-input-border);
            padding: 8px;
            font-family: var(--vscode-font-family);
            resize: vertical;
        }

        table {
            border-collapse: collapse;
            background-color: var(--vscode-editor-background);
        }

        th, td {
            border: 1px solid var(--vscode-panel-border);
            padding: 8px;
            text-align: center;
            position: relative;
        }

        th {
            background-color: var(--vscode-editor-background);
            font-weight: normal;
            color: var(--vscode-foreground);
        }

        /* Rotated headers */
        .rotated {
            height: 120px;
            white-space: nowrap;
            vertical-align: bottom;
            width: 30px;
        }

        .rotated > div {
            transform: rotate(-90deg);
            width: 30px;
        }

        .rotated > div > span {
            padding: 5px 10px;
        }

        /* File name cells */
        .filename {
            text-align: left;
            cursor: pointer;
            max-width: 200px;
            overflow: hidden;
            text-overflow: ellipsis;
            white-space: nowrap;
        }

        .filename:hover {
            text-decoration: underline;
        }

        .filename.high-compliance {
            color: #22c55e;
        }

        .filename.medium-compliance {
            color: #f59e0b;
        }

        .filename.low-compliance {
            color: #ef4444;
        }

        .filename.no-compliance {
            color: var(--vscode-disabledForeground);
        }

        /* Asset ref cells */
        .asset-ref {
            font-family: monospace;
            font-size: 1.35em;
            color: var(--vscode-textLink-foreground);
            font-weight: bold;
        }

        /* Compliance cells */
        .compliance-cell {
            font-size: 1.2em;
            cursor: pointer;
            position: relative;
            transition: background-color 0.2s;
        }

        .compliance-cell:hover {
            background-color: var(--vscode-list-hoverBackground);
        }
        
        .compliance-cell.selected {
            box-shadow: inset 0 0 0 2px var(--vscode-focusBorder);
        }

        /* Tooltip styles */
        .tooltip {
            position: absolute;
            z-index: 1000;
            background-color: var(--vscode-editorHoverWidget-background);
            border: 1px solid var(--vscode-editorHoverWidget-border);
            color: var(--vscode-editorHoverWidget-foreground);
            padding: 10px;
            border-radius: 3px;
            box-shadow: 0 2px 8px var(--vscode-widget-shadow);
            max-width: 400px;
            display: none;
            pointer-events: none;
        }

        .tooltip h4 {
            margin: 0 0 5px 0;
            color: var(--vscode-textLink-foreground);
        }

        .tooltip p {
            margin: 5px 0;
            font-size: 0.9em;
        }

        .sticky-header {
            position: sticky;
            top: 0;
            z-index: 10;
            background-color: var(--vscode-editor-background);
        }

        .sticky-column {
            position: sticky;
            left: 0;
            z-index: 5;
            background-color: var(--vscode-editor-background);
        }

        /* Bottom align file header */
        th.sticky-column {
            vertical-align: bottom;
            padding-bottom: 10px;
        }
    </style>
</head>
<body>
    <div class="main-container">
        <div class="left-pane" id="leftPane">
            <div class="matrix-container" id="matrixContainer">
                <div class="matrix-pane" id="matrixPane">
                    <h3 style="margin: 0 0 10px 0;">VQL Plane</h3>
                    <table id="matrix">
                        <!-- Content will be generated by JavaScript -->
                    </table>
                    <div id="selectionActions" style="display: none; position: absolute; top: 10px; right: 10px; z-index: 100;">
                        <button id="clearButton" style="padding: 5px 10px; cursor: pointer; background-color: var(--vscode-button-secondaryBackground); color: var(--vscode-button-secondaryForeground); border: none; border-radius: 2px;">Clear Selection</button>
                    </div>
                </div>
                <div class="matrix-resize-handle" id="matrixVqlResize"></div>
                <div class="vql-pane" id="vqlPane">
                    <div class="pane-header">
                        <h4>VQL Command</h4>
                        <button id="copyVqlButton" style="padding: 3px 8px; cursor: pointer; background-color: var(--vscode-button-background); color: var(--vscode-button-foreground); border: none; border-radius: 2px; font-size: 12px;">Copy</button>
                    </div>
                    <textarea id="vqlCommand" readonly class="pane-textarea"></textarea>
                </div>
                <div class="matrix-resize-handle" id="vqlNaturalResize"></div>
                <div class="natural-pane" id="naturalPane">
                    <div class="pane-header">
                        <h4>Natural Language</h4>
                        <button id="copyNaturalButton" style="padding: 3px 8px; cursor: pointer; background-color: var(--vscode-button-background); color: var(--vscode-button-foreground); border: none; border-radius: 2px; font-size: 12px;">Copy</button>
                    </div>
                    <textarea id="naturalCommand" readonly class="pane-textarea"></textarea>
                </div>
            </div>
            <div class="resize-handle-horizontal" id="horizontalResize"></div>
            <div class="details-panel" id="detailsPanel">
                <button class="toggle-button asset-toggle in-panel" id="assetToggle" title="Toggle Asset Review Details">Hide Asset</button>
                <h3>Asset Review Details</h3>
                <div class="empty">Click on a compliance square to view details</div>
            </div>
        </div>
        <div class="resize-handle-vertical" id="verticalResize"></div>
        <div class="principle-panel" id="principlePanel">
            <button class="toggle-button principle-toggle in-panel" id="principleToggle" title="Toggle Principle Details">Hide Principle</button>
            <h3>Principle Details</h3>
            <div class="empty">Click on a principle header or compliance square to view details</div>
        </div>
    </div>
    <div class="tooltip" id="tooltip"></div>

    <script>
        const vscode = acquireVsCodeApi();
        let matrixData = null;
        let selectedCells = new Set(); // Track selected cells

        // Handle messages from the extension
        window.addEventListener('message', event => {
            const message = event.data;
            if (message.command === 'updateMatrix') {
                matrixData = message;
                renderMatrix();
            }
        });

        // Add selection action button handlers
        document.getElementById('clearButton').addEventListener('click', () => {
            clearSelections();
        });
        
        // Add toggle functionality for panels
        document.getElementById('assetToggle').addEventListener('click', () => {
            toggleDetailsVisibility();
        });
        
        document.getElementById('principleToggle').addEventListener('click', () => {
            togglePrincipleVisibility();
        });
        
        document.getElementById('copyVqlButton').addEventListener('click', () => {
            const vqlCommand = document.getElementById('vqlCommand');
            navigator.clipboard.writeText(vqlCommand.value).then(() => {
                vscode.postMessage({ command: 'showInfo', message: 'VQL command copied to clipboard' });
            });
        });
        
        document.getElementById('copyNaturalButton').addEventListener('click', () => {
            const naturalCommand = document.getElementById('naturalCommand');
            navigator.clipboard.writeText(naturalCommand.value).then(() => {
                vscode.postMessage({ command: 'showInfo', message: 'Natural language command copied to clipboard' });
            });
        });
        
        function clearSelections() {
            selectedCells.clear();
            document.querySelectorAll('.compliance-cell.selected').forEach(cell => {
                cell.classList.remove('selected');
            });
            // Uncheck all checkboxes
            document.querySelectorAll('input[type="checkbox"]').forEach(checkbox => {
                checkbox.checked = false;
            });
            updateSelectionButtons();
        }
        
        function toggleRowSelection(assetShortName, isChecked) {
            if (!matrixData) return;
            
            matrixData.principles.forEach(p => {
                const cellKey = \`\${assetShortName}-\${p}\`;
                const cell = document.querySelector(\`[data-cell-key="\${cellKey}"]\`);
                
                if (isChecked) {
                    selectedCells.add(cellKey);
                    if (cell) cell.classList.add('selected');
                } else {
                    selectedCells.delete(cellKey);
                    if (cell) cell.classList.remove('selected');
                }
            });
            
            updateSelectionButtons();
        }
        
        function updateRowCheckbox(assetShortName) {
            if (!matrixData) return;
            
            // Check if all principles for this asset are selected
            const allSelected = matrixData.principles.every(p => {
                const cellKey = \`\${assetShortName}-\${p}\`;
                return selectedCells.has(cellKey);
            });
            
            // Find the checkbox for this row and update its state
            const checkbox = document.querySelector(\`input[data-asset="\${assetShortName}"]\`);
            if (checkbox) {
                checkbox.checked = allSelected;
            }
        }
        
        function toggleDetailsVisibility() {
            const detailsPanel = document.getElementById('detailsPanel');
            const horizontalResize = document.getElementById('horizontalResize');
            const assetToggle = document.getElementById('assetToggle');
            const matrixPane = document.getElementById('matrixPane');
            const matrixContainer = document.getElementById('matrixContainer');
            
            if (detailsPanel.style.display === 'none') {
                // Show details panel - move button back to panel
                detailsPanel.style.display = 'block';
                horizontalResize.style.display = 'block';
                assetToggle.textContent = 'Hide Asset';
                assetToggle.title = 'Hide Asset Review Details';
                assetToggle.className = 'toggle-button asset-toggle in-panel';
                detailsPanel.insertBefore(assetToggle, detailsPanel.firstChild);
                matrixContainer.style.flex = '0 0 66%';
            } else {
                // Hide details panel - move button to body
                detailsPanel.style.display = 'none';
                horizontalResize.style.display = 'none';
                assetToggle.textContent = 'Show Asset';
                assetToggle.title = 'Show Asset Review Details';
                assetToggle.className = 'toggle-button asset-toggle in-matrix';
                document.body.appendChild(assetToggle);
                matrixContainer.style.flex = '1';
            }
        }
        
        function togglePrincipleVisibility() {
            const principlePanel = document.getElementById('principlePanel');
            const verticalResize = document.getElementById('verticalResize');
            const principleToggle = document.getElementById('principleToggle');
            const leftPane = document.getElementById('leftPane');
            const matrixPane = document.getElementById('matrixPane');
            
            if (principlePanel.style.display === 'none') {
                // Show principle panel - move button back to panel
                principlePanel.style.display = 'block';
                verticalResize.style.display = 'block';
                principleToggle.textContent = 'Hide Principle';
                principleToggle.title = 'Hide Principle Details';
                principleToggle.className = 'toggle-button principle-toggle in-panel';
                principlePanel.insertBefore(principleToggle, principlePanel.firstChild);
                leftPane.style.flex = '0 0 66%';
            } else {
                // Hide principle panel - move button to body
                principlePanel.style.display = 'none';
                verticalResize.style.display = 'none';
                principleToggle.textContent = 'Show Principle';
                principleToggle.title = 'Show Principle Details';
                principleToggle.className = 'toggle-button principle-toggle in-matrix';
                document.body.appendChild(principleToggle);
                leftPane.style.flex = '1';
            }
        }
        
        function updateSelectionButtons() {
            const selectionActions = document.getElementById('selectionActions');
            const vqlPane = document.getElementById('vqlPane');
            const naturalPane = document.getElementById('naturalPane');
            const matrixPane = document.getElementById('matrixPane');
            const matrixVqlResize = document.getElementById('matrixVqlResize');
            const vqlNaturalResize = document.getElementById('vqlNaturalResize');
            
            if (selectedCells.size > 0) {
                selectionActions.style.display = 'block';
                vqlPane.style.display = 'flex';
                naturalPane.style.display = 'flex';
                matrixVqlResize.style.display = 'block';
                vqlNaturalResize.style.display = 'block';
                matrixPane.classList.add('with-commands');
                updateCommandTextBoxes();
            } else {
                selectionActions.style.display = 'none';
                vqlPane.style.display = 'none';
                naturalPane.style.display = 'none';
                matrixVqlResize.style.display = 'none';
                vqlNaturalResize.style.display = 'none';
                matrixPane.classList.remove('with-commands');
            }
        }
        
        function updateCommandTextBoxes() {
            if (!matrixData || selectedCells.size === 0) return;
            
            // Group selections by asset
            const assetGroups = new Map();
            selectedCells.forEach(cellKey => {
                const [assetRef, principle] = cellKey.split('-');
                if (!assetGroups.has(assetRef)) {
                    assetGroups.set(assetRef, new Set());
                }
                assetGroups.get(assetRef).add(principle);
            });
            
            // Generate VQL commands
            const vqlCommands = [];
            const naturalCommands = [];
            
            assetGroups.forEach((principles, assetRef) => {
                // Find the asset details
                const asset = matrixData.assets.find(a => a.short_name === assetRef);
                if (!asset) return;
                
                // Sort principles alphabetically
                const sortedPrinciples = Array.from(principles).sort();
                
                // VQL command
                if (sortedPrinciples.length === matrixData.principles.length) {
                    vqlCommands.push(\`:\${assetRef}.rv(-pr)\`);
                } else {
                    vqlCommands.push(\`:\${assetRef}.rv(\${sortedPrinciples.join(',')})\`);
                }
                
                // Natural language command
                const entityInfo = matrixData.entities[asset.entity];
                const assetTypeInfo = matrixData.assetTypes[asset.asset_type];
                const assetDescription = \`\${entityInfo ? entityInfo.description : asset.entity} \${assetTypeInfo ? assetTypeInfo.description : asset.asset_type}\`;
                
                let naturalText = \`Review the \${assetDescription} file (\${assetRef}) at \${asset.path}\\n\`;
                naturalText += \`for the following principles:\\n\`;
                
                sortedPrinciples.forEach(p => {
                    const principleInfo = matrixData.principleDetails[p];
                    if (principleInfo) {
                        naturalText += \`- \${principleInfo.long_name} (\${p}): \${principleInfo.guidance || 'No guidance available'}\\n\`;
                    }
                });
                
                naturalCommands.push(naturalText);
            });
            
            // Update text boxes
            document.getElementById('vqlCommand').value = vqlCommands.join('\\n');
            document.getElementById('naturalCommand').value = naturalCommands.join('\\n\\n');
        }

        function renderMatrix() {
            if (!matrixData) return;

            const table = document.getElementById('matrix');
            table.innerHTML = '';

            // Create header row
            const headerRow = document.createElement('tr');
            
            // Empty cells for file info columns
            headerRow.innerHTML = '<th class="sticky-header sticky-column">File</th>' +
                                  '<th class="rotated sticky-header"><div><span>Asset Shortname</span></div></th>' +
                                  '<th class="rotated sticky-header"><div><span>Entity</span></div></th>' +
                                  '<th class="rotated sticky-header"><div><span>Type</span></div></th>';

            // Add principle headers
            matrixData.principles.forEach(p => {
                const principleInfo = matrixData.principleDetails[p];
                const th = document.createElement('th');
                th.className = 'rotated sticky-header';
                th.innerHTML = \`<div><span title="\${principleInfo.long_name}">\${principleInfo.long_name}</span></div>\`;
                
                // Add hover and click for principles
                th.style.cursor = 'pointer';
                th.onmouseenter = (e) => showPrincipleTooltip(e, p, principleInfo);
                th.onmouseleave = hideTooltip;
                th.onclick = () => showPrinciplePanel(p, principleInfo);
                
                headerRow.appendChild(th);
            });

            // Add select all header
            const selectAllHeader = document.createElement('th');
            selectAllHeader.className = 'rotated sticky-header';
            selectAllHeader.innerHTML = '<div><span>Select All</span></div>';
            headerRow.appendChild(selectAllHeader);

            table.appendChild(headerRow);

            // Create data rows
            matrixData.assets.forEach(asset => {
                const row = document.createElement('tr');
                
                // Calculate lowest compliance for this asset
                let lowestCompliance = null;
                let hasLow = false;
                let hasMedium = false;
                let hasHigh = false;
                
                matrixData.principles.forEach(p => {
                    const review = asset.principle_reviews?.[p];
                    if (review) {
                        if (review.rating === 'L') hasLow = true;
                        else if (review.rating === 'M') hasMedium = true;
                        else if (review.rating === 'H') hasHigh = true;
                    }
                });
                
                if (hasLow) lowestCompliance = 'low';
                else if (hasMedium) lowestCompliance = 'medium';
                else if (hasHigh) lowestCompliance = 'high';
                else lowestCompliance = 'no';
                
                // File name cell with color based on compliance
                const fileCell = document.createElement('td');
                fileCell.className = \`filename sticky-column \${lowestCompliance}-compliance\`;
                fileCell.textContent = asset.path.split('/').pop();
                fileCell.title = asset.path;
                fileCell.onclick = () => {
                    vscode.postMessage({ command: 'openFile', path: asset.path });
                };
                row.appendChild(fileCell);

                // Asset ref cell
                const assetCell = document.createElement('td');
                assetCell.className = 'asset-ref';
                assetCell.textContent = asset.short_name;
                row.appendChild(assetCell);

                // Entity cell
                const entityCell = document.createElement('td');
                entityCell.className = 'asset-ref';
                entityCell.textContent = asset.entity;
                row.appendChild(entityCell);

                // Asset type cell
                const typeCell = document.createElement('td');
                typeCell.className = 'asset-ref';
                typeCell.textContent = asset.asset_type;
                row.appendChild(typeCell);

                // Compliance cells
                matrixData.principles.forEach(p => {
                    const cell = document.createElement('td');
                    cell.className = 'compliance-cell';
                    
                    // Check if this cell is selected
                    const cellKey = \`\${asset.short_name}-\${p}\`;
                    cell.setAttribute('data-cell-key', cellKey);
                    if (selectedCells.has(cellKey)) {
                        cell.classList.add('selected');
                    }
                    
                    const review = asset.principle_reviews?.[p];
                    if (review) {
                        const emoji = getComplianceEmoji(review.rating);
                        cell.textContent = emoji;
                        
                        // Add hover functionality
                        cell.onmouseenter = (e) => showTooltip(e, asset, p, review);
                        cell.onmouseleave = hideTooltip;
                        
                        // Add click functionality for both panels
                        cell.onclick = (e) => {
                            if (e.ctrlKey || e.metaKey) {
                                // Ctrl+click for selection
                                const cellKey = \`\${asset.short_name}-\${p}\`;
                                if (selectedCells.has(cellKey)) {
                                    selectedCells.delete(cellKey);
                                    cell.classList.remove('selected');
                                } else {
                                    selectedCells.add(cellKey);
                                    cell.classList.add('selected');
                                }
                                updateSelectionButtons();
                                updateRowCheckbox(asset.short_name);
                            } else {
                                // Regular click for showing details
                                const principleInfo = matrixData.principleDetails[p];
                                showPrinciplePanel(p, principleInfo);
                                showDetailsPanel(asset, p, review);
                            }
                        };
                    } else {
                        cell.textContent = '—';
                        cell.style.fontWeight = 'bold';
                        cell.style.color = 'var(--vscode-disabledForeground)';
                        
                        // Add click functionality for unreviewed cells too
                        cell.onclick = (e) => {
                            if (e.ctrlKey || e.metaKey) {
                                // Ctrl+click for selection
                                const cellKey = \`\${asset.short_name}-\${p}\`;
                                if (selectedCells.has(cellKey)) {
                                    selectedCells.delete(cellKey);
                                    cell.classList.remove('selected');
                                } else {
                                    selectedCells.add(cellKey);
                                    cell.classList.add('selected');
                                }
                                updateSelectionButtons();
                                updateRowCheckbox(asset.short_name);
                            } else {
                                // Regular click - show principle panel and empty details
                                const principleInfo = matrixData.principleDetails[p];
                                showPrinciplePanel(p, principleInfo);
                                showDetailsPanel(asset, p, null);
                            }
                        };
                    }
                    
                    row.appendChild(cell);
                });

                // Add select all checkbox
                const selectAllCell = document.createElement('td');
                selectAllCell.style.textAlign = 'center';
                const checkbox = document.createElement('input');
                checkbox.type = 'checkbox';
                checkbox.style.cursor = 'pointer';
                checkbox.setAttribute('data-asset', asset.short_name);
                checkbox.addEventListener('change', (e) => {
                    toggleRowSelection(asset.short_name, e.target.checked);
                });
                selectAllCell.appendChild(checkbox);
                row.appendChild(selectAllCell);

                table.appendChild(row);
            });
        }

        function getComplianceEmoji(rating) {
            switch (rating) {
                case 'H': return '🟩';
                case 'M': return '🟨';
                case 'L': return '🟥';
                default: return '—';
            }
        }

        function showTooltip(event, asset, principle, review) {
            const tooltip = document.getElementById('tooltip');
            const principleInfo = matrixData.principleDetails[principle];
            
            const ratingEmoji = getComplianceEmoji(review.rating);
            const ratingText = review.rating === 'H' ? 'High' : review.rating === 'M' ? 'Medium' : 'Low';
            
            tooltip.innerHTML = \`
                <h4>\${asset.path}</h4>
                <p><strong>\${principleInfo.long_name}</strong></p>
                <p>Rating: \${ratingEmoji} \${ratingText}</p>
                \${review.analysis ? \`<p>Analysis: \${review.analysis}</p>\` : ''}
                <p><em>Last reviewed: \${new Date(review.last_modified).toLocaleDateString()}</em></p>
            \`;
            
            tooltip.style.display = 'block';
            
            // Position tooltip
            const rect = event.target.getBoundingClientRect();
            tooltip.style.left = rect.left + 'px';
            tooltip.style.top = (rect.bottom + 5) + 'px';
            
            // Adjust if tooltip goes off screen
            const tooltipRect = tooltip.getBoundingClientRect();
            if (tooltipRect.right > window.innerWidth) {
                tooltip.style.left = (window.innerWidth - tooltipRect.width - 10) + 'px';
            }
            if (tooltipRect.bottom > window.innerHeight) {
                tooltip.style.top = (rect.top - tooltipRect.height - 5) + 'px';
            }
        }

        function hideTooltip() {
            document.getElementById('tooltip').style.display = 'none';
        }

        function showDetailsPanel(asset, principle, review) {
            const detailsPanel = document.getElementById('detailsPanel');
            const principleInfo = matrixData.principleDetails[principle];
            const entityInfo = matrixData.entities[asset.entity];
            const assetTypeInfo = matrixData.assetTypes[asset.asset_type];
            
            // Create asset long name from entity and type descriptions
            const assetLongName = \`\${entityInfo ? entityInfo.description : asset.entity} \${assetTypeInfo ? assetTypeInfo.description : asset.asset_type}\`;
            
            if (!review) {
                detailsPanel.innerHTML = \`
                    <h3>\${assetLongName} | \${principleInfo.long_name} Review</h3>
                    <p><strong>Asset:</strong> \${asset.short_name} &nbsp;&nbsp; <strong>Entity:</strong> \${entityInfo ? entityInfo.description : asset.entity} &nbsp;&nbsp; <strong>Asset Type:</strong> \${assetTypeInfo ? assetTypeInfo.description : asset.asset_type} &nbsp;&nbsp; <strong>File:</strong> \${asset.path}</p>
                    <div class="empty">This principle has not been reviewed yet</div>
                \`;
                return;
            }
            
            const ratingEmoji = getComplianceEmoji(review.rating);
            const ratingText = review.rating === 'H' ? 'High' : review.rating === 'M' ? 'Medium' : 'Low';
            
            detailsPanel.innerHTML = \`
                <h3>\${assetLongName} | \${principleInfo.long_name} Review | \${ratingEmoji} \${ratingText}</h3>
                <p><strong>Asset:</strong> \${asset.short_name} &nbsp;&nbsp; <strong>Entity:</strong> \${entityInfo ? entityInfo.description : asset.entity} &nbsp;&nbsp; <strong>Asset Type:</strong> \${assetTypeInfo ? assetTypeInfo.description : asset.asset_type} &nbsp;&nbsp; <strong>File:</strong> \${asset.path}</p>
                \${review.analysis ? \`
                    <div class="analysis">
                        <strong>Analysis:</strong><br>
                        \${review.analysis}
                    </div>
                \` : ''}
                <p><em>Last reviewed: \${new Date(review.last_modified).toLocaleDateString()}</em></p>
            \`;
        }

        function showPrincipleTooltip(event, principleShort, principleInfo) {
            const tooltip = document.getElementById('tooltip');
            
            tooltip.innerHTML = \`
                <h4>\${principleInfo.long_name} (\${principleShort})</h4>
                \${principleInfo.guidance ? \`<p>\${principleInfo.guidance}</p>\` : '<p>No guidance available</p>'}
            \`;
            
            tooltip.style.display = 'block';
            
            // Position tooltip
            const rect = event.target.getBoundingClientRect();
            tooltip.style.left = rect.left + 'px';
            tooltip.style.top = (rect.bottom + 5) + 'px';
        }

        function showPrinciplePanel(principleShort, principleInfo) {
            const principlePanel = document.getElementById('principlePanel');
            
            principlePanel.innerHTML = \`
                <h3>Principle: \${principleInfo.long_name}</h3>
                <p><strong>Short Name:</strong> \${principleShort}</p>
                <div id="principleContent">
                    <p><strong>Guidance:</strong></p>
                    <p>\${principleInfo.guidance || 'No guidance available'}</p>
                </div>
            \`;
        }


        // Resizable panes
        let isResizingVertical = false;
        let isResizingHorizontal = false;
        let isResizingMatrixVql = false;
        let isResizingVqlNatural = false;

        document.getElementById('verticalResize').addEventListener('mousedown', (e) => {
            isResizingVertical = true;
            document.body.style.cursor = 'col-resize';
        });

        document.getElementById('horizontalResize').addEventListener('mousedown', (e) => {
            isResizingHorizontal = true;
            document.body.style.cursor = 'row-resize';
        });

        document.getElementById('matrixVqlResize').addEventListener('mousedown', (e) => {
            isResizingMatrixVql = true;
            document.body.style.cursor = 'row-resize';
        });

        document.getElementById('vqlNaturalResize').addEventListener('mousedown', (e) => {
            isResizingVqlNatural = true;
            document.body.style.cursor = 'row-resize';
        });

        document.addEventListener('mousemove', (e) => {
            if (isResizingVertical) {
                const container = document.querySelector('.main-container');
                const leftPane = document.getElementById('leftPane');
                const percentage = (e.clientX - container.offsetLeft) / container.offsetWidth * 100;
                
                if (percentage > 30 && percentage < 80) {
                    leftPane.style.flex = \`0 0 \${percentage}%\`;
                    document.getElementById('detailsPanel').style.flex = \`0 0 \${100 - percentage - 2}%\`;
                }
            } else if (isResizingHorizontal) {
                const leftPane = document.getElementById('leftPane');
                const matrixContainer = document.getElementById('matrixContainer');
                const percentage = (e.clientY - leftPane.offsetTop) / leftPane.offsetHeight * 100;
                
                if (percentage > 30 && percentage < 80) {
                    matrixContainer.style.flex = \`0 0 \${percentage}%\`;
                    document.getElementById('detailsPanel').style.flex = \`0 0 \${100 - percentage - 2}%\`;
                }
            } else if (isResizingMatrixVql) {
                const matrixContainer = document.getElementById('matrixContainer');
                const matrixPane = document.getElementById('matrixPane');
                const vqlPane = document.getElementById('vqlPane');
                const containerRect = matrixContainer.getBoundingClientRect();
                const relativeY = e.clientY - containerRect.top;
                const percentage = relativeY / containerRect.height * 100;
                
                if (percentage > 20 && percentage < 70) {
                    matrixPane.style.flex = \`0 0 \${percentage}%\`;
                    // VQL pane keeps its fixed height, natural pane gets the rest
                }
            } else if (isResizingVqlNatural) {
                const matrixContainer = document.getElementById('matrixContainer');
                const vqlPane = document.getElementById('vqlPane');
                const naturalPane = document.getElementById('naturalPane');
                const containerRect = matrixContainer.getBoundingClientRect();
                const relativeY = e.clientY - containerRect.top;
                
                // Calculate the height for VQL pane
                const matrixPane = document.getElementById('matrixPane');
                const matrixHeight = matrixPane.getBoundingClientRect().height;
                const resizeHandle1Height = 10;
                const availableHeight = containerRect.height - matrixHeight - resizeHandle1Height - 10; // 10 for second resize handle
                const vqlHeight = relativeY - matrixHeight - resizeHandle1Height;
                
                if (vqlHeight > 60 && vqlHeight < availableHeight - 60) {
                    vqlPane.style.flex = \`0 0 \${vqlHeight}px\`;
                    naturalPane.style.flex = \`0 0 \${availableHeight - vqlHeight}px\`;
                }
            }
        });

        document.addEventListener('mouseup', () => {
            isResizingVertical = false;
            isResizingHorizontal = false;
            isResizingMatrixVql = false;
            isResizingVqlNatural = false;
            document.body.style.cursor = 'default';
        });
    </script>
</body>
</html>`;
    }
}