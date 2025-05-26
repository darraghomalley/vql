#!/usr/bin/env node
import { Server } from '@modelcontextprotocol/sdk/server/index.js';
import { StdioServerTransport } from '@modelcontextprotocol/sdk/server/stdio.js';
import {
  CallToolRequestSchema,
  ErrorCode,
  ListToolsRequestSchema,
  McpError,
} from '@modelcontextprotocol/sdk/types.js';
import { exec } from 'child_process';
import { promisify } from 'util';
import { readFile } from 'fs/promises';
import { join } from 'path';

const execAsync = promisify(exec);

class VQLMCPServer {
  private server: Server;
  private vqlMode: boolean = true; // VQL mode is on by default

  constructor() {
    this.server = new Server(
      {
        name: 'vql-mcp-server',
        version: '1.0.0',
      },
      {
        capabilities: {
          tools: {},
        },
      }
    );

    this.setupHandlers();
  }

  private async executeVQLCommand(command: string): Promise<string> {
    try {
      const { stdout, stderr } = await execAsync(command);
      if (stderr) {
        console.error(`VQL stderr: ${stderr}`);
      }
      return stdout;
    } catch (error: any) {
      throw new McpError(
        ErrorCode.InternalError,
        `Failed to execute VQL command: ${error.message}`
      );
    }
  }

  private async readVQLStorage(): Promise<any> {
    try {
      // Find VQL directory by looking for VQL/vql_storage.json
      const { stdout } = await execAsync('find . -name "vql_storage.json" -path "*/VQL/*" | head -1');
      const storagePath = stdout.trim();
      
      if (!storagePath) {
        throw new Error('VQL storage file not found. Run vql -su first.');
      }

      const content = await readFile(storagePath, 'utf-8');
      return JSON.parse(content);
    } catch (error: any) {
      throw new McpError(
        ErrorCode.InternalError,
        `Failed to read VQL storage: ${error.message}`
      );
    }
  }

  private setupHandlers() {
    interface ToolArgs {
      directory?: string;
      short?: string;
      long?: string;
      guidance?: string;
      path?: string;
      description?: string;
      shortName?: string;
      entity?: string;
      assetType?: string;
      asset?: string;
      principle?: string;
      review?: string;
      principles?: string[];
      isExemplar?: boolean;
      level?: 'H' | 'M' | 'L';
      referenceAsset?: string;
      referenceAssets?: string[];
    }
    // Handle list tools request
    this.server.setRequestHandler(ListToolsRequestSchema, async () => ({
      tools: [
        // Mode Management
        {
          name: 'enable_vql_mode',
          description: 'Enable VQL mode for AI-assisted code quality management',
          inputSchema: {
            type: 'object',
            properties: {},
          },
        },
        {
          name: 'disable_vql_mode',
          description: 'Disable VQL mode',
          inputSchema: {
            type: 'object',
            properties: {},
          },
        },
        {
          name: 'get_vql_mode',
          description: 'Get current VQL mode status',
          inputSchema: {
            type: 'object',
            properties: {},
          },
        },

        // Setup
        {
          name: 'setup_vql',
          description: 'Initialize VQL in a directory',
          inputSchema: {
            type: 'object',
            properties: {
              directory: {
                type: 'string',
                description: 'Directory path to initialize VQL',
              },
            },
            required: ['directory'],
          },
        },

        // Principles Management
        {
          name: 'list_principles',
          description: 'List all VQL principles',
          inputSchema: {
            type: 'object',
            properties: {},
          },
        },
        {
          name: 'add_principle',
          description: 'Add a new principle',
          inputSchema: {
            type: 'object',
            properties: {
              short: {
                type: 'string',
                description: 'Single character identifier for the principle',
              },
              long: {
                type: 'string',
                description: 'Full name of the principle',
              },
              guidance: {
                type: 'string',
                description: 'Detailed guidance for the principle',
              },
            },
            required: ['short', 'long', 'guidance'],
          },
        },
        {
          name: 'load_principles_from_markdown',
          description: 'Load principles from a markdown file',
          inputSchema: {
            type: 'object',
            properties: {
              path: {
                type: 'string',
                description: 'Path to the markdown file containing principles',
              },
            },
            required: ['path'],
          },
        },

        // Entity Management
        {
          name: 'list_entities',
          description: 'List all entities',
          inputSchema: {
            type: 'object',
            properties: {},
          },
        },
        {
          name: 'add_entity',
          description: 'Add a new entity',
          inputSchema: {
            type: 'object',
            properties: {
              short: {
                type: 'string',
                description: 'Short identifier for the entity',
              },
              long: {
                type: 'string',
                description: 'Full name of the entity',
              },
            },
            required: ['short', 'long'],
          },
        },

        // Asset Type Management
        {
          name: 'list_asset_types',
          description: 'List all asset types',
          inputSchema: {
            type: 'object',
            properties: {},
          },
        },
        {
          name: 'add_asset_type',
          description: 'Add a new asset type',
          inputSchema: {
            type: 'object',
            properties: {
              short: {
                type: 'string',
                description: 'Short identifier for the asset type',
              },
              description: {
                type: 'string',
                description: 'Description of the asset type',
              },
            },
            required: ['short', 'description'],
          },
        },

        // Asset Management
        {
          name: 'list_assets',
          description: 'List all assets',
          inputSchema: {
            type: 'object',
            properties: {},
          },
        },
        {
          name: 'add_asset',
          description: 'Add a new asset',
          inputSchema: {
            type: 'object',
            properties: {
              shortName: {
                type: 'string',
                description: 'Short identifier for the asset',
              },
              entity: {
                type: 'string',
                description: 'Entity this asset belongs to',
              },
              assetType: {
                type: 'string',
                description: 'Type of the asset',
              },
              path: {
                type: 'string',
                description: 'File path to the asset',
              },
            },
            required: ['shortName', 'entity', 'assetType', 'path'],
          },
        },

        // Review Management
        {
          name: 'store_review',
          description: 'Store a review for an asset and principle',
          inputSchema: {
            type: 'object',
            properties: {
              asset: {
                type: 'string',
                description: 'Asset identifier',
              },
              principle: {
                type: 'string',
                description: 'Principle identifier',
              },
              review: {
                type: 'string',
                description: 'Review content',
              },
            },
            required: ['asset', 'principle', 'review'],
          },
        },
        {
          name: 'get_all_reviews',
          description: 'Get all reviews for a specific asset',
          inputSchema: {
            type: 'object',
            properties: {
              asset: {
                type: 'string',
                description: 'Asset identifier',
              },
            },
            required: ['asset'],
          },
        },
        {
          name: 'get_review',
          description: 'Get a specific review for an asset and principle',
          inputSchema: {
            type: 'object',
            properties: {
              asset: {
                type: 'string',
                description: 'Asset identifier',
              },
              principle: {
                type: 'string',
                description: 'Principle identifier',
              },
            },
            required: ['asset', 'principle'],
          },
        },
        {
          name: 'get_multiple_reviews',
          description: 'Get reviews for an asset and multiple principles',
          inputSchema: {
            type: 'object',
            properties: {
              asset: {
                type: 'string',
                description: 'Asset identifier',
              },
              principles: {
                type: 'array',
                items: {
                  type: 'string',
                },
                description: 'Array of principle identifiers',
              },
            },
            required: ['asset', 'principles'],
          },
        },

        // Compliance Management
        {
          name: 'set_exemplar',
          description: 'Set whether an asset is an exemplar',
          inputSchema: {
            type: 'object',
            properties: {
              asset: {
                type: 'string',
                description: 'Asset identifier',
              },
              isExemplar: {
                type: 'boolean',
                description: 'Whether the asset is an exemplar',
              },
            },
            required: ['asset', 'isExemplar'],
          },
        },
        {
          name: 'set_compliance',
          description: 'Set compliance level for an asset and principle',
          inputSchema: {
            type: 'object',
            properties: {
              asset: {
                type: 'string',
                description: 'Asset identifier',
              },
              principle: {
                type: 'string',
                description: 'Principle identifier',
              },
              level: {
                type: 'string',
                enum: ['H', 'M', 'L'],
                description: 'Compliance level: H (High), M (Medium), L (Low)',
              },
            },
            required: ['asset', 'principle', 'level'],
          },
        },

        // AI Workflow Commands
        {
          name: 'review_asset_all_principles',
          description: 'Review an asset against all principles',
          inputSchema: {
            type: 'object',
            properties: {
              asset: {
                type: 'string',
                description: 'Asset identifier to review',
              },
            },
            required: ['asset'],
          },
        },
        {
          name: 'review_asset_principles',
          description: 'Review an asset against specific principles',
          inputSchema: {
            type: 'object',
            properties: {
              asset: {
                type: 'string',
                description: 'Asset identifier to review',
              },
              principles: {
                type: 'array',
                items: {
                  type: 'string',
                },
                description: 'Array of principle identifiers to review against',
              },
            },
            required: ['asset', 'principles'],
          },
        },
        {
          name: 'refactor_asset_all_principles',
          description: 'Refactor an asset based on all principles',
          inputSchema: {
            type: 'object',
            properties: {
              asset: {
                type: 'string',
                description: 'Asset identifier to refactor',
              },
            },
            required: ['asset'],
          },
        },
        {
          name: 'refactor_asset_principles',
          description: 'Refactor an asset based on specific principles',
          inputSchema: {
            type: 'object',
            properties: {
              asset: {
                type: 'string',
                description: 'Asset identifier to refactor',
              },
              principles: {
                type: 'array',
                items: {
                  type: 'string',
                },
                description: 'Array of principle identifiers to refactor against',
              },
            },
            required: ['asset', 'principles'],
          },
        },
        {
          name: 'refactor_asset_using_reference',
          description: 'Refactor an asset using another asset as reference',
          inputSchema: {
            type: 'object',
            properties: {
              asset: {
                type: 'string',
                description: 'Asset identifier to refactor',
              },
              referenceAsset: {
                type: 'string',
                description: 'Reference asset identifier to use as example',
              },
            },
            required: ['asset', 'referenceAsset'],
          },
        },
        {
          name: 'refactor_asset_principles_with_references',
          description: 'Refactor an asset based on specific principles using reference assets',
          inputSchema: {
            type: 'object',
            properties: {
              asset: {
                type: 'string',
                description: 'Asset identifier to refactor',
              },
              principles: {
                type: 'array',
                items: {
                  type: 'string',
                },
                description: 'Array of principle identifiers to refactor against',
              },
              referenceAssets: {
                type: 'array',
                items: {
                  type: 'string',
                },
                description: 'Array of reference asset identifiers to use as examples',
              },
            },
            required: ['asset', 'principles', 'referenceAssets'],
          },
        },
        {
          name: 'refactor_asset_all_principles_with_references',
          description: 'Refactor an asset based on all principles using reference assets',
          inputSchema: {
            type: 'object',
            properties: {
              asset: {
                type: 'string',
                description: 'Asset identifier to refactor',
              },
              referenceAssets: {
                type: 'array',
                items: {
                  type: 'string',
                },
                description: 'Array of reference asset identifiers to use as examples',
              },
            },
            required: ['asset', 'referenceAssets'],
          },
        },
        {
          name: 'refactor_asset_principles_with_references',
          description: 'Refactor an asset based on specific principles using reference assets',
          inputSchema: {
            type: 'object',
            properties: {
              asset: {
                type: 'string',
                description: 'Asset identifier to refactor',
              },
              principles: {
                type: 'array',
                items: {
                  type: 'string',
                },
                description: 'Array of principle identifiers to refactor against',
              },
              referenceAssets: {
                type: 'array',
                items: {
                  type: 'string',
                },
                description: 'Array of reference asset identifiers to use as examples',
              },
            },
            required: ['asset', 'principles', 'referenceAssets'],
          },
        },
        {
          name: 'refactor_asset_all_principles_with_references',
          description: 'Refactor an asset based on all principles using reference assets',
          inputSchema: {
            type: 'object',
            properties: {
              asset: {
                type: 'string',
                description: 'Asset identifier to refactor',
              },
              referenceAssets: {
                type: 'array',
                items: {
                  type: 'string',
                },
                description: 'Array of reference asset identifiers to use as examples',
              },
            },
            required: ['asset', 'referenceAssets'],
          },
        },

        // Batch Operations
        {
          name: 'review_all_assets_all_principles',
          description: 'Review all assets against all principles',
          inputSchema: {
            type: 'object',
            properties: {},
          },
        },
        {
          name: 'review_all_assets_principles',
          description: 'Review all assets against specific principles',
          inputSchema: {
            type: 'object',
            properties: {
              principles: {
                type: 'array',
                items: {
                  type: 'string',
                },
                description: 'Array of principle identifiers to review against',
              },
            },
            required: ['principles'],
          },
        },
        {
          name: 'refactor_all_assets_all_principles',
          description: 'Refactor all assets based on all principles',
          inputSchema: {
            type: 'object',
            properties: {},
          },
        },
        {
          name: 'refactor_all_assets_principles',
          description: 'Refactor all assets based on specific principles',
          inputSchema: {
            type: 'object',
            properties: {
              principles: {
                type: 'array',
                items: {
                  type: 'string',
                },
                description: 'Array of principle identifiers to refactor against',
              },
            },
            required: ['principles'],
          },
        },
      ],
    }));

    // Handle tool calls
    this.server.setRequestHandler(CallToolRequestSchema, async (request) => {
      const { name, arguments: args = {} } = request.params;
      const typedArgs = args as ToolArgs;

      try {
        switch (name) {
          // Mode Management
          case 'enable_vql_mode':
            this.vqlMode = true;
            return { content: [{ type: 'text', text: 'VQL mode enabled' }] };

          case 'disable_vql_mode':
            this.vqlMode = false;
            return { content: [{ type: 'text', text: 'VQL mode disabled' }] };

          case 'get_vql_mode':
            return { content: [{ type: 'text', text: `VQL mode is ${this.vqlMode ? 'enabled' : 'disabled'}` }] };

          // Setup
          case 'setup_vql':
            const setupOutput = await this.executeVQLCommand(`vql -su "${typedArgs.directory}"`);
            return { content: [{ type: 'text', text: setupOutput }] };

          // Principles Management
          case 'list_principles':
            const principlesOutput = await this.executeVQLCommand('vql -pr');
            return { content: [{ type: 'text', text: principlesOutput }] };

          case 'add_principle':
            const addPrincipleOutput = await this.executeVQLCommand(
              `vql -pr -add ${typedArgs.short} ${typedArgs.long} "${typedArgs.guidance}"`
            );
            return { content: [{ type: 'text', text: addPrincipleOutput }] };

          case 'load_principles_from_markdown':
            const loadPrinciplesOutput = await this.executeVQLCommand(
              `vql -pr -get "${typedArgs.path}"`
            );
            return { content: [{ type: 'text', text: loadPrinciplesOutput }] };

          // Entity Management
          case 'list_entities':
            const entitiesOutput = await this.executeVQLCommand('vql -er');
            return { content: [{ type: 'text', text: entitiesOutput }] };

          case 'add_entity':
            const addEntityOutput = await this.executeVQLCommand(
              `vql -er -add ${typedArgs.short} ${typedArgs.long}`
            );
            return { content: [{ type: 'text', text: addEntityOutput }] };

          // Asset Type Management
          case 'list_asset_types':
            const assetTypesOutput = await this.executeVQLCommand('vql -at');
            return { content: [{ type: 'text', text: assetTypesOutput }] };

          case 'add_asset_type':
            const addAssetTypeOutput = await this.executeVQLCommand(
              `vql -at -add ${typedArgs.short} "${typedArgs.description}"`
            );
            return { content: [{ type: 'text', text: addAssetTypeOutput }] };

          // Asset Management
          case 'list_assets':
            const assetsOutput = await this.executeVQLCommand('vql -ar');
            return { content: [{ type: 'text', text: assetsOutput }] };

          case 'add_asset':
            const addAssetOutput = await this.executeVQLCommand(
              `vql -ar -add ${typedArgs.shortName} ${typedArgs.entity} ${typedArgs.assetType} "${typedArgs.path}"`
            );
            return { content: [{ type: 'text', text: addAssetOutput }] };

          // Review Management
          case 'store_review':
            const storeReviewOutput = await this.executeVQLCommand(
              `vql -st ${typedArgs.asset} ${typedArgs.principle} "${typedArgs.review}"`
            );
            return { content: [{ type: 'text', text: storeReviewOutput }] };

          case 'get_all_reviews':
            if (!typedArgs.asset) {
              throw new McpError(ErrorCode.InvalidParams, 'Asset parameter is required');
            }
            const storage = await this.readVQLStorage();
            const asset = storage.asset_references?.[typedArgs.asset];
            if (!asset) {
              return { content: [{ type: 'text', text: `Asset '${typedArgs.asset}' not found` }] };
            }
            const reviews = asset.principle_reviews || {};
            const formattedReviews = Object.entries(reviews)
              .map(([principle, review]: [string, any]) => 
                `Principle ${principle}: ${review.review}\nCompliance: ${review.compliance || 'Not set'}\n`
              )
              .join('\n');
            return { content: [{ type: 'text', text: formattedReviews || 'No reviews found' }] };

          case 'get_review':
            if (!typedArgs.asset || !typedArgs.principle) {
              throw new McpError(ErrorCode.InvalidParams, 'Asset and principle parameters are required');
            }
            const storageForReview = await this.readVQLStorage();
            const assetForReview = storageForReview.asset_references?.[typedArgs.asset];
            if (!assetForReview) {
              return { content: [{ type: 'text', text: `Asset '${typedArgs.asset}' not found` }] };
            }
            const review = assetForReview.principle_reviews?.[typedArgs.principle];
            if (!review) {
              return { content: [{ type: 'text', text: `No review found for asset '${typedArgs.asset}' and principle '${typedArgs.principle}'` }] };
            }
            return { content: [{ type: 'text', text: `${review.review}\nCompliance: ${review.compliance || 'Not set'}` }] };

          case 'get_multiple_reviews':
            if (!typedArgs.asset || !typedArgs.principles) {
              throw new McpError(ErrorCode.InvalidParams, 'Asset and principles parameters are required');
            }
            const storageForMultiple = await this.readVQLStorage();
            const assetForMultiple = storageForMultiple.asset_references?.[typedArgs.asset];
            if (!assetForMultiple) {
              return { content: [{ type: 'text', text: `Asset '${typedArgs.asset}' not found` }] };
            }
            const multipleReviews = (typedArgs.principles || [])
              .map((principle: string) => {
                const review = assetForMultiple.principle_reviews?.[principle];
                if (review) {
                  return `Principle ${principle}: ${review.review}\nCompliance: ${review.compliance || 'Not set'}`;
                }
                return null;
              })
              .filter(Boolean)
              .join('\n\n');
            return { content: [{ type: 'text', text: multipleReviews || 'No reviews found for specified principles' }] };

          // Compliance Management
          case 'set_exemplar':
            const setExemplarOutput = await this.executeVQLCommand(
              `vql -se ${typedArgs.asset} ${typedArgs.isExemplar ? 't' : 'f'}`
            );
            return { content: [{ type: 'text', text: setExemplarOutput }] };

          case 'set_compliance':
            const setComplianceOutput = await this.executeVQLCommand(
              `vql -sc ${typedArgs.asset} ${typedArgs.principle} ${typedArgs.level}`
            );
            return { content: [{ type: 'text', text: setComplianceOutput }] };

          // AI Workflow Commands - These return guidance for the AI to execute
          case 'review_asset_all_principles':
            return {
              content: [{
                type: 'text',
                text: `To review asset '${typedArgs.asset}' against all principles:
1. Run 'vql -pr' to get all principles
2. Run 'vql -ar' to get asset details including file path
3. Read the asset file content
4. For each principle:
   - Analyze the code against the principle criteria
   - Generate a detailed review with compliance rating
   - Store using 'vql -st ${typedArgs.asset} [principle] "[review with rating]"'
   - Include phrases like "HIGH compliance", "MEDIUM compliance", or "LOW compliance" in the review`
              }]
            };

          case 'review_asset_principles':
            return {
              content: [{
                type: 'text',
                text: `To review asset '${typedArgs.asset}' against principles ${(typedArgs.principles || []).join(', ')}:
1. Get principle definitions for: ${(typedArgs.principles || []).join(', ')}
2. Run 'vql -ar' to get asset details including file path
3. Read the asset file content
4. For each specified principle:
   - Analyze the code against the principle criteria
   - Generate a detailed review with compliance rating
   - Store using 'vql -st ${typedArgs.asset} [principle] "[review with rating]"'
   - Include phrases like "HIGH compliance", "MEDIUM compliance", or "LOW compliance" in the review`
              }]
            };

          case 'refactor_asset_all_principles':
            return {
              content: [{
                type: 'text',
                text: `To refactor asset '${typedArgs.asset}' based on all principles:
1. Run 'vql -pr' to get all principles
2. Run 'vql -ar' to get asset details including file path
3. Read the asset file content
4. Analyze against all principles to identify improvements
5. Apply refactoring changes to the file
6. MANDATORY: Review the refactored asset against all principles:
   - For each principle, analyze the refactored code
   - Generate a new review reflecting the post-refactoring state
   - Store using 'vql -st ${typedArgs.asset} [principle] "[updated review with new rating]"'
   - Include phrases like "After refactoring: HIGH compliance" in the review
7. The refactoring is NOT complete until all reviews are updated

Note: For commands using "-pr" (all principles), this is equivalent to specifying all available principles.`
              }]
            };

          case 'refactor_asset_principles':
            return {
              content: [{
                type: 'text',
                text: `To refactor asset '${typedArgs.asset}' based on principles ${(typedArgs.principles || []).join(', ')}:
1. Get principle definitions for: ${(typedArgs.principles || []).join(', ')}
2. Run 'vql -ar' to get asset details including file path
3. Read the asset file content
4. Analyze against specified principles to identify improvements
5. Apply refactoring changes to the file
6. MANDATORY: Review the refactored asset against the specified principles:
   - For each principle in [${(typedArgs.principles || []).join(', ')}], analyze the refactored code
   - Generate a new review reflecting the post-refactoring state
   - Store using 'vql -st ${typedArgs.asset} [principle] "[updated review with new rating]"'
   - Include phrases like "After refactoring: HIGH compliance" in the review
7. The refactoring is NOT complete until all specified principle reviews are updated`
              }]
            };

          case 'refactor_asset_using_reference':
            return {
              content: [{
                type: 'text',
                text: `To refactor asset '${typedArgs.asset}' using '${typedArgs.referenceAsset}' as reference:
1. Run 'vql -ar' to get details for both assets
2. Read both asset files
3. Get reviews for reference asset to understand exemplary patterns
4. Identify patterns in reference asset that can improve target asset
5. Apply similar patterns to target asset
6. MANDATORY: Review the refactored asset against all affected principles:
   - Identify which principles were addressed by the refactoring
   - For each affected principle, analyze the refactored code
   - Store using 'vql -st ${typedArgs.asset} [principle] "[updated review with new rating]"'
   - Include phrases like "After refactoring to match ${typedArgs.referenceAsset}: HIGH compliance"
7. The refactoring is NOT complete until reviews are updated`
              }]
            };

          case 'refactor_asset_principles_with_references':
            return {
              content: [{
                type: 'text',
                text: `To refactor asset '${typedArgs.asset}' based on principles ${(typedArgs.principles || []).join(', ')} using references ${(typedArgs.referenceAssets || []).join(', ')}:
1. Get principle definitions for: ${(typedArgs.principles || []).join(', ')}
2. Run 'vql -ar' to get details for target and reference assets
3. Read all asset files (target: ${typedArgs.asset}, references: ${(typedArgs.referenceAssets || []).join(', ')})
4. For each reference asset, identify patterns that exemplify the specified principles
5. Analyze target asset against specified principles
6. Apply patterns from reference assets to improve compliance with specified principles
7. MANDATORY: Review the refactored asset against the specified principles:
   - For each principle in [${(typedArgs.principles || []).join(', ')}], analyze the refactored code
   - Generate new reviews reflecting post-refactoring state
   - Store using 'vql -st ${typedArgs.asset} [principle] "[updated review with new rating]"'
   - Include phrases like "After refactoring using patterns from ${(typedArgs.referenceAssets || []).join(', ')}: HIGH compliance"
8. The refactoring is NOT complete until all specified principle reviews are updated`
              }]
            };

          case 'refactor_asset_all_principles_with_references':
            return {
              content: [{
                type: 'text',
                text: `To refactor asset '${typedArgs.asset}' based on all principles using references ${(typedArgs.referenceAssets || []).join(', ')}:
1. Run 'vql -pr' to get all principles
2. Run 'vql -ar' to get details for target and reference assets
3. Read all asset files (target: ${typedArgs.asset}, references: ${(typedArgs.referenceAssets || []).join(', ')})
4. For each reference asset, identify patterns that exemplify various principles
5. Analyze target asset against all principles
6. Apply patterns from reference assets to improve overall compliance
7. MANDATORY: Review the refactored asset against all principles:
   - For each principle, analyze the refactored code
   - Generate new reviews reflecting post-refactoring state
   - Store using 'vql -st ${typedArgs.asset} [principle] "[updated review with new rating]"'
   - Include phrases like "After refactoring using patterns from ${(typedArgs.referenceAssets || []).join(', ')}: HIGH compliance"
8. The refactoring is NOT complete until all reviews are updated`
              }]
            };


          // Batch Operations
          case 'review_all_assets_all_principles':
            return {
              content: [{
                type: 'text',
                text: `To review all assets against all principles:
1. Run 'vql -ar' to get all assets
2. Run 'vql -pr' to get all principles
3. For each asset and principle combination:
   - Read the asset file
   - Analyze against the principle
   - Generate and store review with compliance rating`
              }]
            };

          case 'review_all_assets_principles':
            return {
              content: [{
                type: 'text',
                text: `To review all assets against principles ${(typedArgs.principles || []).join(', ')}:
1. Run 'vql -ar' to get all assets
2. Get principle definitions for: ${(typedArgs.principles || []).join(', ')}
3. For each asset and specified principle combination:
   - Read the asset file
   - Analyze against the principle
   - Generate and store review with compliance rating`
              }]
            };

          case 'refactor_all_assets_all_principles':
            return {
              content: [{
                type: 'text',
                text: `To refactor all assets based on all principles:
1. Run 'vql -ar' to get all assets
2. Run 'vql -pr' to get all principles
3. For each asset:
   - Read the file
   - Analyze against all principles to identify improvements
   - Apply refactoring changes to improve compliance
4. MANDATORY: After refactoring each asset, review it:
   - For each principle, analyze the refactored code
   - Generate new reviews reflecting post-refactoring state
   - Store using 'vql -st [asset] [principle] "[updated review]"'
   - Include phrases like "After refactoring: HIGH compliance"
5. The refactoring process is NOT complete until all reviews are updated

Note: This batch operation uses "-pr" (all principles) syntax for comprehensive refactoring.`
              }]
            };

          case 'refactor_all_assets_principles':
            return {
              content: [{
                type: 'text',
                text: `To refactor all assets based on principles ${(typedArgs.principles || []).join(', ')}:
1. Run 'vql -ar' to get all assets
2. Get principle definitions for: ${(typedArgs.principles || []).join(', ')}
3. For each asset:
   - Read the file
   - Analyze against specified principles to identify improvements
   - Apply refactoring changes to improve compliance
4. MANDATORY: After refactoring each asset, review it:
   - For each principle in [${(typedArgs.principles || []).join(', ')}], analyze the refactored code
   - Generate new reviews reflecting post-refactoring state
   - Store using 'vql -st [asset] [principle] "[updated review]"'
   - Include phrases like "After refactoring: HIGH compliance"
5. The refactoring process is NOT complete until all specified principle reviews are updated`
              }]
            };

          default:
            throw new McpError(
              ErrorCode.MethodNotFound,
              `Unknown tool: ${name}`
            );
        }
      } catch (error: any) {
        if (error instanceof McpError) {
          throw error;
        }
        throw new McpError(
          ErrorCode.InternalError,
          `Tool execution failed: ${error.message}`
        );
      }
    });
  }

  async run() {
    const transport = new StdioServerTransport();
    await this.server.connect(transport);
    console.error('VQL MCP server running on stdio');
  }
}

// Main execution
const server = new VQLMCPServer();
server.run().catch(console.error);