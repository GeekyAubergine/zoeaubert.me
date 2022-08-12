module.exports = {
    darkMode: 'class',
    content: ['./src/**/*.{js,jsx,ts,tsx}'],
    safelist: ['gatsby-highlight'],
    theme: {
        extend: {
            colors: {
                background: {
                    DEFAULT: '#FFFFFF',
                    dark: '#101012',
                },
                headings: {
                    DEFAULT: '#252527',
                    dark: '#E4E3E7',
                },
                text: {
                    DEFAULT: '#252527',
                    dark: '#E4E3E7',
                },
                secondary: {
                    DEFAULT: '#505050',
                    dark: '#A3A1A8',
                },
                tag: {
                    DEFAULT: '#F6F8FA',
                    dark: '#181D27',
                },
                accent: {
                    DEFAULT: '#05614F',
                    dark: '#2DB69B',
                },
                code: {
                    DEFAULT: '#F6FAF9',
                    dark: '#181D27',
                    text: '#05614F',
                    textDark: '#2DB69B',
                },
                border: {
                    DEFAULT: '#888888',
                },
                middleGray: '#888888',
            },
            height: (theme) => ({
                'screen/2': '50vh',
                'screen/3': 'calc(100vh / 3)',
                'screen2/3': 'calc(200vh / 3)',
                'screen/4': 'calc(100vh / 4)',
                'screen/5': 'calc(100vh / 5)',
            }),
        },
        fontFamily: {
            sans: ['Helvetica', 'Arial', 'sans-serif'],
        },
    },
    plugins: [],
}
