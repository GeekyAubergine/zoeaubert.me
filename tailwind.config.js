module.exports = {
    darkMode: 'class',
    content: ['./templates/**/*.html'],
    safelist: [
        'gatsby-highlight',
        'photogrid',
        'gatsby-resp-image-image',
        'image-wrapper',
        'statuslol_container',
        'statuslol',
        'statuslol_emoji_container',
        'statuslol_content',
        'statuslol_time',
        'statuslol_emoji',
        'col-span-1',
        'col-span-2',
        'col-span-3',
        'col-span-4',
        'col-span-5',
        'col-span-6',
        'col-span-7',
        'col-span-8',
        'col-span-9',
    ],
    theme: {
        extend: {
            colors: {
                background: {
                    DEFAULT: '#FFFFFF',
                    dark: '#181818',
                },
                headings: {
                    DEFAULT: '#080808',
                    dark: '#EDEDED',
                },
                text: {
                    DEFAULT: '#080808',
                    dark: '#EDEDED',
                },
                secondary: {
                    DEFAULT: '#5A585F',
                    dark: '#ABA9B0',
                },
                accent: {
                    DEFAULT: '#842A87', //'#FEB847',//#F58123', //'#89BA6A', //'#FEB847', //'#DEB9FF', //'#BB9EE0',//'#FEB847', ///#FEB847",//"#F2DE7C",//"#FEB847",//D9BBFF//00D5C6
                    dark: '#ED95E6',
                },
                code: {
                    DEFAULT: '#D4D4D4',
                    dark: '#303030',
                },
                border: {
                    DEFAULT: '#D8D8D8',
                    dark: '#404040',
                },
                middleGray: {
                    DEFAULT: '#888888',
                    dark: '#181818',
                },
            },
            gridTemplateColumns: {
                // Simple 8 row grid
                8: 'repeat(8, minmax(0, 1fr))',
                24: 'repeat(24, minmax(0, 1fr))',
                36: 'repeat(36, minmax(0, 1fr))',
                45: 'repeat(45, minmax(0, 1fr))',
            },
        },
        fontFamily: {
            sans: ['Helvetica', 'Arial', 'sans-serif'],
        },
    },
    plugins: [require('@tailwindcss/nesting')(require('postcss-nesting'))],
}
