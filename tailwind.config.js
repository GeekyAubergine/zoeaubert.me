module.exports = {
    darkMode: 'class',
    content: ['./src/**/*.{js,jsx,ts,tsx}'],
    safelist: [
        'gatsby-highlight',
    ],
    theme: {
        extend: {
            colors: {
                background: {
                    DEFAULT: '#FFFFFF',
                    dark: '#0D0E10',
                },
                text: {
                    DEFAULT: '#101010',
                    dark: '#D8E1EA',
                },
                secondary: {
                    DEFAULT: '#505050',
                    dark: '#C0C0C0',
                },
                tag: {
                    DEFAULT: '#F6F8FA',
                    dark: '#181D27',
                },
                accent: {
                    DEFAULT: '#105887',
                    dark: '#53ACFF',
                },
                code: {
                    DEFAULT: '#F6F8FA',
                    dark: '#181D27',
                    text: '#105887',
                    textDark: '#53ACFF',
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
