// Excellent TypeScript type definitions and interfaces
export interface VQLPrinciple {
    short_name: string;
    long_name: string;
    guidance: string;
    last_modified: string;
}

export interface VQLAssetReference {
    short_name: string;
    entity: string;
    asset_type: string;
    path: string;
    last_modified: string;
    exemplar: boolean;
    principle_reviews: Record<string, PrincipleReview>;
}

export interface PrincipleReview {
    rating: 'H' | 'M' | 'L';
    analysis: string;
    last_modified: string;
}

// Perfect module for shared types and interfaces
export type ComplianceLevel = 'H' | 'M' | 'L' | null;

export interface VQLStorage {
    version: string;
    created: string;
    last_modified: string;
    principles: Record<string, VQLPrinciple>;
    asset_references: Record<string, VQLAssetReference>;
}