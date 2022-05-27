module.exports = {
    important: true,
    darkMode: 'class',
    content: ['./src/**/*.{js,jsx,ts,tsx}'],
    theme: {
        extend: {
            colors: {
                background: {
                    DEFAULT: '#FFFFFF',
                    dark: '#181818',
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
                    DEFAULT: "#EBEAEA",
                    dark: "#303030",
                },
                accent: {
                    DEFAULT: '#DA026E',
                    dark: '#FA007D',
                },
                code: {
                    DEFAULT: '#EBEBEB',
                    dark: '#282828',
                    text: '#CD0079',
                    textDark: '#FD419F',
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
