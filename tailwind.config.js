module.exports = {
    darkMode: 'class',
    content: ['./src/**/*.{js,jsx,ts,tsx}'],
    theme: {
        extend: {
            colors: {
                background: {
                    DEFAULT: '#FFFFFF',
                    dark: '#1B1B1B',
                },
                text: {
                    DEFAULT: '#101010',
                    dark: '#F0F0F0',
                },
                secondary: {
                    DEFAULT: '#505050',
                    dark: '#C0C0C0',
                },
                tag: {
                    DEFAULT: '#E2E1E1',
                    dark: '#343434',
                },
                accent: {
                    DEFAULT: '#B14949',
                    dark: '#C85E5E',
                },
                code: {
                    DEFAULT: '#B14949',
                    dark: '#1D1F21',
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
