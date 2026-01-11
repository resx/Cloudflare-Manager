// History logging utility
export interface HistoryItem {
    id: string
    type: string
    action: string
    description: string
    status: 'success' | 'error'
    timestamp: string
    user?: string
}

const HISTORY_KEY = 'cf_operation_history'
const MAX_HISTORY_ITEMS = 100

export const historyLogger = {
    // Add a history item
    log(item: Omit<HistoryItem, 'id' | 'timestamp'>): void {
        try {
            const history = this.getAll()
            const newItem: HistoryItem = {
                ...item,
                id: Date.now().toString() + Math.random().toString(36).substr(2, 9),
                timestamp: new Date().toISOString()
            }

            history.unshift(newItem)

            // Keep only the last MAX_HISTORY_ITEMS
            if (history.length > MAX_HISTORY_ITEMS) {
                history.splice(MAX_HISTORY_ITEMS)
            }

            localStorage.setItem(HISTORY_KEY, JSON.stringify(history))
        } catch (error) {
            console.error('Failed to log history:', error)
        }
    },

    // Get all history items
    getAll(): HistoryItem[] {
        try {
            const data = localStorage.getItem(HISTORY_KEY)
            return data ? JSON.parse(data) : []
        } catch (error) {
            console.error('Failed to get history:', error)
            return []
        }
    },

    // Clear all history
    clear(): void {
        try {
            localStorage.removeItem(HISTORY_KEY)
        } catch (error) {
            console.error('Failed to clear history:', error)
        }
    },

    // Get history by type
    getByType(type: string): HistoryItem[] {
        return this.getAll().filter(item => item.type === type)
    },

    // Get history by time range
    getByTimeRange(range: '24h' | '7d' | '30d' | 'all'): HistoryItem[] {
        const history = this.getAll()

        if (range === 'all') return history

        const now = Date.now()
        const ranges = {
            '24h': 86400000,
            '7d': 604800000,
            '30d': 2592000000
        }

        const rangeMs = ranges[range]
        return history.filter(item => {
            const itemTime = new Date(item.timestamp).getTime()
            return now - itemTime <= rangeMs
        })
    }
}

// Convenience functions for common operations
export const logHistory = {
    dns(action: string, description: string, status: 'success' | 'error' = 'success') {
        historyLogger.log({
            type: 'dns',
            action,
            description,
            status,
            user: '当前账户'
        })
    },

    firewall(action: string, description: string, status: 'success' | 'error' = 'success') {
        historyLogger.log({
            type: 'firewall',
            action,
            description,
            status,
            user: '当前账户'
        })
    },

    ssl(action: string, description: string, status: 'success' | 'error' = 'success') {
        historyLogger.log({
            type: 'ssl',
            action,
            description,
            status,
            user: '当前账户'
        })
    },

    cache(action: string, description: string, status: 'success' | 'error' = 'success') {
        historyLogger.log({
            type: 'cache',
            action,
            description,
            status,
            user: '当前账户'
        })
    },

    worker(action: string, description: string, status: 'success' | 'error' = 'success') {
        historyLogger.log({
            type: 'worker',
            action,
            description,
            status,
            user: '当前账户'
        })
    }
}
