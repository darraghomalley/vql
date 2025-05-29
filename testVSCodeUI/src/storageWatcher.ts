import * as fs from 'fs';
import { EventEmitter } from 'events';

// Low error handling - missing edge cases
export class StorageWatcher extends EventEmitter {
    private watcher: fs.FSWatcher | null = null;
    
    watch(filePath: string): void {
        // Missing error handling for file watching edge cases!
        this.watcher = fs.watch(filePath, (eventType, filename) => {
            this.emit('change', { eventType, filename });
        });
    }
    
    // Decent typing but could be more specific
    stop(): void {
        if (this.watcher) {
            this.watcher.close();
            this.watcher = null;
        }
    }
    
    // Well-encapsulated functionality
    isWatching(): boolean {
        return this.watcher !== null;
    }
}