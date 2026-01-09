import { ref, type App, type Component } from 'vue'
import ToastComponent from '@/components/Toast.vue'

let toastInstance: any = null

export function setupToast(app: App) {
    const container = document.createElement('div')
    document.body.appendChild(container)

    const toastApp = app.mount(container) as any
    toastInstance = toastApp
}

export function useToast() {
    if (!toastInstance) {
        console.warn('Toast not initialized. Call setupToast(app) in main.ts')
        return {
            success: (msg: string) => alert(msg),
            error: (msg: string) => alert(msg),
            warning: (msg: string) => alert(msg),
            info: (msg: string) => alert(msg),
        }
    }

    return {
        success: (msg: string) => toastInstance.success(msg),
        error: (msg: string) => toastInstance.error(msg),
        warning: (msg: string) => toastInstance.warning(msg),
        info: (msg: string) => toastInstance.info(msg),
    }
}

// Simple message function compatible with Naive UI's useMessage
export function useMessage() {
    return useToast()
}

// Simple dialog replacement
export function useDialog() {
    return {
        warning: (options: { title: string; content: string; positiveText?: string; negativeText?: string; onPositiveClick?: () => void }) => {
            if (confirm(`${options.title}\n\n${options.content}`)) {
                options.onPositiveClick?.()
            }
        },
        error: (options: { title: string; content: string }) => {
            alert(`${options.title}\n\n${options.content}`)
        },
        info: (options: { title: string; content: string }) => {
            alert(`${options.title}\n\n${options.content}`)
        },
    }
}
