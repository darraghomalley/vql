import * as fs from 'fs';
import * as path from 'path';

// Medium error handling - could be better
export class StorageReader {
    private data: any;
    
    constructor(filePath: string) {
        // Some error handling but could use specific error types
        try {
            const content = fs.readFileSync(filePath, 'utf8');
            this.data = JSON.parse(content);
        } catch (err) {
            console.log('Failed to read storage');
            this.data = null;
        }
    }
    
    getData(): any {
        return this.data;
    }
    
    // Single responsibility but could be more modular
    saveData(filePath: string): void {
        fs.writeFileSync(filePath, JSON.stringify(this.data));
    }
}