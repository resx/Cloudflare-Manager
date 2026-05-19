/** @type {import('tailwindcss').Config} */
export default {
    content: [
        "./index.html",
        "./src/**/*.{vue,js,ts,jsx,tsx}",
    ],
    theme: {
        extend: {
            colors: {
                background: '#f1f3f9',
                foreground: '#333333',

                card: {
                    DEFAULT: '#ffffff',
                    foreground: '#333333',
                },

                primary: {
                    DEFAULT: '#1d4ed8',
                    foreground: '#ffffff',
                },

                muted: {
                    DEFAULT: '#f0f0f0',
                    foreground: '#666666',
                },

                accent: {
                    DEFAULT: '#dbeafe',
                    foreground: '#1d4ed8',
                },

                border: '#e5e7eb',

                success: {
                    DEFAULT: '#d1fae5',
                    foreground: '#065f46',
                },

                banner: {
                    from: '#FDF2F3',
                    to: '#F1F0FB',
                },

                glass: {
                    light: 'rgba(255, 255, 255, 0.6)',
                    'light-strong': 'rgba(255, 255, 255, 0.75)',
                    dark: 'rgba(30, 30, 30, 0.6)',
                    'dark-strong': 'rgba(30, 30, 30, 0.75)',
                    border: 'rgba(255, 255, 255, 0.2)',
                    'border-dark': 'rgba(255, 255, 255, 0.1)',
                },
            },
            borderRadius: {
                'island': '20px',
                'card': '14px',
                'topbar': '14px',
            },
            boxShadow: {
                'island': '0 4px 24px -2px rgba(0, 0, 0, 0.08), 0 2px 8px -2px rgba(0, 0, 0, 0.04)',
                'island-md': '0 8px 32px -4px rgba(0, 0, 0, 0.10), 0 4px 12px -2px rgba(0, 0, 0, 0.05)',
                'island-lg': '0 16px 48px -8px rgba(0, 0, 0, 0.12), 0 8px 20px -4px rgba(0, 0, 0, 0.06)',
                'island-xl': '0 24px 64px -12px rgba(0, 0, 0, 0.14), 0 12px 28px -6px rgba(0, 0, 0, 0.07)',
                'glass': '0 8px 32px rgba(31, 38, 135, 0.08)',
                'glass-lg': '0 16px 48px rgba(31, 38, 135, 0.12)',
            },
            backdropBlur: {
                'glass': '20px',
                'glass-strong': '30px',
            },
            spacing: {
                'island': '1.5rem',
            },
            animation: {
                'gradient-shift': 'gradientShift 60s ease infinite',
            },
            keyframes: {
                gradientShift: {
                    '0%, 100%': { backgroundPosition: '0% 50%' },
                    '25%': { backgroundPosition: '100% 50%' },
                    '50%': { backgroundPosition: '100% 100%' },
                    '75%': { backgroundPosition: '0% 100%' },
                },
            },
        },
    },
    plugins: [],
}
