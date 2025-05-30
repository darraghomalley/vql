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
                'VQL Compliance Matrix',
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
            overflow: auto;
            border: 1px solid var(--vscode-panel-border);
            padding: 10px;
            min-height: 200px;
            position: relative;
        }

        .principle-panel {
            flex: 0 0 33%;
            background-color: var(--vscode-editor-background);
            border: 1px solid var(--vscode-panel-border);
            padding: 20px;
            overflow-y: auto;
            min-height: 150px;
        }

        .details-panel {
            flex: 0 0 33%;
            background-color: var(--vscode-editor-background);
            border: 1px solid var(--vscode-panel-border);
            padding: 20px;
            overflow-y: auto;
            max-height: 100%;
            min-width: 300px;
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
            font-size: 0.9em;
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
                <h3 style="margin: 0 0 10px 0;">Compliance Matrix</h3>
                <table id="matrix">
                    <!-- Content will be generated by JavaScript -->
                </table>
                <div id="selectionActions" style="display: none; position: absolute; bottom: 10px; right: 10px;">
                    <button id="reviewButton" style="padding: 5px 10px; cursor: pointer; background-color: var(--vscode-button-background); color: var(--vscode-button-foreground); border: none; border-radius: 2px; margin-right: 5px;">Review</button>
                    <button id="refactorButton" style="padding: 5px 10px; cursor: pointer; background-color: var(--vscode-button-background); color: var(--vscode-button-foreground); border: none; border-radius: 2px; margin-right: 5px;">Refactor</button>
                    <button id="clearButton" style="padding: 5px 10px; cursor: pointer; background-color: var(--vscode-button-secondaryBackground); color: var(--vscode-button-secondaryForeground); border: none; border-radius: 2px;">Clear</button>
                </div>
            </div>
            <div class="resize-handle-horizontal" id="horizontalResize"></div>
            <div class="details-panel" id="detailsPanel">
                <h3>Asset Review Details</h3>
                <div class="empty">Click on a compliance square to view details</div>
            </div>
        </div>
        <div class="resize-handle-vertical" id="verticalResize"></div>
        <div class="principle-panel" id="principlePanel">
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
        
        document.getElementById('reviewButton').addEventListener('click', () => {
            // Placeholder for review functionality
            console.log('Review selected cells:', Array.from(selectedCells));
        });
        
        document.getElementById('refactorButton').addEventListener('click', () => {
            // Placeholder for refactor functionality
            console.log('Refactor selected cells:', Array.from(selectedCells));
        });
        
        function clearSelections() {
            selectedCells.clear();
            document.querySelectorAll('.compliance-cell.selected').forEach(cell => {
                cell.classList.remove('selected');
            });
            updateSelectionButtons();
        }
        
        function updateSelectionButtons() {
            const selectionActions = document.getElementById('selectionActions');
            if (selectedCells.size > 0) {
                selectionActions.style.display = 'block';
            } else {
                selectionActions.style.display = 'none';
            }
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

        document.getElementById('verticalResize').addEventListener('mousedown', (e) => {
            isResizingVertical = true;
            document.body.style.cursor = 'col-resize';
        });

        document.getElementById('horizontalResize').addEventListener('mousedown', (e) => {
            isResizingHorizontal = true;
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
                    document.getElementById('principlePanel').style.flex = \`0 0 \${100 - percentage - 2}%\`;
                }
            }
        });

        document.addEventListener('mouseup', () => {
            isResizingVertical = false;
            isResizingHorizontal = false;
            document.body.style.cursor = 'default';
        });
    </script>
</body>
</html>`;
    }
}