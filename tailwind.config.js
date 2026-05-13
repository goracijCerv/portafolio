/** @type {import('tailwindcss').Config} */
module.exports = {
    content: [
        "./src/**/*.rs",
        "./index.html",
    ],
    darkMode: 'class',
    theme: {
        extend: {
            colors: {
                void: 'var(--void)',
                'void-light': 'var(--void-light)',
                'gray-deep': 'var(--gray-deep)',
                'gray-mid': 'var(--gray-mid)',
                'gray-light': 'var(--gray-light)',
                'text-main': 'var(--text-main)',
                'text-heading': 'var(--text-heading)',

                'aurora-purple': 'var(--aurora-purple)',
                'aurora-purple-light': 'var(--aurora-purple-light)',
                'aurora-purple-glow': 'var(--aurora-purple-glow)',

                'aurora-blue': 'var(--aurora-blue)',
                'aurora-blue-light': 'var(--aurora-blue-light)',
                'aurora-blue-glow': 'var(--aurora-blue-glow)',

                'aurora-green': 'var(--aurora-green)',
                'aurora-green-light': 'var(--aurora-green-light)',
                'aurora-green-glow': 'var(--aurora-green-glow)',
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
                'load-fill': 'load-fill 2s ease-in-out forwards'
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
                'glitch1': {
                    '0%': { 'clip-path': 'inset(20px 0 85% 0)' },
                    '25%': { 'clip-path': 'inset(60px 0 30% 0)' },
                    '50%': { 'clip-path': 'inset(30px 0 15% 0)' },
                    '75%': { 'clip-path': 'inset(80px 0 90% 0)' },
                    '100%': { 'clip-path': 'inset(40px 0 50% 0)' },
                },
                'glitch2': {
                    '0%': { 'clip-path': 'inset(15px 0 15% 0)' },
                    '25%': { 'clip-path': 'inset(80px 0 80% 0)' },
                    '50%': { 'clip-path': 'inset(50px 0 40% 0)' },
                    '75%': { 'clip-path': 'inset(10px 0 10% 0)' },
                    '100%': { 'clip-path': 'inset(70px 0 70% 0)' },
                },
                'blink': {
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
                'load-fill': {                                    // <-- AÑADE ESTO
                    to: { left: '0' },
                },
            },
        },
    },
    plugins: [],
}