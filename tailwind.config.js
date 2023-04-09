module.exports = {
    darkMode: 'media',
    content: ['./src/**/**/*.{njk,md}'],
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
                background: '#202022',
                headings: '#E4E3E7',
                text: '#F4F3F7',
                secondary: '#ABA9B0',
                tag: '#181D27',
                accent: '#FEB847', //'#89BA6A', //'#FEB847', //'#DEB9FF', //'#BB9EE0',//'#FEB847', ///#FEB847",//"#F2DE7C",//"#FEB847",//D9BBFF//00D5C6
                code: {
                    DEFAULT: '#181D27',
                    text: '#0061DF8EA45C',
                },
                border: '#514F55',
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
