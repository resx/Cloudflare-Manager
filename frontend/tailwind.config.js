/** @type {import('tailwindcss').Config} */
export default {
    content: [
        "./index.html",
        "./src/**/*.{vue,js,ts,jsx,tsx}",
    ],
    theme: {
        extend: {
            colors: {
                // True Island Theme Colors (from GitLab design)
                background: '#f1f3f9',        // Unified background for entire page
                foreground: '#333333',         // Main text

                card: {
                    DEFAULT: '#ffffff',          // Island (white container)
                    foreground: '#333333',
                },

                primary: {
                    DEFAULT: '#1d4ed8',          // Active nav text blue
                    foreground: '#ffffff',
                },

                muted: {
                    DEFAULT: '#f0f0f0',         // Subtle gray
                    foreground: '#666666',       // Secondary text
                },

                accent: {
                    DEFAULT: '#dbeafe',          // Active nav background (light blue)
                    foreground: '#1d4ed8',
                },

                border: '#e5e7eb',             // Very light border (gray-200)

                // Status colors
                success: {
                    DEFAULT: '#d1fae5',
                    foreground: '#065f46',
                },

                // Banner gradient colors
                banner: {
                    from: '#FDF2F3',             // Light pink
                    to: '#F1F0FB',               // Light purple
                },
            },
            borderRadius: {
                'island': '16px',              // Main island container
                'card': '8px',                 // Inner cards
            },
            boxShadow: {
                'island': '0 1px 3px 0 rgba(0, 0, 0, 0.06), 0 1px 2px 0 rgba(0, 0, 0, 0.04)',
                'island-md': '0 4px 6px -1px rgba(0, 0, 0, 0.08), 0 2px 4px -1px rgba(0, 0, 0, 0.04)',
                'island-lg': '0 10px 15px -3px rgba(0, 0, 0, 0.08), 0 4px 6px -2px rgba(0, 0, 0, 0.03)',
                'island-xl': '0 20px 25px -5px rgba(0, 0, 0, 0.08), 0 10px 10px -5px rgba(0, 0, 0, 0.02)',
            },
            spacing: {
                'island': '1.5rem',  // Standard island padding
            },
        },
    },
    plugins: [],
}
