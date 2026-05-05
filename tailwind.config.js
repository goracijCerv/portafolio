/** @type {import('tailwindcss').Config} */
module.exports = {
    content: [
        "./src/**/*.rs",           // Scan Rust files
        "./index.html",
    ],
    theme: {
        extend: {
            colors: {
                void: '#08080a',
                'void-light': '#12121a',
                'gray-deep': '#1a1a2e',
                'gray-mid': '#2d2d44',
                'gray-light': '#4a4a6a',
                'aurora-purple': '#7b2cbf',
                'aurora-purple-light': '#9d4edd',
                'aurora-purple-glow': '#c77dff',
                'aurora-blue': '#3a86ff',
                'aurora-blue-light': '#4cc9f0',
                'aurora-blue-glow': '#90e0ef',
                'aurora-green': '#2ec4b6',
                'aurora-green-light': '#3bf4fb',
                'aurora-green-glow': '#72efdd',
            },
            fontFamily: {
                sans: ['Space Grotesk', 'sans-serif'],
                mono: ['JetBrains Mono', 'monospace'],
                figtree: ['Figtree', 'sans-serif'],
            },
            animation: {
                'aurora-shift': 'aurora-shift 8s ease infinite',
                'aurora-flow': 'aurora-flow 22s ease-in-out infinite',
                'aurora-bar': 'aurora-bar 3s linear infinite',
                'glitch1': 'glitch1 4s infinite linear alternate-reverse',
                'glitch2': 'glitch2 3s infinite linear alternate-reverse',
                'blink': 'blink 2s ease-in-out infinite',
                'scroll-pulse': 'scroll-pulse 2s ease-in-out infinite',
                'marquee': 'marquee 30s linear infinite',
                'spin-slow': 'spin 2s linear infinite',
                'pulse-loader': 'pulse-loader 2s ease-in-out infinite',
                'crystal-rotate': 'crystal-rotate 5s linear infinite',
            },
            keyframes: {
                'aurora-shift': {
                    '0%, 100%': { backgroundPosition: '0% 50%' },
                    '50%': { backgroundPosition: '100% 50%' },
                },
                'aurora-flow': {
                    '0%, 100%': { transform: 'translateX(0) rotate(-4deg)', opacity: '0.1' },
                    '25%': { transform: 'translateX(8%) rotate(-2deg)', opacity: '0.18' },
                    '50%': { transform: 'translateX(-4%) rotate(-6deg)', opacity: '0.13' },
                    '75%': { transform: 'translateX(5%) rotate(-3deg)', opacity: '0.22' },
                },
                'aurora-bar': {
                    '0%': { backgroundPosition: '0% 50%' },
                    '100%': { backgroundPosition: '300% 50%' },
                },
                glitch1: {
                    '0%': { clip: 'rect(20px, 9999px, 15px,0)' },
                    '25%': { clip: 'rect(60px, 9999px, 70px,0)' },
                    '50%': { clip: 'rect(30px, 9999px, 85px,0)' },
                    '75%': { clip: 'rect(80px, 9999px, 10px,0)' },
                    '100%': { clip: 'rect(40px, 9999px, 50px,0)' },
                },
                glitch2: {
                    '0%': { clip: 'rect(15px, 9999px, 85px,0)' },
                    '25%': { clip: 'rect(80px, 9999px, 20px,0)' },
                    '50%': { clip: 'rect(50px, 9999px, 60px,0)' },
                    '75%': { clip: 'rect(10px, 9999px, 90px,0)' },
                    '100%': { clip: 'rect(70px, 9999px, 30px,0)' },
                },
                blink: {
                    '50%': { opacity: '0.3' },
                },
                'scroll-pulse': {
                    '0%, 100%': { opacity: '0.4' },
                    '50%': { opacity: '1' },
                },
                marquee: {
                    '0%': { transform: 'translateX(0)' },
                    '100%': { transform: 'translateX(-50%)' },
                },
                'pulse-loader': {
                    '0%, 100%': { opacity: '0.3' },
                    '50%': { opacity: '1' },
                },
                'crystal-rotate': {
                    to: { transform: 'rotate(360deg)' },
                },
            },
        },
    },
    plugins: [],
}