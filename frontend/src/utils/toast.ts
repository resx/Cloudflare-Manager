// Global toast helper for easy access
export const toast = {
    success: (msg: string) => {
        const t = (window as any).__toast__
        if (t) t.success(msg)
        else console.log('[Toast]', msg)
    },
    error: (msg: string) => {
        const t = (window as any).__toast__
        if (t) t.error(msg)
        else console.error('[Toast]', msg)
    },
    warning: (msg: string) => {
        const t = (window as any).__toast__
        if (t) t.warning(msg)
        else console.warn('[Toast]', msg)
    },
    info: (msg: string) => {
        const t = (window as any).__toast__
        if (t) t.info(msg)
        else console.info('[Toast]', msg)
    },
}

// Re-export for compatibility with Naive UI style
export const message = toast
