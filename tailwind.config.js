module.exports = {
    darkMode: 'class',
    content: ['./src/**/*.{js,jsx,ts,tsx}'],
    safelist: [
        'gatsby-highlight',
        'photogrid',
        'gatsby-resp-image-image',
        'statuslol_container',
        'statuslol',
        'statuslol_emoji_container',
        'statuslol_content',
        'statuslol_time',
        'statuslol_emoji',
    ],
    theme: {
        extend: {
            colors: {
                background: {
                    DEFAULT: '#FFFFFF',
                    dark: '#121216',
                },
                headings: {
                    DEFAULT: '#252527',
                    dark: '#E4E3E7',
                },
                text: {
                    DEFAULT: '#252527',
                    dark: '#F4F3F7',
                },
                secondary: {
                    DEFAULT: '#505050',
                    dark: '#A3A1A8',
                },
                tag: {
                    DEFAULT: '#F6FAF9',
                    dark: '#181D27',
                },
                accent: {
                    DEFAULT: '#00645C', //"#05614F",
                    dark: '#00D5C6', ///#FEB847",//"#F2DE7C",//"#FEB847",//D9BBFF
                },
                code: {
                    DEFAULT: '#F6FAF9',
                    dark: '#181D27',
                    text: '#00645C',
                    textDark: '#1DF8EA',
                },
                border: {
                    DEFAULT: '#505050',
                    dark: '#A3A1A8',
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
