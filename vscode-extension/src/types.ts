export interface VQLStorage {
    version: string;
    created: string;
    last_modified: string;
    commands: Record<string, any>;
    asset_types: Record<string, AssetType>;
    entities: Record<string, Entity>;
    principles: Record<string, Principle>;
    asset_references: Record<string, AssetReference>;
}

export interface AssetType {
    short_name: string;
    description: string;
    last_modified: string;
}

export interface Entity {
    short_name: string;
    description: string;
    last_modified: string;
}

export interface Principle {
    short_name: string;
    long_name: string;
    guidance?: string;
    last_modified: string;
}

export interface AssetReference {
    short_name: string;
    entity: string;
    asset_type: string;
    path: string;
    last_modified: string;
    exemplar: boolean;
    principle_reviews: Record<string, Review>;
}

export interface Review {
    rating?: string;
    analysis?: string;
    last_modified: string;
}

export type ComplianceLevel = 'H' | 'M' | 'L' | null;